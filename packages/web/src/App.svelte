<script lang="ts">
	import { loop_guard } from "svelte/internal";

	import _, { keys, map } from "underscore";

	import { getBoard, Position, Move, printBoard } from "vite-wasm-functions";
	import Piece from "./lib/Piece.svelte";

	let board = getBoard();

	$: board = board;

	let marked: number[] = [];
	let moves: Move[] = [];
	let selected = null;

	// FIXME row and col are wierd
	const handleFieldClick = (row: number, col: number) => (_e: Event) => {
		if (marked.includes(row * 8 + col)) {
			let move = moves.find(
				(m) => m.getEnd().x === row && m.getEnd().y === col
			);
			if (move) {
				console.log(`Doing move: ${move}`);
				move.do(board);

				selected = null;
				marked = [];
				moves = [];

				board = board; // For svelte re-render
			} else {
				console.log(`No move found for ${row} ${col}`);
			}
			return;
		}

		if (!board.isTurnFor(col, row)) return;

		if (selected == row * 8 + col) {
			selected = null;
			marked = [];
			moves = [];

			return;
		}

		if (board.get(col, row) === 0) return;

		console.log(`Clicked on field ${row}, ${col}`);
		moves = board.generateMoves(new Position(row, col));
		console.log(`Found ${moves.length} moves`);
		marked = moves
			.map((m: Move) => m.getEnd())
			.map((p: Position) => p.x * 8 + p.y);

		selected = row * 8 + col;
	};
</script>

<main>
	<h2>Svelte Chess</h2>

	<div class="board noselect">
		{#each Array.from(Array(8).keys()) as col}
			<div class="column">
				{#each Array.from(Array(8).keys()) as row}
					<div
						class="field {marked.includes(col * 8 + row)
							? 'marked'
							: ''}"
						on:click={handleFieldClick(col, row)}
					>
						{#if board.get(row, col) != 0}
							<!-- id={board.get(row, col)} -->
							<Piece
								bind:board
								bind:row
								bind:col
								width="4em"
								height="4em"
								selected={selected == col * 8 + row}
							/>
						{/if}
					</div>
				{/each}
			</div>
		{/each}
	</div>

	<button on:click={() => console.log(board.toString())}>
		Print board
	</button>
</main>

<style>
	:root {
		font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
			Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;

		--board-size: 5em;

		--dark-color: #333;
		--light-color: #eee;
		--marked-color: rgb(91, 104, 223);
	}

	main {
		text-align: center;
		padding: 1em;
		margin: 0 auto;

		display: flex;

		align-items: center;
		flex-direction: column;
	}

	.board {
		width: calc(var(--board-size) * 8);
		height: calc(var(--board-size) * 8);

		display: flex;
		flex-direction: row;
	}

	.board .field {
		width: var(--board-size);
		height: var(--board-size);
		background-color: var(--light-color);

		display: flex;
		align-items: center;
		justify-content: center;

		transition: background-color 0.2s;
	}

	.board .column:nth-child(odd) .field:nth-child(even) {
		background-color: var(--dark-color);
	}

	.board .column:nth-child(even) .field:nth-child(odd) {
		background-color: var(--dark-color);
	}

	.field.marked {
		background-color: var(--marked-color) !important;
	}

	.noselect {
		-webkit-touch-callout: none; /* iOS Safari */
		-webkit-user-select: none; /* Safari */
		-khtml-user-select: none; /* Konqueror HTML */
		-moz-user-select: none; /* Old versions of Firefox */
		-ms-user-select: none; /* Internet Explorer/Edge */
		user-select: none; /* Non-prefixed version, currently
                                  supported by Chrome, Edge, Opera and Firefox */
	}
</style>
