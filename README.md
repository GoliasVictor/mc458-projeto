# Instalando 

Para executar o projeto basta ter instalado o Rust e o Cargo. Você pode instalar o Rust seguindo as instruções em https://www.rust-lang.org/tools/install.
Depois de instalar o Rust, você executa o benchmark com o comando:

```bash
cargo bench
```

# Análise dos Resultados
Para analisar os resultados dos benchmarks, você pode usar os scripts Python localizados na pasta `analise`. Certifique-se de ter as bibliotecas necessárias instaladas, como `pandas` e `matplotlib`. Você pode instalar essas bibliotecas usando pip:

```bash
pip install -r analise/requirements.txt
```

Em seguida, você pode executar os scripts na pasta raiz do projeto para gerar tabelas e gráficos dos resultados:

```bash
python analise/graficos.py <<< "b1.json"
python analise/tabelas.py <<< "b2.json"
```

# Estrutura do Projeto
- `benches/`: Contém os benchmarks do projeto.
- `analise/`: Contém scripts para análise dos resultados dos benchmarks.
- `src/`: Contém o código-fonte do projeto.
- `Cargo.toml`: Arquivo de configuração do Cargo.
