<script lang="ts">
    // Sem 'createEventDispatcher'
	export let currentPage: number;
	export let totalPages: number;
	export let isLoading: boolean;
	export let rowsLength: number;
	export let pageSize: number;

    // NOVO: Callbacks para navegação
    export let onprev: () => void = () => {};
    export let onnext: () => void = () => {};

	$: isFirstPage = currentPage === 0;
	$: isLastPage = rowsLength < pageSize || currentPage + 1 >= totalPages;
</script>

<div class="pagination">
    <button on:click={onprev} disabled={isFirstPage || isLoading}>
		&larr; Anterior
	</button>
	
	<span class="page-info">
        Página <strong>{currentPage + 1}</strong> de <strong>{totalPages}</strong>
    </span>
	
    <button on:click={onnext} disabled={isLoading || isLastPage}>
		Próxima &rarr;
	</button>
</div>

<style src="./Pagination.css"></style>