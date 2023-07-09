<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    export let icon: string;
    export let aria_label: string;
    export let color = '';
    export let hover = true;
    const dispatch = createEventDispatcher();

    function handle_click(): void {
        dispatch('click');
    }
</script>

<button
    on:click={handle_click}
    class="flex h-12 w-12 items-center justify-center border
    {hover ? 'hover:scale-105 hover:shadow-[0.15rem_0.15rem_0px] hover:shadow-base-content' : ''} {color}"
    aria-label={aria_label}
>
    <iconify-icon width="26" height="26" {icon} />
</button>
