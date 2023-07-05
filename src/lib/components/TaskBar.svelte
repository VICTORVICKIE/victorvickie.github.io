<script lang="ts">
    import { Apps } from '$lib';
    import { TaskIcon } from '$lib/components';
</script>

<footer class="fixed bottom-0 z-10 h-16 w-screen">
    <!-- Taskbar -->
    <nav class="flex h-16 items-center justify-evenly border bg-gradient-to-r from-base-300 via-base-200 to-base-300 text-base-content">
        <!-- Taskbar Icons -->
        {#each Apps.slice(0, 5) as app}
            <TaskIcon {app} />
        {/each}
    </nav>
</footer>
