<script lang="ts">
	//TODO switch to svg
	import whiteKing from "../assets/img/White_King.png";
	import whiteQueen from "../assets/img/White_Queen.png";
	import whiteRook from "../assets/img/White_Rook.png";
	import whiteBishop from "../assets/img/White_Bishop.png";
	import whiteKnight from "../assets/img/White_Knight.png";
	import whitePawn from "../assets/img/White_Pawn.png";
	import blackKing from "../assets/img/Black_King.png";
	import blackQueen from "../assets/img/Black_Queen.png";
	import blackRook from "../assets/img/Black_Rook.png";
	import blackBishop from "../assets/img/Black_Bishop.png";
	import blackKnight from "../assets/img/Black_Knight.png";
	import blackPawn from "../assets/img/Black_Pawn.png";
	import type { Board } from "vite-wasm-functions";

	// export let id: number;
	export let board: Board;
	export let row: number;
	export let col: number;

	export let width = "1rem";
	export let height = "1rem";
	export let focusable = false;
	export let selected = false;

	$: id = board.get(row, col);

	let tabindex = focusable ? 0 : -1;
	let icons = [
		{
			id: 0b10000001,
			name: "White_King",
			src: whiteKing,
		},
		{
			id: 0b01000001,
			name: "White_Queen",
			src: whiteQueen,
		},
		{
			id: 0b00100001,
			name: "White_Rook",
			src: whiteRook,
		},
		{
			id: 0b00010001,
			name: "White_Bishop",
			src: whiteBishop,
		},
		{
			id: 0b00001001,
			name: "White_Knight",
			src: whiteKnight,
		},
		{
			id: 0b00000101,
			name: "White_Pawn",
			src: whitePawn,
		},
		{
			id: 0b10000010,
			name: "Black_King",
			src: blackKing,
		},
		{
			id: 0b01000010,
			name: "Black_Queen",
			src: blackQueen,
		},
		{
			id: 0b00100010,
			name: "Black_Rook",
			src: blackRook,
		},
		{
			id: 0b00010010,
			name: "Black_Bishop",
			src: blackBishop,
		},
		{
			id: 0b00001010,
			name: "Black_Knight",
			src: blackKnight,
		},
		{
			id: 0b00000110,
			name: "Black_Pawn",
			src: blackPawn,
		},
	];
	$: displayIcon = icons.find((e) => e.id === id);
</script>

{#if displayIcon}
	<img
		src={displayIcon.src}
		alt={displayIcon.name}
		style={`--width:${width};--height:${height};`}
		{tabindex}
		class={`${selected ? "selected" : ""}`}
	/>
{/if}

<style>
	img {
		--transition-curve: cubic-bezier(0.68, -0.55, 0.27, 1.55);

		pointer-events: none;

		width: var(--width);
		height: var(--height);

		transition: width 0.2s var(--transition-curve),
			height 0.2s var(--transition-curve);
	}

	img.selected {
		width: calc(var(--width) * 1.2);
		height: calc(var(--height) * 1.2);
	}
</style>
