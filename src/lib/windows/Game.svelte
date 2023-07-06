<script lang="ts">
    import { minimax, default as wasm_init, type InitOutput } from 'minimax';
    import { onMount } from 'svelte';

    let wasm: InitOutput;
    let P1 = -1;
    let AI = 1;

    let board = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0]
    ];
    const symbols = new Map([[0, ' ']]);

    function P1_turn(x: number, y: number) {
        board[x][y] = P1;
        board = board;
        AI_turn();
    }

    function init() {
        board = [
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0]
        ];

        if (Math.random() < 0.5) {
            symbols.set(P1, 'X');
            symbols.set(AI, 'O');
        } else {
            symbols.set(AI, 'X');
            symbols.set(P1, 'O');
            AI_turn();
        }
    }

    onMount(async () => {
        wasm = await wasm_init();
        init();
    });

    function AI_turn() {
        let [x, y] = minimax(board);
        board[x][y] = AI;
        board = board;
    }
</script>

<div class="grid min-h-full w-full grid-cols-1 gap-8">
    <div class="flex items-center justify-between px-4">
        <select class="select select-xs w-28 max-w-xs">
            <option disabled selected>Difficulty</option>
            <option>Possible</option>
            <option>Impossible</option>
        </select>
        <div class="flex h-4 justify-center gap-1">
            <div class="mt-1">
                <iconify-icon width="16" height="16" icon="game-icons:cross-mark" />
            </div>
            <div>turn</div>
        </div>
    </div>
    <div class="m-auto grid h-48 w-48 grid-cols-3">
        {#each board as row, x}
            {#each row as cell, y}
                <button on:click={() => P1_turn(x, y)} disabled={Boolean(cell)} class="cell flex h-16 w-16 items-center justify-center">
                    {#if symbols?.get(cell) == 'X'}
                        <iconify-icon width="48" height="48" icon="game-icons:cross-mark" />
                    {:else if symbols?.get(cell) == 'O'}
                        <iconify-icon width="48" height="48" icon="game-icons:circle-claws" />
                    {/if}
                </button>
            {/each}
        {/each}
    </div>
    <button on:click={init} class="btn-outline btn m-auto border-none hover:bg-inherit hover:text-base-content">Restart Game</button>
</div>

<style>
    .cell:not(:nth-child(3n)) {
        border-right: 1px solid hsl(var(--bc));
    }

    .cell:not(:nth-last-child(-n + 3)) {
        border-bottom: 1px solid hsl(var(--bc));
    }
</style>
