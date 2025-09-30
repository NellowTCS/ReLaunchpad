<script lang="ts">
  import { onMount, onDestroy } from "svelte";

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
  let loading = false; // Set to false since we're using mock data
  let selectedIndex = 0;
  let searchInput: HTMLInputElement;
  let viewMode: "grid" | "list" = "grid";
  let showFrequent = false;
  let iconCache = new Map<string, string | null>();
  let loadingIcons = new Set<string>();

  // Mock app data for frontend testing
  const mockApps: AppInfo[] = [
    {
      name: "Finder",
      bundle_id: "com.apple.finder",
      path: "/System/Applications/Finder.app",
      icon_path: "/System/Applications/Finder.app/Contents/Resources/AppIcon.icns"
    },
    {
      name: "Safari",
      bundle_id: "com.apple.safari",
      path: "/System/Applications/Safari.app",
      icon_path: "/System/Applications/Safari.app/Contents/Resources/AppIcon.icns"
    },
    {
      name: "Calendar",
      bundle_id: "com.apple.calendar",
      path: "/System/Applications/Calendar.app",
      icon_path: "/System/Applications/Calendar.app/Contents/Resources/AppIcon.icns"
    },
    {
      name: "Notes",
      bundle_id: "com.apple.notes",
      path: "/System/Applications/Notes.app",
      icon_path: "/System/Applications/Notes.app/Contents/Resources/AppIcon.icns"
    },
    {
      name: "Mail",
      bundle_id: "com.apple.mail",
      path: "/System/Applications/Mail.app",
      icon_path: "/System/Applications/Mail.app/Contents/Resources/AppIcon.icns"
    }
  ];

  // Mock frequent apps for testing
  const mockFrequentApps: AppInfo[] = [
    mockApps[0], // Finder
    mockApps[1], // Safari
    mockApps[3]  // Notes
  ];

  // Cleanup function for event listeners
  let cleanup: (() => void)[] = [];

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

  function loadApps() {
    // Use mock data instead of invoking system calls
    apps = mockApps;
    apps.sort((a, b) => a.name.localeCompare(b.name));
    frequentApps = mockFrequentApps;
    filteredApps = apps;
  }

  async function launch(bundleId: string) {
    if (!bundleId) return;
    
    // Simulate launching app for testing
    console.log(`Simulating launch of app with bundle ID: ${bundleId}`);
    await hideWindow();
  }

  async function hideWindow() {
    // Simulate hiding window for testing
    console.log("Simulating window hide");
  }

  function handleKeydown(event: KeyboardEvent) {
    if (filteredApps.length === 0) return;

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
          selectedIndex = 0;
        } else {
          hideWindow();
        }
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
    requestAnimationFrame(() => {
      const selected = document.querySelector(".tile.selected");
      if (selected) {
        selected.scrollIntoView({ behavior: "smooth", block: "nearest" });
      }
    });
  }

  function selectApp(index: number) {
    if (index >= 0 && index < filteredApps.length) {
      selectedIndex = index;
    }
  }

  function uint8ArrayToBase64(uint8Array: Uint8Array): string {
    const CHUNK_SIZE = 8192;
    let binary = '';
    
    for (let i = 0; i < uint8Array.length; i += CHUNK_SIZE) {
      const chunk = uint8Array.slice(i, i + CHUNK_SIZE);
      binary += String.fromCharCode.apply(null, Array.from(chunk));
    }
    
    return btoa(binary);
  }

  async function loadImage(iconPath: string): Promise<string | null> {
    if (!iconPath) return null;
    
    if (iconCache.has(iconPath)) {
      return iconCache.get(iconPath)!;
    }

    if (loadingIcons.has(iconPath)) {
      while (loadingIcons.has(iconPath)) {
        await new Promise(resolve => setTimeout(resolve, 10));
      }
      return iconCache.get(iconPath) || null;
    }

    loadingIcons.add(iconPath);

    try {
      // For mock data, return a placeholder base64 image or null
      // In a real app, you might want to include actual base64 strings for testing
      console.log(`Simulating icon load for: ${iconPath}`);
      const placeholderImage = null; // Replace with actual base64 string if needed
      iconCache.set(iconPath, placeholderImage);
      return placeholderImage;
    } catch (error) {
      console.error("Failed to load mock image:", iconPath, error);
      iconCache.set(iconPath, null);
      return null;
    } finally {
      loadingIcons.delete(iconPath);
    }
  }

  function focusSearchInput() {
    if (searchInput) {
      searchInput.focus();
      searchInput.select();
    }
  }

  onMount(() => {
    loadApps();
    setTimeout(focusSearchInput, 100);

    cleanup.push(() => {
      iconCache.clear();
      loadingIcons.clear();
    });
  });

  onDestroy(() => {
    cleanup.forEach(fn => fn());
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
        autocomplete="off"
        spellcheck="false"
      />
      <div class="search-meta">
        {#if showFrequent && !query}
          <span class="frequent-indicator">Frequent Apps ({frequentApps.length})</span>
        {:else}
          <span class="app-count">{filteredApps.length} app{filteredApps.length !== 1 ? 's' : ''}</span>
        {/if}
        <div class="view-toggle">
          <button
            class="view-btn"
            class:active={viewMode === "grid"}
            on:click={() => (viewMode = "grid")}
            title="Grid view"
          >
            ⊞
          </button>
          <button
            class="view-btn"
            class:active={viewMode === "list"}
            on:click={() => (viewMode = "list")}
            title="List view"
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
          title={`${app.name} (${app.bundle_id})`}
        >
          {#if app.icon_path}
            {#await loadImage(app.icon_path) then base64Data}
              {#if base64Data}
                <img
                  src={base64Data}
                  alt={app.name}
                  loading="lazy"
                  class="app-icon"
                  on:error={(e) => {
                    const img = e.currentTarget as HTMLImageElement;
                    img.style.display = "none";
                    if (app.icon_path) {
                      iconCache.set(app.icon_path, null);
                    }
                  }}
                />
              {:else}
                <div class="icon-placeholder">
                  {app.name.charAt(0).toUpperCase()}
                </div>
              {/if}
            {:catch error}
              <div class="icon-placeholder">
                {app.name.charAt(0).toUpperCase()}
              </div>
            {/await}
          {:else}
            <div class="icon-placeholder">
              {app.name.charAt(0).toUpperCase()}
            </div>
          {/if}
          <div class="app-info">
            <span class="app-name" title={app.name}>{app.name}</span>
            {#if viewMode === "list"}
              <span class="bundle-id" title={app.bundle_id}>{app.bundle_id}</span>
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
          <button class="clear-search-btn" on:click={() => query = ""}>
            Clear search
          </button>
        </div>
      {/if}

      {#if filteredApps.length === 0 && !query.trim() && !loading}
        <div class="no-results">
          <div class="no-results-icon">😔</div>
          <p>No apps available</p>
          <p class="suggestion">
            Try refreshing or check your system permissions
          </p>
          <button class="refresh-btn" on:click={loadApps}>
            Refresh apps
          </button>
        </div>
      {/if}
    </div>
  {/if}
</main>