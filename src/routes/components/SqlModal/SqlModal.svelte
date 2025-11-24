<script lang="ts">
    import Modal from '../Modal/Modal.svelte';

    export let isOpen: boolean;
    export let onclose: () => void = () => {};
    export let onrun: (query: string) => void = () => {};

    let query = "SELECT * FROM t LIMIT 10";

    function handleRun() {
        onrun(query);
    }
</script>

{#if isOpen}
    <Modal title="Executar Consulta SQL" {onclose}>
        <div class="sql-container">
            <p class="hint">
                A tabela atual está disponível como a view <code>t</code>.
            </p>
            
            <textarea 
                bind:value={query} 
                placeholder="Ex: SELECT count(*) FROM t WHERE valor > 100"
                spellcheck="false"
            ></textarea>

            <div class="actions">
                <button class="btn-secondary" on:click={onclose}>Cancelar</button>
                <button class="btn-primary" on:click={handleRun}>
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 6px;"><polygon points="5 3 19 12 5 21 5 3"/></svg>
                    Executar Query
                </button>
            </div>
        </div>
    </Modal>
{/if}

<style src="./SqlModal.css"></style>