<script lang="ts">
    import Modal from '../Modal/Modal.svelte';

    export let isOpen: boolean;
    export let isSqlMode: boolean;
    export let isLoading: boolean = false;
    
    export let onclose: () => void = () => {};
    export let onexport: (format: string, scope: 'all' | 'query') => void = () => {};

    let selectedFormat = 'CSV';
    let selectedScope: 'all' | 'query' = 'all';

    $: if (isOpen && isSqlMode) {
        selectedScope = 'query';
    } else if (isOpen) {
        selectedScope = 'all';
    }

    function handleExport() {
        onexport(selectedFormat, selectedScope);
    }
</script>

{#if isOpen}
    <Modal title="Exportar Dados" {onclose}>
        <div class="export-options">
            
            <div>
                <div class="section-title">Formato do Arquivo</div>
                <div class="format-grid">
                    <button 
                        class="format-btn" 
                        class:selected={selectedFormat === 'CSV'} 
                        on:click={() => selectedFormat = 'CSV'}
                    >
                        <svg class="icon-csv" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><text x="8" y="17" font-family="sans-serif" font-size="6" font-weight="bold" stroke="none" fill="currentColor">CSV</text></svg>
                        <span class="format-label">CSV</span>
                    </button>

                    <button 
                        class="format-btn" 
                        class:selected={selectedFormat === 'JSON'} 
                        on:click={() => selectedFormat = 'JSON'}
                    >
                        <svg class="icon-json" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><path d="M10 12h4"/><path d="M12 10v4"/></svg>
                        <span class="format-label">JSON</span>
                    </button>

                    <button 
                        class="format-btn" 
                        class:selected={selectedFormat === 'PARQUET'} 
                        on:click={() => selectedFormat = 'PARQUET'}
                    >
                        <svg class="icon-parquet" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round"><path d="M4 22h14a2 2 0 0 0 2-2V7.5L14.5 2H6a2 2 0 0 0-2 2v4"/><path d="M14 2v6h6"/><path d="M10 20v-6"/><path d="M14 20v-6"/><path d="M6 20v-6"/></svg>
                        <span class="format-label">Parquet</span>
                    </button>
                </div>
            </div>

            <div>
                <div class="section-title">Dados para Exportar</div>
                <div class="radio-group">
                    <label class="radio-item">
                        <input type="radio" bind:group={selectedScope} value="all" disabled={isSqlMode}>
                        <div>
                            <strong>Tabela Completa</strong>
                            <div style="font-size: 0.8rem; color: var(--color-text-muted);">Todos os dados do arquivo original.</div>
                        </div>
                    </label>

                    <label class="radio-item">
                        <input type="radio" bind:group={selectedScope} value="query" disabled={!isSqlMode}>
                        <div>
                            <strong>Resultado da Visualização Atual</strong>
                            <div style="font-size: 0.8rem; color: var(--color-text-muted);">
                                {#if isSqlMode}
                                    Apenas os dados resultantes da sua query SQL.
                                {:else}
                                    (Disponível apenas no Modo SQL)
                                {/if}
                            </div>
                        </div>
                    </label>
                </div>
            </div>

            <div class="actions">
                <button class="btn-secondary" on:click={onclose} disabled={isLoading}>Cancelar</button>
                <button class="btn-primary" on:click={handleExport} disabled={isLoading}>
                    {#if isLoading}
                        <span>Exportando...</span>
                    {:else}
                        <span>Salvar Arquivo...</span>
                    {/if}
                </button>
            </div>
        </div>
    </Modal>
{/if}

<style src="./ExportModal.css"></style>