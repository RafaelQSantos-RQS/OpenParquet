# Guia de Contribuição

Obrigado por considerar contribuir para o **OpenParquet**! 🎉
Projetos open source só vivem por causa de pessoas como você.

## 🛠️ Como preparar o ambiente

1.  **Pré-requisitos:**
    * [Rust](https://www.rust-lang.org/tools/install) (para o backend)
    * [Node.js LTS](https://nodejs.org/) (para o frontend)
    * Gerenciador de pacotes do seu sistema (para dependências Linux, se necessário)

2.  **Instalação:**
    ```bash
    git clone [https://github.com/rafaelqsantos-rqs/openparquet.git](https://github.com/rafaelqsantos-rqs/openparquet.git)
    cd openparquet
    npm install
    ```

3.  **Rodando em Desenvolvimento:**
    ```bash
    npm run tauri dev
    ```

## 📐 Padrões de Código

* **Commits:** Recomendamos seguir o padrão [Conventional Commits](https://www.conventionalcommits.org/).
    * `feat: adicionar filtro de colunas`
    * `fix: corrigir erro de drag and drop`
    * `docs: atualizar roadmap`
* **Linting:** Antes de enviar, verifique se não há erros:
    ```bash
    npm run check
    # Opcional (se tiver configurado)
    cargo clippy
    ```

## 🔄 Fluxo de Trabalho (Workflow)

1.  Faça um **Fork** do projeto.
2.  Crie uma **Branch** para sua feature (`git checkout -b feature/MinhaFeature`).
3.  Faça seus commits.
4.  Faça o **Push** (`git push origin feature/MinhaFeature`).
5.  Abra um **Pull Request** no repositório original descrevendo suas mudanças.

## 🐛 Encontrou um Bug?

Abra uma [Issue](https://github.com/rafaelqsantos-rqs/openparquet/issues) descrevendo:
1.  O que você esperava que acontecesse.
2.  O que realmente aconteceu.
3.  Passos para reproduzir o erro.

---
Obrigado por ajudar a fazer o OpenParquet melhor! 🚀
