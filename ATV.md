# Atividade Prática: Implementação do VMTranslator – Parte 1

## (Acesso a Dados, Operações Aritméticas e Lógicas)

> **Objetivo**: Desenvolver a **primeira etapa** do tradutor VM → Assembly Hack, implementando apenas:
> 
> - Comandos `push` e `pop` para todos os segmentos de memória
> - Operações aritméticas (`add`, `sub`, `neg`) e lógicas/relacionais (`eq`, `gt`, `lt`, `and`, `or`, `not`)
> 
> 📖 **Material de apoio**: Parte 1 - Acesso e operações sobre os dados (Notion)
> 
> 🎥 **Vídeo complementar**: Implementação do VMTranslator - Playlist
> 

---

## 👥 Equipe e Linguagem

- ✅ **Mudança de dupla permitida**: Você pode formar nova dupla em relação ao projeto anterior (JackCompiler).
- ✅ **Mudança de linguagem permitida**: Sinta-se à vontade para escolher uma linguagem diferente, caso queira explorar novas ferramentas.
- 📝 **No `README.md`**, registre obrigatoriamente:
    - Nomes completos e identificação (RA/matrícula) dos integrantes
    - Linguagem de programação e versão utilizada
    - Instruções claras de build e execução
    - (Opcional) Breve justificativa da escolha da linguagem

---

## 🗂️ Configuração do Repositório

1. Crie um **novo repositório** com o nome exato:
    
    ```
    vmtranslator
    ```
    
2. Estruture o projeto seguindo **boas práticas da linguagem escolhida**:
    - Separação de módulos: `parser/`, `codewriter/`, `cmd/` ou `main/`
    - Gerenciamento de dependências (ex: `go.mod`, `requirements.txt`, `pom.xml`)
    - Testes unitários (recomendado, mesmo que simples)
3. Inclua um `.gitignore` adequado.
4. **Histórico de commits é critério de avaliação**:
    - Commits atômicos e com mensagens descritivas
    - Exemplos:
        
        ```
        feat(parser): implementa tokenização e commandType
        fix(codewriter): corrige cálculo de endereço para segmento 'that'
        docs(readme): adiciona instruções de execução para Linux
        ```
        

---

## ⚙️ Componentes a Implementar (Parte 1)

### 1️⃣ Parser – Analisador de Arquivos `.vm`

Responsável por ler, filtrar comentários e tokenizar comandos VM.

### API Mínima Sugerida:

| Método | Descrição |
| --- | --- |
| `NewParser(filename)` | Abre o arquivo e prepara a leitura |
| `HasMoreCommands() bool` | Indica se há comandos pendentes |
| `Advance()` | Avança para o próximo comando |
| `CommandType() string` | Retorna: `C_ARITHMETIC`, `C_PUSH`, `C_POP` |
| `Arg1() string` | Retorna o primeiro argumento (ex: `local`, `add`) |
| `Arg2() int` | Retorna o índice (apenas para `push`/`pop`) |

### Exemplo simplificado (Python):

```python
class Parser:
    def __init__(self, filename):
        with open(filename) as f:
            self.lines = [
                line.split()
                for line in f
                if line.strip() and not line.strip().startswith("//")
            ]
        self.index = 0

    def hasMoreCommands(self):
        return self.index < len(self.lines)

    def advance(self):
        cmd = self.lines[self.index]
        self.index += 1
        return cmd
```

> 💡 Dica: Em linguagens estáticas, use `enum` para tipos de comando e `struct`/`class` para representar comandos parseados.
> 

---

### 2️⃣ CodeWriter – Gerador de Assembly Hack

Traduz comandos VM para instruções `.asm` válidas para a CPU Hack.

### API Mínima Requerida:

| Método | Descrição |
| --- | --- |
| `NewCodeWriter(filename)` | Abre arquivo `.asm` para escrita |
| `WriteArithmetic(cmd string)` | Gera código para `add`, `sub`, `neg`, `eq`, `gt`, `lt`, `and`, `or`, `not` |
| `WritePush(segment string, index int)` | Gera código para empilhar valores |
| `WritePop(segment string, index int)` | Gera código para desempilhar valores |
| `Close()` | Finaliza e fecha o arquivo |

### Segmentos de memória suportados (Parte 1):

| Segmento | Descrição | Endereço base |
| --- | --- | --- |
| `constant` | Valores imutáveis | N/A (usa valor direto) |
| `local` | Variáveis locais da função | `LCL` (RAM[1]) |
| `argument` | Argumentos da função | `ARG` (RAM[2]) |
| `this` | Base para objetos/arrays | `THIS` (RAM[3]) |
| `that` | Base para estruturas dinâmicas | `THAT` (RAM[4]) |
| `temp` | Registradores temporários | RAM[5]–RAM[12] |
| `pointer` | Acesso a `this`/`that` | RAM[3] ou RAM[4] |
| `static` | Variáveis estáticas da classe | `StaticBase + index` |

> 📌 **Importante**: Para `push constant X`, basta carregar o valor `X` na pilha. Não há leitura de memória.
> 

---

### 3️⃣ Main (VMTranslator) – Orquestrador

```go
func main() {
    input := os.Args[1]  // ex: BasicTest.vm
    output := strings.Replace(input, ".vm", ".asm", 1)

    p := parser.NewParser(input)
    cw := codewriter.NewCodeWriter(output)

    for p.HasMoreCommands() {
        p.Advance()
        switch p.CommandType() {
        case "C_ARITHMETIC":
            cw.WriteArithmetic(p.Arg1())
        case "C_PUSH":
            cw.WritePush(p.Arg1(), p.Arg2())
        case "C_POP":
            cw.WritePop(p.Arg1(), p.Arg2())
        }
    }
    cw.Close()
}
```

---

## 🧪 Testando Sua Implementação

### 📦 Materiais de Teste Atualizados

Utilize os arquivos do **Project 07** disponíveis em:

🔗 https://github.com/profsergiocosta/nand2tetris-suite/tree/main/projects/07

### Estrutura de Testes:

```
projects/07/
├── StackArithmetic/
│   ├── SimpleAdd/
│   │   ├── SimpleAdd.vm
│   │   ├── SimpleAdd.tst
│   │   └── SimpleAdd.cmp
│   └── ...
└── MemoryAccess/
    ├── BasicTest/
    │   ├── BasicTest.vm
    │   ├── BasicTest.tst
    │   └── BasicTest.cmp
    └── ...
```

### Fluxo de Validação:

1. Traduza: `./vmtranslator MemoryAccess/BasicTest/BasicTest.vm`
2. Abra o **CPUEmulator** (do Nand2Tetris)
3. Carregue o script: `MemoryAccess/BasicTest/BasicTest.tst`
4. Execute e compare a saída (`BasicTest.out`) com `BasicTest.cmp`

✅ **Sucesso**:

```
Comparison ended successfully.
```

❌ **Falha**: Valores divergentes serão destacados — revise a lógica de endereçamento ou operações na pilha.

---

## 📦 Entregáveis

- [ ]  Repositório `vmtranslator` criado e acessível
- [ ]  Código-fonte organizado por módulos
- [ ]  `README.md` completo com:
    - Nomes da dupla
    - Linguagem e versão
    - Como compilar/executar
    - Exemplo de uso
- [ ]  Mínimo de **5 commits significativos** no histórico
- [ ]  Tradução funcional dos testes:
    - `StackArithmetic/SimpleAdd`
    - `MemoryAccess/BasicTest`

---

## 🎯 Critérios de Avaliação

| Critério | Peso | Detalhes |
| --- | --- | --- |
| **Correção funcional** | 40% | Saída `.asm` gera comportamento idêntico ao esperado nos testes |
| **Qualidade do código** | 25% | Legibilidade, coesão, uso de padrões da linguagem |
| **Documentação** | 15% | README claro, comentários úteis, API bem definida |
| **Versionamento** | 10% | Commits atômicos, mensagens descritivas, evolução lógica |
| **Testes e validação** | 10% | Capacidade de executar e passar nos scripts `.tst` fornecidos |

---

## 📚 Recursos Adicionais

- 📘 *The Elements of Computing Systems*, Cap. 8 — Nand2Tetris Book
- 🎥 Playlist: Implementação do VMTranslator
- 🎥 Testando operações de memória e aritmética
- 🌐 Notion: Parte 1 - Acesso e operações sobre os dados
- 🗂️ Repositório de testes: projects/07

---

> 💡 **Dica de implementação incremental**:
> 
> 1. Comece com `push constant` + `pop local` → valide com `SimpleAdd`
> 2. Adicione `add`/`sub` → teste novamente
> 3. Expanda para outros segmentos (`argument`, `this`, `that`)
> 4. Implemente operações lógicas (`eq`, `gt`, `lt`) por último
> 
> Teste **cada comando isoladamente** antes de integrar. Isso reduz drasticamente o tempo de depuração.
> 

Boa implementação! 🚀 Em caso de dúvidas, abra uma *issue* no repositório ou entre em contato com o professor.