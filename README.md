# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📝 Descrição do Projeto
Este sistema foi desenvolvido como parte do desafio da MegaStore para otimizar a gestão e busca de produtos em um catálogo de grande escala. O objetivo principal foi substituir métodos de busca lentos (lineares) por uma solução de alta performance utilizando a linguagem **Rust**.

A solução implementa uma **Tabela Hash**, garantindo que a busca por produtos ocorra em tempo constante, proporcionando uma experiência rápida para o usuário final, independentemente de quantos milhões de itens existam no banco de dados.

## 🛠️ Tecnologias Utilizadas
* **Linguagem:** [Rust](https://www.rust-lang.org/)
* **Estrutura de Dados:** `std::collections::HashMap` (Tabela Hash)
* **Gerenciador de Pacotes:** Cargo

## 🚀 Como Executar o Projeto
Para rodar a aplicação no seu ambiente Linux Ubuntu:

1. Certifique-se de que o Rust está instalado.
2. No terminal do projeto, execute:
   
   bash: cargo run

🧪 Como Executar os Testes
A qualidade e a confiabilidade do sistema são garantidas por testes automatizados. Para rodá-los:

Bash: cargo test

O sistema deve retornar "test result: ok" para validar a funcionalidade.

📐 Arquitetura e Algoritmos
A escolha do HashMap (Tabela Hash) foi estratégica para resolver o problema de latência.

1 . Complexidade de Busca: O(1) no caso médio.
2 . Eficiência: Ao indexar os produtos pelo nome, eliminamos a necessidade de percorrer toda a lista de itens, reduzindo drasticamente o uso de CPU durante as consultas.

📁 Estrutura do Repositório

src/lib.rs: Contém a lógica de dados e as estruturas do sistema.
src/main.rs: Ponto de entrada para execução do programa.
tests/: Pasta dedicada aos testes de integração e qualidade.
Cargo.toml: Arquivo de configuração e metadados do projeto.
---
