# Torto Video Aula Rust

**Referências**

Livros que vamos se basear:

- Livro oficial do Rust - https://rust-br.github.io/rust-book-pt-br/


## Introducão e conceitos Básico de Rust

- Me apresentar e apresentar a proposta de videos
- Explicacão basica de como a linguagem funciona
- instalar rust e criar o projeto

## Tipos de dados
- Tipos dados
- Mutabilidade de variaveis
- Shadowing
- Como criar uma funcão (pub and private)


## Condicões e repiticões

- condicões (if, else, if else, let if (comentar no 5 video mais profundamente) )
- repeticões (loop, while, for)
- Slices (slices de string)

## Ownership

- Pilhas e Heap
- Regras de Ownership
    1. Cada valor em Rust possui uma variável que é dita seu owner (sua dona).
    1. Pode apenas haver um owner por vez.
    1. Quando o owner sai fora de escopo, o valor será destruído.
- Escopos
- Referencia de varaivel (mut &)

## Structs, imp e enum

- Structs
- Structs de tuplas e structs vazias
- Impl e varios impl
- Enum

## Option, Match

- Option
- Match
- if let

## mod and workspace

- mod 
- workspace

## Colecões mais usadas

- String
- Vec
- HashMap

## Tratamento de Errors

- Panic (unwrap e expect)
- Result (?)

## Generic and Trait

- Generic in fn, imp and struct
- Trait (Display, Ops::Add)


## Serde_json

- Deserialize
- Serialize

## Tests

- Criando um test
- Executando tests

## Clouser

- Clouser anonymos
- clouser move
- clouser problems

## Thread, async/await

- Diferencas de thread e async/await
- criando uma thread
- criando um async/await (tokio)

## Box, Mutex, Rc, Arc

- Explicando a existencia dessas structs
- Mostrando como funciona Box
- Mostrnado como funciona Mutex
- Usando Rc e Arc em exemplos




<style>
body { counter-reset: h1counter h2counter h3counter h4counter h5counter h6counter; }

h1 { counter-reset: h2counter; }
h2 { counter-reset: h3counter; }
h3 { counter-reset: h4counter; }
h4 { counter-reset: h5counter; }
h5 { counter-reset: h6counter; }
h6 {}

h2:before {
    counter-increment: h2counter;
    content: counter(h2counter) ".\0000a0\0000a0";
}

h3:before {
    counter-increment: h3counter;
    content: counter(h2counter) "." counter(h3counter) ".\0000a0\0000a0";
}

h4:before {
    counter-increment: h4counter;
    content: counter(h2counter) "." counter(h3counter) "." counter(h4counter) ".\0000a0\0000a0";
}

h5:before {
    counter-increment: h5counter;
    content: counter(h2counter) "." counter(h3counter) "." counter(h4counter) "." counter(h5counter) ".\0000a0\0000a0";
}

h6:before {
    counter-increment: h6counter;
    content: counter(h2counter) "." counter(h3counter) "." counter(h4counter) "." counter(h5counter) "." counter(h6counter) ".\0000a0\0000a0";
}
</style>