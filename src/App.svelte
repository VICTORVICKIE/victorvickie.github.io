<script lang="ts">
    import { Apps, app_state } from '$lib';
    import { TaskBar, Window } from '$lib/components';
    import Windows from '$lib/windows';
    import { onMount } from 'svelte';
    function open_game() {
        if ($app_state['Game'] === 'minimize') {
            $app_state['Game'] = 'restore';
        } else {
            $app_state['Game'] = 'open';
        }
    }

    onMount(() => ($app_state['Profile'] = 'open'));
</script>

<section>
    <main>
        <div class="relative h-[calc(100dvh-4rem)] w-full bg-[url('/neo-wallpaper.png')] bg-bottom bg-no-repeat">
            {#each Apps as app}
                {#if $app_state[app] !== 'close'}
                    <Window target={app}><svelte:component this={Windows.get(app)} /></Window>
                {/if}
            {/each}
            <button on:click={open_game} class="absolute bottom-6 right-5">
                <img class="h-6 w-6" src="/ezic.png" alt="EZIC" />
            </button>
        </div>
    </main>
    <TaskBar />
</section>
