// +layout.ts
export const ssr = false;

import { onMount } from "svelte";
import type { AppWindow } from "@tauri-apps/api/window";

onMount(async () => {
  try {
    // Runtime-only imports
    const { registerGlobalShortcut, unregisterAllGlobalShortcuts } =
      await import("@tauri-apps/plugin-global-shortcut");
    const { invoke } = await import("@tauri-apps/api/core");

    // Register global shortcut
    await registerGlobalShortcut("Alt+Space", async () => {
      try {
        await invoke("show_main_window");
      } catch (error) {
        console.error("Failed to show main window:", error);
      }
    });

    // Return cleanup function
    return async () => {
      try {
        await unregisterAllGlobalShortcuts();
      } catch (error) {
        console.error("Failed to unregister shortcuts:", error);
      }
    };
  } catch (error) {
    console.error("Failed to setup global shortcuts:", error);
  }
});
