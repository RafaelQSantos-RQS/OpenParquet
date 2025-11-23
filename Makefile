# Makefile para OpenParquet

# Variáveis de Comandos
NPM := npm
TAURI := npm run tauri

.PHONY: all dev build check format clean release help

# Ajuda (Padrão)
help:
	@echo "🔧 Comandos do OpenParquet:"
	@echo "  make dev      - Inicia o app em modo de desenvolvimento"
	@echo "  make build    - Compila o binário de produção"
	@echo "  make check    - Roda verificações de qualidade (Lint/Types)"
	@echo "  make release  - Gera nova versão, tag e commit (ex: make release v=patch)"

# --- Desenvolvimento ---
dev:
	$(TAURI) dev

# --- Build ---
build:
	$(TAURI) build

# --- Qualidade ---
check:
	@echo "🔍 Verificando Frontend..."
	$(NPM) run check
	@echo "🔍 Verificando Backend (Rust)..."
	cd src-tauri && cargo clippy -- -D warnings

# --- Release & Versionamento ---
release:
	@if [ -z "$(v)" ]; then echo "❌ Erro: Especifique a versão. Ex: make release v=patch"; exit 1; fi
	@echo "🚀 Gerando versão $(v)..."
	$(NPM) version $(v)
	@echo "⬆️ Enviando para o GitHub..."
	git push && git push --tags
	@echo "✅ Sucesso! A nova versão está sendo gerada pelo GitHub Actions."