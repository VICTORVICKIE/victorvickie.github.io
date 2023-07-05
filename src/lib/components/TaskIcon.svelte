<script lang="ts">
    import { AppIcon, app_state } from '$lib';
    import { IconButton } from '$lib/components';

    export let app: string;
    function open_app() {
        if ($app_state[app] === 'minimize') {
            $app_state[app] = 'restore';
        } else {
            $app_state[app] = 'open';
        }
    }
</script>

<div class="indicator">
    <span class:hidden={$app_state[app] === 'close'} class="badge-xxs badge badge-accent indicator-start indicator-item" />
    <IconButton icon={AppIcon[app]} on:click={open_app} />
</div>

<style>
    /* based on daisyui badge-xs ratio */
    .badge-xxs {
        height: 0.5rem;
        font-size: 0.5rem;
        line-height: 0.5rem;
        padding-left: 0.208rem;
        padding-right: 0.208rem;
    }
</style>
