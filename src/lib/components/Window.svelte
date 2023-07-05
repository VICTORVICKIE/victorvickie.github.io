<script lang="ts">
    import { app_state } from '$lib';
    import { IconButton } from '$lib/components';
    import { onMount } from 'svelte';
    import Moveable from 'svelte-moveable';

    export let target: string;

    let window: HTMLDivElement;
    let size: string;
    let pos: string;
    let maximized = false;
    let restore_pos = '';

    function resize({ detail: event }): void {
        event.target.style.width = `${event.width}px`;
        event.target.style.height = `${event.height}px`;
        event.target.style.transform = event.drag.transform;
    }

    function move({ detail: event }): void {
        event.target.style.transform = event.transform;
    }

    function open(): void {
        pos = 'right-0 top-0 md:right-1/3 md:top-1/4';
        window.style.transform = restore_pos;
        size = 'h-full w-full md:h-1/2 md:w-1/3';
        restore_pos = '';
    }

    function close(): void {
        $app_state[target] = 'close';
    }

    function maximize(): void {
        maximized = true;
        restore_pos = window.style.transform;
        window.style.transform = '';
        pos = 'right-0 top-0';
        window.style.width = '';
        window.style.height = '';
        size = 'h-full w-full md:h-full md:w-full';
    }

    function restore(): void {
        maximized = false;
        pos = 'right-[5%] top-[15%] md:right-1/3 md:top-1/4';
        window.style.transform = restore_pos;
        size = 'h-4/6 w-11/12 md:h-1/2 md:w-1/3';
    }

    function minimize(): void {
        size = 'w-0 h-0';
        restore_pos = window.style.transform;

        $app_state[target] = 'minimize';
    }

    $: handle = `${target}-Handle`;
    onMount(open);
    $: if ($app_state[target] === 'restore') restore();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
    on:click={() => window.style.zIndex = "10"}
    bind:this={window}
    class="{target} absolute grid grid-rows-[3rem_1fr] overflow-hidden border bg-base-200 {size} {pos}"
>
    <div class=" grid w-full grid-cols-2 border bg-base-300">
        <div class="ml-2 {handle} flex items-center justify-start">{target}</div>
        <div class="flex items-center justify-end">
            <IconButton icon="codicon:chrome-minimize" color="text-warning" on:click={minimize} />
            {#if maximized}
                <IconButton icon="codicon:chrome-restore" color="text-success" on:click={restore} />
            {:else}
                <IconButton icon="codicon:chrome-maximize" color="text-success" on:click={maximize} />
            {/if}
            <IconButton icon="codicon:chrome-close" color="text-error" on:click={close} />
        </div>
    </div>
    <div class="overflow-auto p-4">
        <slot />
    </div>
</div>
<Moveable
    target={`.${target}`}
    individualGroupable={true}
    draggable={true}
    resizable={{ edge: true }}
    keepRatio={true}
    dragTarget={`.${handle}`}
    on:drag={move}
    on:resize={resize}
/>

<style>
    :global(.moveable-control, .moveable-line) {
        opacity: 0 !important;
    }
</style>
