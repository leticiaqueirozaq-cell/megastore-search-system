# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto

Este projeto implementa um sistema de busca otimizado para um catálogo de produtos utilizando a linguagem Rust.

O objetivo é permitir buscas rápidas em grandes volumes de dados utilizando estruturas de dados eficientes, como tabelas hash, melhorando o desempenho das consultas em catálogos de e-commerce.

## Tecnologias Utilizadas

- Rust
- Cargo
- HashMap (tabela hash)
- Git
- GitHub

## Como Executar o Sistema

1. Clone o repositório:

git clone https://github.com/leticiaqueirozaq-cell/megastore-search-system

2. Entre na pasta do projeto:

cd megastore-search-system

3. Execute o sistema:

cargo run

## Como Executar os Testes

Para executar os testes automatizados utilize o comando:

cargo test

## Exemplos de Uso

Exemplos de consultas que podem ser realizadas no sistema:

- Notebook
- Mouse
- Teclado

O sistema retorna os produtos encontrados no índice.

## Arquitetura do Sistema

O sistema foi organizado em componentes principais:

- **Product**: estrutura que representa um produto.
- **ProductIndex**: responsável pela indexação dos produtos utilizando HashMap.
- **Sistema de busca**: realiza consultas no índice.
- **Main**: executa o sistema.

## Algoritmos e Estruturas de Dados

O sistema utiliza:

- **HashMap** para indexação de produtos.
- **Vec (vetores)** para armazenar listas de produtos associados às chaves.

A utilização de HashMap permite buscas com complexidade média **O(1)**.

## Considerações de Desempenho e Escalabilidade

O uso de tabelas hash permite que o sistema realize buscas rápidas mesmo em grandes volumes de dados.

Isso torna a solução escalável para catálogos com milhares ou milhões de produtos.

## Contribuições

Contribuições são bem-vindas através de Pull Requests.

## Licença

Este projeto utiliza a licença MIT.
