# VMTranslator — Nand2Tetris Project 07 (Parte 1)

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

**Por que Rust?** Segurança de memória em tempo de compilação, tipagem forte com `enum` e `match` exaustivo — ideal para modelar os tipos de comando VM e garantir que todos os segmentos sejam tratados corretamente.

---

## Estrutura do Projeto

```
vmtranslator-rust/
├── src/
│   ├── main.rs          # Orquestrador: CLI → Parser → CodeWriter → .asm
│   ├── parser.rs        # Leitura, remoção de comentários e tokenização do .vm
│   └── code_writer.rs   # Geração de instruções Hack Assembly
├── tests/
│   ├── SimpleAdd.vm
│   ├── StackTest.vm
│   ├── BasicTest.vm
│   ├── PointerTest.vm
│   ├── StaticTest.vm
│   └── output/          # Arquivos .asm gerados
├── translate_all.sh     # Script para traduzir todos os .vm de uma vez
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

### Arquivo único

```bash
cargo run -- <caminho/para/arquivo.vm>
```

O arquivo `.asm` é gerado em `output/` dentro do mesmo diretório do `.vm`.

**Exemplo:**

```bash
cargo run -- tests/SimpleAdd.vm
# Gera: tests/output/SimpleAdd.asm
```

### Todos os arquivos de teste de uma vez

```bash
./translate_all.sh
```

---

## Testes

### Testes unitários do parser

```bash
cargo test
```

### Validação no CPU Emulator (Nand2Tetris)

1. Gere o `.asm` com `cargo run -- <arquivo.vm>`
2. Copie o `.asm` gerado para o mesmo diretório do `.tst` correspondente
3. Abra o **CPUEmulator** do Nand2Tetris
4. Carregue o script `.tst` e execute
5. Resultado esperado: `Comparison ended successfully`

---

## Segmentos Suportados

| Segmento   | Endereço base |
|------------|---------------|
| `constant` | valor literal |
| `local`    | `LCL` (RAM[1]) |
| `argument` | `ARG` (RAM[2]) |
| `this`     | `THIS` (RAM[3]) |
| `that`     | `THAT` (RAM[4]) |
| `temp`     | `RAM[5]`–`RAM[12]` |
| `pointer`  | `THIS` ou `THAT` |
| `static`   | `@FileName.i` |

## Operações Suportadas

`add`, `sub`, `neg`, `eq`, `gt`, `lt`, `and`, `or`, `not`
