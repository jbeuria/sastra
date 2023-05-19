<h1>Welcome to your library project</h1>
<p>Create your package using @sveltejs/package and preview/showcase your work with SvelteKit</p>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<script>
    import { invoke } from '@tauri-apps/api/tauri'
  
    let name = ''
    let greetMsg = ''
  
    async function greet() {
      greetMsg = await invoke('greet', { name })
    }
  </script>
  
  <div>
    <input id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
    <button on:click="{greet}">Greet</button>
    <p>{greetMsg}</p>
  </div>