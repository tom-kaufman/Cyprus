<script>
    import BottomBar from '$components/BottomBar.svelte';
</script>

<main>
    <div class="content">
        <slot></slot>
    </div>
    <div class="navbar" >
        <BottomBar />
    </div>
</main>
  
<style>
    main {
        display: flex;
        flex-direction: column;
        min-height: 100vh;
    }

    .content {
        flex: 1;
        overflow-y: auto;
    }

    .navbar {
        position: fixed;
        bottom: 0;
        width: 100%;
        z-index: 1; /* Ensure it's above the content */
    }
</style>