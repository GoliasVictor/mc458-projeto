import json
import os
import numpy as np
import pandas as pd

# Carrega e processa dados
with open(input(), 'r') as f:
    data = json.load(f)

# Cria DataFrame e processa durações
df = pd.json_normalize(data, 'durations', ['matrix_type', 'i', 'population', 'operation'])
df['total_ns'] = df['secs'] * 1e9 + df['nanos']

# Função de conversão
def format_duration(ns):
    for unit, factor in [('s', 1e9), ('ms', 1e6), ('µs', 1e3), ('ns', 1)]:
        if ns >= factor or unit == 'ns':
            return f"{ns/factor:.3f} {unit}"

# Agrupa e formata
result = (df.groupby(['operation', 'matrix_type', 'i', 'population'])['total_ns']
          .mean()
          .reset_index())
result['duration'] = result['total_ns'].apply(format_duration)
def cast(x):
    if x % 1 < 0.000001:
        return f"{int(x)}"
    return f"{x}"
# Gera tabelas
for operation in result['operation'].unique():
    op_data = result[result['operation'] == operation]
    
    pivot_table = (op_data.pivot_table(
        index=['i', 'population'],
        columns='matrix_type',
        values='duration',
        aggfunc='first'
    )
    .sort_index()
    .reset_index())
    

    pivot_table["percentage"] = (100 * pivot_table["population"] / np.pow(10, 2*pivot_table["i"].astype(int))).apply(lambda x: cast(x) + r"\%")
    pivot_table['i'] = pivot_table['i'].apply(lambda x: f"$10^{x}$x$10^{x}$")
    cols = ["i", "percentage"] + [col for col in pivot_table.columns if col not in ['i', 'percentage']]

    latex = pivot_table[cols].fillna('').to_latex(
        index=False,
        header=["Tamanho", "Ocupação"] + [col for col in cols[2:]],
        escape=False,
        column_format='c c ' + 'c ' * len(cols[2:])
    )
    output_dir = 'tables'
    os.makedirs(output_dir, exist_ok=True)
    with open(os.path.join(output_dir, f'table_{operation}.tex'), 'w') as f:
        f.write(latex)
    
    print(f"Operação: {operation}")
    print(latex)
    print("="*80)