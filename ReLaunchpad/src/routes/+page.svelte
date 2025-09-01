<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
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
  let loading = true;
  let selectedIndex = 0;
  let searchInput: HTMLInputElement;
  let viewMode: "grid" | "list" = "grid";
  let showFrequent = false;
  let iconCache = new Map<string, string | null>();
  let loadingIcons = new Set<string>();

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

  async function loadApps() {
    try {
      loading = true;
      const [appsResult, frequentResult] = await Promise.all([
        invoke("list_apps") as Promise<AppInfo[]>,
        invoke("get_frequent_apps").catch(() => []) as Promise<AppInfo[]>,
      ]);

      apps = appsResult || [];
      apps.sort((a, b) => a.name.localeCompare(b.name));

      frequentApps = (frequentResult || []) as AppInfo[];
      filteredApps = apps;
    } catch (error) {
      console.error("Failed to load apps:", error);
      apps = [];
      frequentApps = [];
      filteredApps = [];
    } finally {
      loading = false;
    }
  }

  async function launch(bundleId: string) {
    if (!bundleId) return;
    
    try {
      await invoke("open_app", { bundleId });
      // Track app usage for frequent apps
      await invoke("track_app_usage", { bundleId }).catch(console.warn);

      // Hide window after launching
      await hideWindow();
    } catch (error) {
      console.error("Failed to launch app:", error);
    }
  }

  async function hideWindow() {
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      const window = getCurrentWindow();
      await window.hide();
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
          selectedIndex = 0;
        } else {
          // Hide window on second Escape
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
    // Use requestAnimationFrame for smoother scrolling
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
    // Handle large files efficiently by processing in chunks
    const CHUNK_SIZE = 8192; // 8KB chunks
    let binary = '';
    
    for (let i = 0; i < uint8Array.length; i += CHUNK_SIZE) {
      const chunk = uint8Array.slice(i, i + CHUNK_SIZE);
      binary += String.fromCharCode.apply(null, Array.from(chunk));
    }
    
    return btoa(binary);
  }

  async function loadImage(iconPath: string): Promise<string | null> {
    if (!iconPath) return null;
    
    // Check cache first
    if (iconCache.has(iconPath)) {
      return iconCache.get(iconPath)!;
    }

    // Prevent duplicate loads
    if (loadingIcons.has(iconPath)) {
      // Wait for the ongoing load
      while (loadingIcons.has(iconPath)) {
        await new Promise(resolve => setTimeout(resolve, 10));
      }
      return iconCache.get(iconPath) || null;
    }

    loadingIcons.add(iconPath);

    try {
      // Method 1: Try using a Rust command to get the icon as base64 PNG
      try {
        const base64Png = await invoke("get_app_icon_base64", { iconPath }) as string;
        if (base64Png && base64Png.length > 0) {
          const dataUrl = `data:image/png;base64,${base64Png}`;
          iconCache.set(iconPath, dataUrl);
          console.log(`Loaded icon via Rust command: ${iconPath}`);
          return dataUrl;
        }
      } catch (rustError) {
        console.log("Rust icon command failed:", rustError);
      }

      // Method 2: Try using convertFileSrc but with better error handling
      try {
        const { convertFileSrc } = await import("@tauri-apps/api/core");
        const assetUrl = convertFileSrc(iconPath);
        console.log("Trying asset URL:", assetUrl);
        
        // Create an Image object to test if it loads
        const testImg = new Image();
        const imageLoaded = await new Promise<boolean>((resolve) => {
          testImg.onload = () => resolve(true);
          testImg.onerror = () => resolve(false);
          testImg.src = assetUrl;
          
          // Timeout after 2 seconds
          setTimeout(() => resolve(false), 2000);
        });
        
        if (imageLoaded) {
          iconCache.set(iconPath, assetUrl);
          console.log(`Asset URL works: ${iconPath}`);
          return assetUrl;
        } else {
          console.log(`Asset URL failed to load: ${iconPath}`);
        }
      } catch (convertError) {
        console.log("convertFileSrc failed:", convertError);
      }

      // Method 3: Try to read file and convert to base64 (fallback)
      try {
        let binaryContents: Uint8Array;
        
        // Try different read methods
        try {
          const { readFile } = await import("@tauri-apps/plugin-fs");
          binaryContents = await readFile(iconPath);
        } catch {
          // Try using invoke
          binaryContents = await invoke("read_icon_file", { path: iconPath }) as Uint8Array;
        }
        
        console.log(`Read file directly: ${iconPath}, size: ${binaryContents.length} bytes`);
        
        // Convert to base64
        const base64String = uint8ArrayToBase64(binaryContents);
        
        // Try as PNG MIME type (more compatible than ICNS)
        const dataUrl = `data:image/png;base64,${base64String}`;
        
        // Test if this base64 image actually works
        const testImg = new Image();
        const base64Works = await new Promise<boolean>((resolve) => {
          testImg.onload = () => resolve(true);
          testImg.onerror = () => resolve(false);
          testImg.src = dataUrl;
          setTimeout(() => resolve(false), 1000);
        });
        
        if (base64Works) {
          iconCache.set(iconPath, dataUrl);
          console.log(`Base64 conversion works: ${iconPath}`);
          return dataUrl;
        } else {
          console.log(`Base64 image failed to load: ${iconPath}`);
        }
        
      } catch (readError) {
        console.log("File read failed:", readError);
      }

      // If all methods fail, return null
      console.log(`All methods failed for: ${iconPath}`);
      iconCache.set(iconPath, null);
      return null;
      
    } catch (error) {
      console.error("Failed to load image:", iconPath, error);
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
    
    // Focus search input on mount
    setTimeout(focusSearchInput, 100);

    // Store cleanup functions
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
        {:else if !loading}
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
                    // Remove from cache on error
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