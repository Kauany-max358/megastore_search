# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto
Este sistema foi desenvolvido para solucionar problemas de latência e eficiência na busca de produtos da MegaStore. O objetivo principal é permitir a localização instantânea de itens em catálogos de larga escala, utilizando técnicas avançadas de indexação. O sistema oferece funcionalidades de cadastro de produtos e consultas rápidas que ignoram a diferenciação entre letras maiúsculas e minúsculas.

## Tecnologias Utilizadas
* Linguagem: Rust (Edição 2021)
* Estrutura de Dados Principal: std::collections::HashMap
* Gerenciador de Pacotes e Build: Cargo
* Ferramenta de Testes: Cargo Test Framework

## Instruções de como executar o sistema de busca
1. Certifique-se de possuir o Rust instalado em seu ambiente (comando `rustc --version`).
2. Navegue até o diretório raiz do projeto via terminal.
3. Execute o comando abaixo para compilar e rodar a aplicação:
   ```bash
   cargo run

## Instruções de como executar os testes
Para validar a integridade das funcionalidades e os requisitos de busca, execute:

Abra o terminal na raiz do projeto.

Utilize o comando:

Bash
cargo test
O sistema executará tanto os testes unitários quanto os de integração presentes na estrutura do projeto.

## Exemplos de uso
O sistema opera através da indexação de chaves únicas. Abaixo, um exemplo de como a lógica de busca é aplicada no código:

Entrada de busca: "TECLADO" ou "teclado"

Processamento: O sistema converte a entrada para minúsculo e consulta o índice.

Retorno: Objeto Produto { id: 10, nome: "Teclado", marca: "Mega", categoria: "PC" }

## Arquitetura do sistema
O projeto segue uma arquitetura modular dividida em:

Módulo de Biblioteca (src/lib.rs): Contém as definições de estruturas (Produto e SistemaBusca) e a implementação dos métodos de indexação e pesquisa.

Módulo Executável (src/main.rs): Responsável pela interface de execução e demonstração do sistema em tempo de execução.

Módulo de Testes (tests/busca_tests.rs): Camada externa dedicada à validação de qualidade e segurança do código.

## Algoritmos e estruturas de dados utilizados
A implementação foca na utilização de Tabelas Hash (através da estrutura HashMap do Rust).

Algoritmo: Utiliza uma função de espalhamento (hashing) para mapear nomes de produtos a endereços de memória diretos.

Desempenho: A complexidade de tempo para busca é de O(1) em média, garantindo que o tempo de resposta não aumente proporcionalmente ao tamanho do catálogo.

## Considerações sobre desempenho e escalabilidade
O sistema foi projetado para alta escalabilidade. Ao contrário de buscas lineares (O(n)), a arquitetura de Tabela Hash permite que o sistema suporte milhões de registros com impacto mínimo na latência de resposta. O gerenciamento de memória do Rust garante que essa escala seja atingida sem vazamentos de memória ou instabilidade do servidor.

## Contribuições
Este projeto possui fins acadêmicos. Para contribuir, realize um fork do repositório, implemente as alterações em uma branch específica e submeta um Pull Request para análise.

## Licença
Este projeto é distribuído sob a Licença MIT.
