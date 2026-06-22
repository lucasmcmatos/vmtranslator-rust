# VMTranslator — Nand2Tetris Projetos 07 e 08

Tradutor de código VM (linguagem intermediária) para Assembly Hack, implementado como parte da disciplina de Compiladores.

---

## Integrantes

| Nome | Matrícula |
|------|-----------|
| Lucas Martins Campos Matos | 20250013668 |

---

## Linguagem

- **Rust** — versão `1.95.0`
- Sem dependências externas (apenas `std`)

**Por que Rust?** Segurança de memória em tempo de compilação, tipagem forte com `enum` e `match` exaustivo — ideal para modelar os tipos de comando VM e garantir que todos os segmentos e comandos sejam tratados corretamente.

---

## Estrutura do Projeto

```
vmtranslator-rust/
├── src/
│   ├── main.rs          # CLI: aceita arquivo .vm ou diretório
│   ├── parser.rs        # Leitura, remoção de comentários e tokenização
│   └── code_writer.rs   # Geração de instruções Hack Assembly
├── tests/
│   ├── part1/           # Testes da Parte 1 (arquivo único)
│   │   ├── SimpleAdd/   # SimpleAdd.vm + SimpleAdd.tst + SimpleAdd.cmp
│   │   ├── BasicTest/   # BasicTest.vm + BasicTest.tst + BasicTest.cmp
│   │   ├── PointerTest/ # PointerTest.vm
│   │   ├── StackTest/   # StackTest.vm
│   │   └── StaticTest/  # StaticTest.vm
│   └── part2/           # Testes da Parte 2 (diretórios com múltiplos .vm)
│       ├── ProgramFlow/
│       └── FunctionCalls/
├── translate_all.sh     # Script para traduzir todos os testes da Parte 1
└── Cargo.toml
```

---

## Como Compilar

```bash
cargo build --release
```

O binário gerado fica em `target/release/vmtranslator-rust`.

---

## Como Executar

### Arquivo único (Parte 1)

```bash
cargo run -- <caminho/para/arquivo.vm>
```

O arquivo `.asm` é gerado **no mesmo diretório** do `.vm` (ao lado do `.tst` e `.cmp`). Bootstrap **não** é emitido.

```bash
cargo run -- tests/part1/SimpleAdd/SimpleAdd.vm
# Gera: tests/part1/SimpleAdd/SimpleAdd.asm
```

### Diretório (Parte 2)

```bash
cargo run -- <caminho/para/diretório/>
```

Todos os `.vm` do diretório são traduzidos em um único `.asm`, gerado **dentro do próprio diretório**. Bootstrap (`SP = 256` + `call Sys.init 0`) é emitido automaticamente.

```bash
cargo run -- tests/part2/FunctionCalls/NestedCall/
# Gera: tests/part2/FunctionCalls/NestedCall/NestedCall.asm
```

---

## Funcionalidades Implementadas

### Parte 1 — Comandos de Memória e Aritmética

| Categoria | Comandos |
|-----------|----------|
| Aritmética | `add`, `sub`, `neg`, `eq`, `gt`, `lt`, `and`, `or`, `not` |
| Memória (push/pop) | `constant`, `local`, `argument`, `this`, `that`, `temp`, `pointer`, `static` |

### Parte 2 — Controle de Fluxo e Sub-rotinas

| Categoria | Comandos | Descrição |
|-----------|----------|-----------|
| Controle de fluxo | `label`, `goto`, `if-goto` | Rótulos com escopo `função$label` |
| Sub-rotinas | `function`, `call`, `return` | Convenção de chamada completa com frame |
| Bootstrap | automático | `SP = 256` + `call Sys.init 0` no modo diretório |

---

## Testes Unitários

```bash
cargo test
```

37 testes cobrindo parser, geração de código de controle de fluxo, bootstrap, `call` e `return`.

---

## Validação no CPU Emulator (Nand2Tetris)

### Pré-requisito

Baixe o pacote do Nand2Tetris e localize `projects/08/`.

### Ordem recomendada de testes

| Diretório | Foco | Comando |
|-----------|------|---------|
| `tests/part2/ProgramFlow/BasicLoop/` | `label`, `goto`, `if-goto` | `cargo run -- tests/part2/ProgramFlow/BasicLoop/` |
| `tests/part2/ProgramFlow/FibonacciSeries/` | `if-goto` com recursão | `cargo run -- tests/part2/ProgramFlow/FibonacciSeries/` |
| `tests/part2/FunctionCalls/SimpleFunction/` | `function` / `return` | `cargo run -- tests/part2/FunctionCalls/SimpleFunction/` |
| `tests/part2/FunctionCalls/NestedCall/` | bootstrap + `call`/`return` aninhados | `cargo run -- tests/part2/FunctionCalls/NestedCall/` |

### Fluxo de validação

```bash
# 1. Traduza o diretório
cargo run -- tests/part2/FunctionCalls/NestedCall/

# 2. Abra o CPUEmulator do Nand2Tetris
# 3. Carregue: tests/part2/FunctionCalls/NestedCall/NestedCall.tst
# 4. Execute — resultado esperado: "Comparison ended successfully"
```

---

## Exemplo de Saída

Entrada (`BasicLoop.vm`):
```
function Sys.init 0
  push constant 0
  label LOOP
  push constant 1
  add
  if-goto LOOP
```

Saída (trecho do `BasicLoop.asm`):
```asm
// Bootstrap code
@256
D=A
@SP
M=D
// call Sys.init 0
...
// function Sys.init 0
(Sys.init)
// push constant 0
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
// label LOOP
(Sys.init$LOOP)
// if-goto LOOP
@SP
AM=M-1
D=M
@Sys.init$LOOP
D;JNE
```
