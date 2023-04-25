<script lang="ts">
    export let changeView: any;
</script>

<h1>Internet Test</h1>
<h3>Developed by Camerxn</h3>
<nav>
    <button on:click={() => changeView("play")}>Play</button>
    <button on:click={() => changeView("about")}>About</button>
</nav>

