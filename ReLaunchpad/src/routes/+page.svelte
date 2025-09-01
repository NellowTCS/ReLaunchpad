<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  type AppInfo = {
    name: string;
    bundle_id: string;
    path: string;
    icon_path?: string;
  };

  let apps: AppInfo[] = [];
  let filteredApps: AppInfo[] = [];
  let frequentApps: AppInfo[] = [];
  let query = "";
  let loading = true;
  let selectedIndex = 0;
  let searchInput: HTMLInputElement;
  let viewMode: "grid" | "list" = "grid";
  let showFrequent = false;

  // Reactive filtering with categories
  $: {
    if (query.trim()) {
      const searchTerm = query.toLowerCase().trim();
      filteredApps = apps.filter(
        (app) =>
          app.name.toLowerCase().includes(searchTerm) ||
          app.bundle_id.toLowerCase().includes(searchTerm)
      );
      showFrequent = false;
    } else {
      if (showFrequent && frequentApps.length > 0) {
        filteredApps = frequentApps;
      } else {
        filteredApps = apps;
      }
    }
    // Reset selection when filter changes
    selectedIndex = 0;
  }

  async function loadApps() {
    try {
      loading = true;
      const [appsResult, frequentResult] = await Promise.all([
        invoke("list_apps") as Promise<AppInfo[]>,
        invoke("get_frequent_apps").catch(() => []) as Promise<AppInfo[]>, // Fallback to empty array if not implemented yet
      ]);

      apps = appsResult;
      apps.sort((a, b) => a.name.localeCompare(b.name));

      frequentApps = frequentResult as AppInfo[];
      filteredApps = apps;
    } catch (error) {
      console.error("Failed to load apps:", error);
    } finally {
      loading = false;
    }
  }

  async function launch(bundleId: string) {
    try {
      await invoke("open_app", { bundleId });
      // Track app usage for frequent apps
      await invoke("track_app_usage", { bundleId }).catch(console.warn);

      // Hide window after launching
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      await getCurrentWindow().hide();
    } catch (error) {
      console.error("Failed to launch app:", error);
    }
  }

  async function hideWindow() {
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      await getCurrentWindow().hide();
    } catch (error) {
      console.error("Failed to hide window:", error);
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (loading || filteredApps.length === 0) return;

    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, filteredApps.length - 1);
        scrollToSelected();
        break;
      case "ArrowUp":
        event.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        scrollToSelected();
        break;
      case "Enter":
        event.preventDefault();
        if (filteredApps[selectedIndex]) {
          launch(filteredApps[selectedIndex].bundle_id);
        }
        break;
      case "Escape":
        event.preventDefault();
        if (query) {
          query = "";
        } else {
          // Hide window on second Escape
          hideWindow();
        }
        selectedIndex = 0;
        break;
      case "Tab":
        event.preventDefault();
        showFrequent = !showFrequent && frequentApps.length > 0;
        break;
      case "ArrowRight":
        if (event.metaKey) {
          event.preventDefault();
          viewMode = viewMode === "grid" ? "list" : "grid";
        }
        break;
    }
  }

  function scrollToSelected() {
    // Scroll selected item into view
    setTimeout(() => {
      const selected = document.querySelector(".tile.selected");
      if (selected) {
        selected.scrollIntoView({ behavior: "smooth", block: "nearest" });
      }
    }, 0);
  }

  function selectApp(index: number) {
    selectedIndex = index;
  }

  onMount(() => {
    loadApps();
    // Focus search input on mount
    setTimeout(() => {
      if (searchInput) {
        searchInput.focus();
      }
    }, 100);
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<main class="container">
  <div class="header">
    <div class="search-container">
      <input
        bind:this={searchInput}
        placeholder="Search apps… (Tab for frequent, ⌘→ for view)"
        bind:value={query}
        class="search-input"
      />
      <div class="search-meta">
        {#if showFrequent && !query}
          <span class="frequent-indicator">📈 Frequent Apps</span>
        {/if}
        <div class="view-toggle">
          <button
            class="view-btn"
            class:active={viewMode === "grid"}
            on:click={() => (viewMode = "grid")}
          >
            ⊞
          </button>
          <button
            class="view-btn"
            class:active={viewMode === "list"}
            on:click={() => (viewMode = "list")}
          >
            ☰
          </button>
        </div>
      </div>
    </div>

    {#if loading}
      <div class="loading">
        <div class="loading-spinner"></div>
        Loading apps...
      </div>
    {/if}
  </div>

  {#if !loading}
    <div
      class="grid"
      class:list-view={viewMode === "list"}
      class:empty={filteredApps.length === 0}
    >
      {#each filteredApps as app, index}
        <button
          class="tile"
          class:selected={index === selectedIndex}
          class:list-item={viewMode === "list"}
          on:click={() => launch(app.bundle_id)}
          on:mouseenter={() => selectApp(index)}
        >
          {#if app.icon_path}
            <img
              src="asset://localhost/{app.icon_path}"
              alt={app.name}
              loading="lazy"
              class="app-icon"
              on:error={(e) => {
                (e.currentTarget as HTMLImageElement).style.display = "none";
              }}
            />
          {:else}
            <div class="icon-placeholder">
              {app.name.charAt(0).toUpperCase()}
            </div>
          {/if}
          <div class="app-info">
            <span class="app-name">{app.name}</span>
            {#if viewMode === "list"}
              <span class="bundle-id">{app.bundle_id}</span>
            {/if}
          </div>
        </button>
      {/each}

      {#if filteredApps.length === 0 && query.trim()}
        <div class="no-results">
          <div class="no-results-icon">🔍</div>
          <p>No apps found for "<strong>{query}</strong>"</p>
          <p class="suggestion">
            Try searching by app name or bundle identifier
          </p>
        </div>
      {/if}
    </div>
  {/if}
</main>
