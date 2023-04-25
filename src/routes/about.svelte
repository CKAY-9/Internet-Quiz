<script lang="ts">
    export let changeView: any;
</script>

<button on:click={() => {
    changeView("home")
}}>Back</button>
<h1>About</h1>