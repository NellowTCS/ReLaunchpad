<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  type AppInfo = { name: string; bundle_id: string; path: string; icon_path?: string };

  let apps: AppInfo[] = [];
  let query = "";

  async function loadApps() {
    apps = await invoke("list_apps");
    apps.sort((a, b) => a.name.localeCompare(b.name));
  }

  function launch(bundleId: string) {
    invoke("open_app", { bundleId });
  }

  onMount(loadApps);
</script>

<main class="container">
  <input placeholder="Search apps…" bind:value={query} />

  <div class="grid">
    {#each apps.filter(a => a.name.toLowerCase().includes(query.toLowerCase())) as app}
      <button class="tile" on:click={() => launch(app.bundle_id)}>
        {#if app.icon_path}
          <img src={"file://" + app.icon_path} alt={app.name} />
        {/if}
        <span>{app.name}</span>
      </button>
    {/each}
  </div>
</main>

<style>
.container {
  width: 100vw;
  height: 100vh;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  background: rgba(240, 240, 240, 0.95);
}

input {
  width: 100%;
  padding: 0.6rem;
  font-size: 1.2rem;
  margin-bottom: 1rem;
  border-radius: 6px;
  border: 1px solid #ccc;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
  gap: 16px;
  flex-grow: 1;
  overflow-y: auto;
}

.tile {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  cursor: pointer;
  padding: 0.5rem;
}

.tile img {
  width: 64px;
  height: 64px;
}

.tile span {
  margin-top: 6px;
  text-align: center;
  font-size: 0.9rem;
  word-break: break-word;
}

@media (prefers-color-scheme: dark) {
  .container {
    background: rgba(20, 20, 20, 0.95);
    color: #f6f6f6;
  }

  input {
    background: #2f2f2f;
    color: #f6f6f6;
    border: 1px solid #555;
  }
}
</style>
