<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  let cleanupFunction: (() => Promise<void>) | null = null;

  onMount(async () => {
    try {
      // Runtime-only imports
      const { register, unregisterAll } = await import("@tauri-apps/plugin-global-shortcut");

      const { invoke } = await import("@tauri-apps/api/core");

      // Register global shortcut
      await register("Ctrl+Alt+Space", async () => {
        try {
          await invoke("show_main_window");
        } catch (error) {
          console.error("Failed to show main window:", error);
        }
      });

      // Store cleanup function
      cleanupFunction = async () => {
        try {
          await unregisterAll();
        } catch (error) {
          console.error("Failed to unregister shortcuts:", error);
        }
      };
    } catch (error) {
      console.error("Failed to setup global shortcuts:", error);
    }
  });

  onDestroy(async () => {
    if (cleanupFunction) {
      await cleanupFunction();
    }
  });
</script>

<slot />

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }
</style>