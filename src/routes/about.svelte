<script lang="ts">
    export let changeView: any;
</script>

<div id="about">
    <h1>About</h1>
    <p>
        Test your knowledge about how the internet works.
        <br>
        Written in Rust (Tauri), and Typescript (Svelte)
    </p>
    <button style="margin-top: 2rem" on:click={() => {
        const abt = document.getElementById("about");
        if (abt !== null) {
            abt.style.opacity = "0";
            changeView("home");
        }
    }}>Back</button>
</div>

<style>
    #about {
        text-align: center;
        animation: 0.5s ease fadeIn;
        transition: 0.5s ease all;
    }
</style>