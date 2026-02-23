<script lang="ts">
	interface Props {
		currentPage: number;
		totalPages: number;
		isLoading: boolean;
		rowsLength: number;
		pageSize: number;
		onprev?: () => void;
		onnext?: () => void;
	}

	let { 
		currentPage, 
		totalPages, 
		isLoading, 
		rowsLength, 
		pageSize,
		onprev = () => {},
		onnext = () => {}
	}: Props = $props();

	let isFirstPage = $derived(currentPage === 0);
	let isLastPage = $derived(rowsLength < pageSize || currentPage + 1 >= totalPages);
</script>

<div class="pagination">
	<button onclick={onprev} disabled={isFirstPage || isLoading}>
		&larr; Anterior
	</button>
	
	<span class="page-info">
		Página <strong>{currentPage + 1}</strong> de <strong>{totalPages}</strong>
	</span>
	
	<button onclick={onnext} disabled={isLoading || isLastPage}>
		Próxima &rarr;
	</button>
</div>

<style src="./Pagination.css"></style>
