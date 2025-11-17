<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	// Props definidas com 'export let' (padrão Svelte 4)
	export let currentPage: number;
	export let totalPages: number;
	export let isLoading: boolean;
	export let rowsLength: number;
	export let pageSize: number;

	const dispatch = createEventDispatcher();

	// Valores derivados com '$:' (padrão Svelte 4)
	$: isFirstPage = currentPage === 0;
	$: isLastPage = rowsLength < pageSize || currentPage + 1 >= totalPages;
</script>

<div class="pagination">
	<button on:click={() => dispatch('prev')} disabled={isFirstPage || isLoading}> &larr; Anterior </button>
	<span>Página {currentPage + 1} de {totalPages}</span>
	<button on:click={() => dispatch('next')} disabled={isLoading || isLastPage}>
		Próxima &rarr;
	</button>
</div>

<style>
	.pagination {
		margin: 1em 0;
		display: flex;
		justify-content: center;
		gap: 1em;
		align-items: center;
		flex-shrink: 0;
	}
</style>