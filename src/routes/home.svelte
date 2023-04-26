<script lang="ts">
	import { onMount } from "svelte";

    export let changeView: any;
    let home = document.getElementById("home");

    onMount(async () => {
        home = document.getElementById("home");
    })
</script>

<div id="home">
    <h1>Internet Test</h1>
    <h3>Developed by Camerxn</h3>
    <nav style="margin-top: 2rem">
        <button on:click={() => {
            if (home !== null) {
                home.style.opacity = "0";
                changeView("play")
            }
        }}>Play</button>
        <button on:click={() => {
            if (home !== null) {
                home.style.opacity = "0";
                changeView("about")
            }
        }}>About</button>
    </nav>
</div>

<style>
    #home {
        text-align: center;
        animation: 0.5s fadeIn backwards;
        transition: 0.5s ease all;
    }
</style>

