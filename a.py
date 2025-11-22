import json
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
from scipy import stats

# Carregar o arquivo JSON
with open('b1.json', 'r') as f:
    data = json.load(f)

# Converter para DataFrame
df = pd.DataFrame(data)

# Converter durations para nanossegundos (mais fácil de trabalhar)
df['duration_ns'] = df['durations'].apply(lambda x: x[0]['secs'] * 1e9 + x[0]['nanos'])
df['duration_ms'] = df['duration_ns'] / 1e6  # Converter para milissegundos

print("=== ANÁLISE EXPLORATÓRIA DOS DADOS ===")
print(f"Total de registros: {len(df)}")
print(f"Colunas disponíveis: {df.columns.tolist()}")
print("\nValores únicos por coluna:")
for col in ['matrix_type', 'operation', 'generator', 'occupation']:
    print(f"{col}: {df[col].unique()}")

print(f"\nEstatísticas descritivas - Population: {df['population'].describe()}")
print(f"Estatísticas descritivas - Size: {df['size'].describe()}")
print(f"Estatísticas descritivas - Duration (ms): {df['duration_ms'].describe()}")

# Análise por operação
print("\n=== ANÁLISE POR OPERAÇÃO ===")
operation_stats = df.groupby('operation').agg({
    'duration_ms': ['mean', 'std', 'min', 'max', 'count'],
    'population': 'mean',
    'size': 'mean'
}).round(2)
print(operation_stats)

# Análise por occupation
print("\n=== ANÁLISE POR OCCUPATION ===")
occupation_stats = df.groupby('occupation').agg({
    'duration_ms': ['mean', 'std', 'min', 'max'],
    'population': 'mean',
    'size': 'mean',
    'operation': 'count'
}).round(2)
print(occupation_stats)

# Configurar o estilo dos gráficos
plt.style.use('default')
fig, axes = plt.subplots(2, 3, figsize=(18, 12))
fig.suptitle('Análise de Desempenho das Operações de Matriz', fontsize=16)

# 1. Distribuição das durações
axes[0, 0].hist(df['duration_ms'], bins=50, alpha=0.7, color='skyblue', edgecolor='black')
axes[0, 0].set_xlabel('Duração (ms)')
axes[0, 0].set_ylabel('Frequência')
axes[0, 0].set_title('Distribuição das Durações')
axes[0, 0].grid(True, alpha=0.3)

# 2. Duração vs Population (colorido por operação)
scatter = axes[0, 1].scatter(df['population'], df['duration_ms'], 
                            c=df['operation'].map({'mul': 'red', 'add': 'blue'}), 
                            alpha=0.6, s=30)
axes[0, 1].set_xlabel('Population')
axes[0, 1].set_ylabel('Duração (ms)')
axes[0, 1].set_title('Duração vs Population')
axes[0, 1].grid(True, alpha=0.3)
# Adicionar legenda
from matplotlib.lines import Line2D
legend_elements = [Line2D([0], [0], marker='o', color='w', markerfacecolor='red', markersize=8, label='Multiplicação'),
                   Line2D([0], [0], marker='o', color='w', markerfacecolor='blue', markersize=8, label='Adição')]
axes[0, 1].legend(handles=legend_elements)

# 3. Duração vs Size (colorido por occupation)
scatter2 = axes[0, 2].scatter(df['size'], df['duration_ms'], 
                             c=df['occupation'], alpha=0.6, s=30, cmap='viridis')
axes[0, 2].set_xlabel('Size')
axes[0, 2].set_ylabel('Duração (ms)')
axes[0, 2].set_title('Duração vs Size (colorido por Occupation)')
axes[0, 2].grid(True, alpha=0.3)
plt.colorbar(scatter2, ax=axes[0, 2], label='Occupation')

# 4. Boxplot por operação
df.boxplot(column='duration_ms', by='operation', ax=axes[1, 0])
axes[1, 0].set_title('Duração por Tipo de Operação')
axes[1, 0].set_ylabel('Duração (ms)')

# 5. Boxplot por occupation
df.boxplot(column='duration_ms', by='occupation', ax=axes[1, 1])
axes[1, 1].set_title('Duração por Occupation')
axes[1, 1].set_ylabel('Duração (ms)')

# 6. Heatmap de correlação
correlation_data = df[['population', 'size', 'occupation', 'duration_ms']]
corr_matrix = correlation_data.corr()
im = axes[1, 2].imshow(corr_matrix, cmap='coolwarm', aspect='auto', vmin=-1, vmax=1)
axes[1, 2].set_xticks(range(len(corr_matrix.columns)))
axes[1, 2].set_yticks(range(len(corr_matrix.columns)))
axes[1, 2].set_xticklabels(corr_matrix.columns, rotation=45)
axes[1, 2].set_yticklabels(corr_matrix.columns)
axes[1, 2].set_title('Matriz de Correlação')

# Adicionar valores na heatmap
for i in range(len(corr_matrix.columns)):
    for j in range(len(corr_matrix.columns)):
        axes[1, 2].text(j, i, f'{corr_matrix.iloc[i, j]:.2f}', 
                       ha='center', va='center', color='white' if abs(corr_matrix.iloc[i, j]) > 0.5 else 'black')

plt.colorbar(im, ax=axes[1, 2])

plt.tight_layout()
plt.show()

# Análise estatística adicional
print("\n=== ANÁLISE ESTATÍSTICA DETALHADA ===")

# Correlações
print("\nCorrelações:")
print(corr_matrix)

# Teste t entre operações
mul_durations = df[df['operation'] == 'mul']['duration_ms']
add_durations = df[df['operation'] == 'add']['duration_ms']
t_stat, p_value = stats.ttest_ind(mul_durations, add_durations)
print(f"\nTeste t entre multiplicação e adição:")
print(f"t-statistic: {t_stat:.4f}, p-value: {p_value:.4f}")

# Análise por grupo (operation × occupation)
print("\n=== ANÁLISE POR GRUPO (Operation × Occupation) ===")
group_analysis = df.groupby(['operation', 'occupation']).agg({
    'duration_ms': ['mean', 'std', 'count'],
    'population': 'mean',
    'size': 'mean'
}).round(2)
print(group_analysis)

# Visualização adicional: performance por grupo
plt.figure(figsize=(12, 6))
sns.boxplot(data=df, x='occupation', y='duration_ms', hue='operation')
plt.title('Desempenho por Occupation e Operação')
plt.ylabel('Duração (ms)')
plt.xlabel('Occupation')
plt.yscale('log')  # Escala logarítmica para melhor visualização
plt.grid(True, alpha=0.3)
plt.legend(title='Operação')
plt.show()

# Análise de regressão simples
print("\n=== ANÁLISE DE REGRESSÃO ===")
print("Relação entre Size e Duração:")

# Filtrar apenas multiplicação para análise mais limpa
mul_df = df[df['operation'] == 'mul']

for occupation in sorted(mul_df['occupation'].unique()):
    subset = mul_df[mul_df['occupation'] == occupation]
    if len(subset) > 1:
        slope, intercept, r_value, p_value, std_err = stats.linregress(subset['size'], subset['duration_ms'])
        print(f"Occupation {occupation}: R² = {r_value**2:.4f}, p = {p_value:.4f}")

print("\nAnálise concluída!")