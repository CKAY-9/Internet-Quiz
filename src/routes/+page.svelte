<script lang="ts">
	import About from "./about.svelte";
import Home from "./home.svelte";
	import Play from "./play.svelte";
    let currentView = "home";
    const changeView = (view: string) => {
        currentView = view;
    }
</script>

<main class="container">
    {#if currentView === "home"}
        <Home changeView={changeView}></Home>
    {/if}
    {#if currentView === "about"}
        <About changeView={changeView}></About>
    {/if}
    {#if currentView === "play"}
        <Play changeView={changeView}></Play>
    {/if}
</main>
