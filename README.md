# MC458 - Projeto: Implementação e Análise de Matrizes Esparsas

Este projeto implementa e compara diferentes estruturas de dados para representação de matrizes esparsas em Rust, desenvolvido como parte da disciplina MC458 (Projeto e Análise de Algoritmos I).

## Visão Geral

Matrizes esparsas são matrizes onde a maioria dos elementos é zero. Este projeto implementa três diferentes abordagens para representá-las eficientemente:

1. **TableMatrix**: Representação densa usando vetores de vetores (`Vec<Vec<f64>>`)
2. **HashMapMatrix**: Representação esparsa usando `HashMap` para armazenar apenas valores não-zero
3. **TreeMatrix**: Representação esparsa usando `BTreeMap` para armazenar valores não-zero de forma ordenada

O projeto inclui benchmarks detalhados que comparam o desempenho dessas implementações em operações matriciais comuns (multiplicação, adição, transposição, etc.).

## Arquitetura do Projeto

### Trait Matrix

O projeto define um trait `Matrix` que especifica as operações básicas que todas as implementações devem suportar:

```rust
pub trait Matrix {
    fn new(size: Pair) -> Self;              // Cria nova matriz
    fn set(&mut self, pos: Pair, value: f64); // Define valor
    fn get(&self, pos: Pair) -> f64;         // Obtém valor
    fn transposed(self) -> Self;             // Transpõe matriz
    fn add(a: &Self, b: &Self) -> Self;      // Soma matrizes
    fn mul(a: &Self, b: &Self) -> Self;      // Multiplica matrizes
    fn muls(a: &Self, scalar: f64) -> Self;  // Multiplica por escalar
    fn from_info(info: &MatrixInfo) -> Self; // Cria de metadata
    fn to_info(&self) -> MatrixInfo;         // Converte para metadata
}
```

### Implementações

#### 1. TableMatrix (`src/table_matrix.rs`)

Matriz densa que armazena todos os valores (incluindo zeros) em um vetor de vetores.

- **Vantagens**: Acesso O(1) a qualquer elemento
- **Desvantagens**: Alto uso de memória para matrizes esparsas
- **Uso recomendado**: Matrizes densas ou pequenas

#### 2. MapMatrix com HashMap (`src/map_matrix.rs`)

Matriz esparsa que usa `HashMap` para armazenar apenas valores não-zero.

- **Vantagens**: Eficiente em memória, bom desempenho em operações aleatórias
- **Desvantagens**: Sem garantia de ordem, overhead de hash
- **Uso recomendado**: Matrizes muito esparsas com acesso aleatório

#### 3. MapMatrix com BTreeMap

Matriz esparsa que usa `BTreeMap` para armazenar valores não-zero de forma ordenada.

- **Vantagens**: Eficiente em memória, valores ordenados, boa localidade de cache
- **Desvantagens**: Operações um pouco mais lentas que HashMap
- **Uso recomendado**: Matrizes esparsas onde ordem importa

### Módulo de Alocação (`src/alloc.rs`)

O projeto inclui um alocador personalizado (`TrackingAllocator`) que monitora alocações e desalocações de memória durante os benchmarks, permitindo análise precisa do uso de memória de cada implementação.

## Instalação

### Pré-requisitos

- **Rust e Cargo**: Instale seguindo as instruções em https://www.rust-lang.org/tools/install
- **Python 3.x** (opcional, para análise de resultados)
- **pip** (opcional, para instalar dependências Python)

### Instalação do Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Instalação das Dependências Python (Opcional)

Para análise de resultados com gráficos e tabelas:

```bash
pip install -r analise/requirements.txt
```

## Uso

### Executando Benchmarks

Para executar todos os benchmarks:

```bash
cargo bench
```

Para executar benchmarks específicos:

```bash
cargo bench --bench benchmarks
```

Os resultados serão salvos em `target/criterion/` e também serão exportados como JSON para análise posterior.

### Compilando a Biblioteca

Para compilar o projeto:

```bash
cargo build --release
```

### Executando Testes

```bash
cargo test
```

## Análise dos Resultados

O projeto inclui scripts Python para analisar os resultados dos benchmarks.

### Instalando Dependências

```bash
pip install -r analise/requirements.txt
```

### Gerando Gráficos

```bash
python analise/graficos.py <<< "b1.json"
```

### Gerando Tabelas

```bash
python analise/tabelas.py <<< "b2.json"
```

Os gráficos e tabelas gerados ajudam a visualizar:
- Tempo de execução vs. tamanho da matriz
- Uso de memória vs. esparsidade
- Comparação entre implementações
- Análise assintótica

## Estrutura do Projeto

```
.
├── Cargo.toml              # Configuração do projeto Rust
├── Cargo.lock              # Lock de dependências
├── README.md               # Este arquivo
├── src/                    # Código-fonte
│   ├── lib.rs             # Ponto de entrada da biblioteca
│   ├── basic.rs           # Definição do trait Matrix e tipos básicos
│   ├── table_matrix.rs    # Implementação com Vec<Vec<f64>>
│   ├── map_matrix.rs      # Implementação base para matrizes baseadas em Map
│   ├── map_matrix/        # Módulos auxiliares para MapMatrix
│   │   ├── hash_map.rs    # Implementação do Map com HashMap
│   │   ├── tree_map.rs    # Implementação do Map com BTreeMap
│   │   └── transposable_map.rs  # Mapa que pode ser transposto
│   └── alloc.rs           # Alocador para rastreamento de memória
├── benches/               # Benchmarks
│   └── benchmarks/
│       ├── main.rs        # Código principal dos benchmarks
│       └── matrix_generator.rs  # Gerador de matrizes para testes
├── analise/               # Scripts de análise em Python
│   ├── graficos.py        # Gera gráficos dos resultados
│   ├── tabelas.py         # Gera tabelas dos resultados
│   └── requirements.txt   # Dependências Python
├── relatorio/             # Relatório técnico em LaTeX
│   └── main.tex           # Documento principal
└── relatorio.pdf          # Relatório compilado
```

## Exemplos de Uso

### Criando e Usando Matrizes

```rust
use projeto::{HashMapMatrix, TableMatrix, TreeMatrix, Matrix};

// Criar uma matriz 3x3 usando HashMap
let mut matrix = HashMapMatrix::new((3, 3));

// Definir valores
matrix.set((0, 0), 1.0);
matrix.set((1, 1), 2.0);
matrix.set((2, 2), 3.0);

// Obter valores
let value = matrix.get((0, 0)); // Retorna 1.0

// Transpor matriz
let transposed = matrix.transposed();

// Multiplicação por escalar
let scaled = HashMapMatrix::muls(&matrix, 2.0);
```

### Operações entre Matrizes

```rust
use projeto::{HashMapMatrix, Matrix};

let mut a = HashMapMatrix::new((2, 2));
a.set((0, 0), 1.0);
a.set((1, 1), 2.0);

let mut b = HashMapMatrix::new((2, 2));
b.set((0, 0), 3.0);
b.set((1, 1), 4.0);

// Soma
let c = HashMapMatrix::add(&a, &b);

// Multiplicação
let d = HashMapMatrix::mul(&a, &b);
```

## Complexidade das Operações

### TableMatrix
- `get(pos)`: O(1)
- `set(pos, value)`: O(1)
- `mul(a, b)`: O(n³) onde n é a dimensão da matriz
- `add(a, b)`: O(n²)
- `transposed()`: O(n²)

### HashMapMatrix
- `get(pos)`: O(1) médio
- `set(pos, value)`: O(1) médio
- `mul(a, b)`: O(k₁ × k₂ / n) onde k₁, k₂ são o número de elementos não-zero
- `add(a, b)`: O(k₁ + k₂)
- `transposed()`: O(1) (usa flag de transposição)

### TreeMatrix
- `get(pos)`: O(log k) onde k é o número de elementos não-zero
- `set(pos, value)`: O(log k)
- `mul(a, b)`: O(k₁ × k₂ / n × log k)
- `add(a, b)`: O((k₁ + k₂) × log k)
- `transposed()`: O(1) (usa flag de transposição)

## Contribuindo

Este é um projeto acadêmico, mas sugestões e melhorias são bem-vindas através de issues e pull requests.

## Licença

Este projeto é desenvolvido para fins educacionais como parte da disciplina MC458 da Unicamp.

## Autor

Desenvolvido como projeto da disciplina MC458 - Projeto e Análise de Algoritmos I.
