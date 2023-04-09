# Trabalho Prático 1

## PUCMINAS - Engenharia de Software - Teste de Software - Prof. Cleiton Tavares

### Descrição

Rust é uma linguagem de programação mantida pela Mozilla Research. O propósito principal dela é ser uma linguagem de sistema segura em questão de tratamento de memória e ser focada em desempenho. A framework de test usanda para ele é fornecida pela própria organização. Dessa forma, não é necessário buscar crates (bibliotecas) externas para criar testes (mesmo que haja no ecossistema).

### Níveis de testes

A comunidade do Rust oferece três níveis de testes, cada um organizado e executado pelo compilador de sua maneira.

- Unit Tests:
  - Testes unitários para lidar com funções atômicas. Esses testes têm que estar no mesmo arquivo onde as funções são definidas;
- Integration Tests
  - Testes para testar os componentes. Esse tipo só consegue lidar com **_libs_** e não com executáveis. Dessa forma, ele é importante para testar API de **crates**;
- Doc Tests.
  - Server para testar exemplos de comentários de métodos ou crates das **_libs_**. Em programação de sistema, é comum ver vários comentários em funções, esse teste ajuda a manter a sincronização da função com sua documentação.

### Tipos de testes

A biblioteca oficial oferece testes de funcionalidade e perfomance. A de funcionalidade já vimos ali, o de perfomance é mostrada no sumário da execução de teste depois de finalizada. Ele mostra o quanto demorou para executar.

### Técnicas de Testes

- O teste de integração é um teste de caixa-preta. Como podemos ver, ele não conhece nenhuma estrutura interna da API biblioteca, a não ser o módulo público;
- Os testes unitários é um teste de caixa-branca, pois além de testar as funções privadas, ele tem acesso aos recursos internos da API.

### Forma de instalação do Rust

Segue o a [página oficial](https://www.rust-lang.org/tools/install) com tutoriais de instação de acordo com SO.

### Como executar o programa

Rust oferece a ferramenta **cargo** para execução de crates. Segue os seguintes passos:

1. Verifique se a instação foi bem sucedida com o comando `cargo --version`. Se não aparecer a versão, tente abrir um novo terminal ou instalar de novo.
2. Execute o seguinte comando, no diretório do projeto, `<CASE_INSENSITIVE=1> cargo run [PALAVRA_A_CONSULTAR] [NOME_DO_ARQUIVO]`. Exemplo: `cargo run Feriado cronograma.txt`. A variável de ambiente _CASE_INSENSITIVE_ é opcional.

### Como executar os testes

Rust oferece o comando `cargo test` para execução de testes dos crates. Há diferentes maneiras de executar, cada passo a seguir mostra uma maneira.

- Para executar todos os testes (unit, integration e doc), execute `cargo test`;
- Para executar um único teste, execute `cargo test -- [NOME DO TESTE]`;
- Para executar os Doctests, execute `cargo test --doc`
