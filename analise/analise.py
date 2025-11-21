import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import json
from collections import defaultdict
from scipy.interpolate import CubicSpline, interp1d
from scipy.signal import savgol_filter
from scipy import stats
import os

source = input()
with open(source, 'r') as f:
    records = json.load(f)

# Criar DataFrame
df = pd.DataFrame(records)

# Converter durações para segundos (assumindo que Duration é em nanosegundos)
df['durations_nano'] = df['durations'].apply(
    lambda x: [d['secs'] * 1e9+ d['nanos']  if isinstance(d, dict) else d  for d in x]
)

# Calcular médias
df['avg_duration'] = df['durations_nano'].apply(lambda x: np.min(x))

df['k'] = df['population'] 
# Agrupar dados
grouped = df.groupby(['operation', 'matrix_type', 'generator', 'k'])['avg_duration'].mean().reset_index()

# Configurar gráfico
plt.figure(figsize=(12, 8))

# Gerar combinações únicas de operação e matrix_type
operations = df['operation'].unique()
matrix_types = df['matrix_type'].unique()
generators = df['generator'].unique()

def rolling_outlier_filter(df, column='f', sorted_by='k', n=3, threshold=2):
    """
    Remove pontos que estão a mais de 'threshold' desvios padrão 
    da média dos n pontos à esquerda e n pontos à direita
    """
    df = df.copy().sort_values(sorted_by).reset_index(drop=True)
    outliers_mask = pd.Series([False] * len(df))
    
    for i in range(len(df)):
        # Definir janela: n pontos à esquerda e n pontos à direita
        left_start = max(0, i - n)
        left_end = max(0, i - 1)  # Não incluir o ponto atual
        right_start = min(len(df) - 1, i + 1)
        right_end = min(len(df) - 1, i + n)
        
        # Coletar pontos da janela (excluindo o ponto atual)
        window_indices = list(range(left_start, left_end + 1)) + list(range(right_start, right_end + 1))
        window_values = df.iloc[window_indices][column]
        
        # Calcular estatísticas da janela (excluindo NaN)
        if len(window_values) > 1:
            mean_val = window_values.mean()
            std_val = window_values.std()
            
            # Verificar se o ponto atual é outlier
            if std_val > 0:  # Evitar divisão por zero
                current_value = df.iloc[i][column]
                z_score = abs(current_value - mean_val) / std_val
                if z_score > threshold:
                    outliers_mask.iloc[i] = True
    
    print(f"Removidos {outliers_mask.sum()} outliers de {len(df)} pontos")
    return df[~outliers_mask]
def lim_inf_filter(df, column='f', sorted_by='k', n=3, threshold=2):
    df = df.copy().sort_values(sorted_by).reset_index(drop=True)
    outliers_mask = pd.Series([False] * len(df))
    min_value = df.iloc[len(df)-1][column]
    for i in range(len(df)-1, -1, -1):  # Evitar divisão por zero
        current_value = df.iloc[i][column]
        if current_value < min_value:
            min_value = current_value
        else:
            outliers_mask.iloc[i] = True
    print(f"Removidos {outliers_mask.sum()} outliers de {len(df)} pontos")
            
    return df[~outliers_mask]

def lim_sup_filter(df, column='f', sorted_by='k', n=3, threshold=2):
    df = df.copy().sort_values(sorted_by).reset_index(drop=True)
    outliers_mask = pd.Series([False] * len(df))
    max_value = df.iloc[0][column]
    for i in range(len(df)):  # Evitar divisão por zero
        current_value = df.iloc[i][column]
        if current_value > max_value:
            max_value = current_value
        else:
            outliers_mask.iloc[i] = True
    print(f"Removidos {outliers_mask.sum()} outliers de {len(df)} pontos")
            
    return df[~outliers_mask]
def plot_smooth_curve(x, y, window_length=7, polyorder=3):
    if len(y) < window_length:
        window_length = len(y) if len(y) % 2 != 0 else len(y) - 1
    if window_length < 3:
        return y  # Não é possível suavizar com menos de 3 pontos
    return savgol_filter(y, window_length, polyorder)
# Plotar linhas para cada combinação
assintoc_functions = {
    '0-constant': lambda k:  1,
    '1-linear': lambda k:  k,
    '2-nlog': lambda k: k * np.log(k) ,
    '3-nsqrt': lambda k: k *np.sqrt(k) ,
    '4-nlogsqrt': lambda k: k * np.log(k)*np.sqrt(k) ,
    '5-quadratic': lambda k: k**2,
    '6-quadratic-log': lambda k: k**2 * np.log(k),
} 

    
for generator in generators:
    for operation in operations:
        for matrix_type in matrix_types:
            subset = grouped[
                (grouped['operation'] == operation) 
                & (grouped['matrix_type'] == matrix_type) 
                & (grouped['generator'] == generator)
            ]
            subset = subset[subset['k'] > 100] 
            subset = rolling_outlier_filter(subset, column='avg_duration', sorted_by='k', n=10, threshold=1)

            for assintotic_name, assintoc_function in assintoc_functions.items():
                subset['f'] =  subset['avg_duration'] / assintoc_function(subset['k'])                
                inf_subset = lim_inf_filter(subset, column='avg_duration', sorted_by='k', n=5, threshold=1)
                sup_subset = lim_sup_filter(subset, column='avg_duration', sorted_by='k', n=5, threshold=1)
                plt.plot(subset['k'], subset['f'], 'o', markersize=4, alpha=0.1)
            
                slope, intercept, r, p, std_err = stats.linregress(subset['k'], subset['f'])
                plt.plot(subset['k'], intercept + slope*subset['k'], '--', label=f"Linear Fit (r={r:.2f})")
                
                mean_f = (subset['f']*subset['k']).sum() / (subset['k'].sum())
                plt.axhline(y=mean_f, color='gray', linestyle='--', alpha=0.5)

    
                
                inf_smooth = plot_smooth_curve(inf_subset['k'], inf_subset['f'], window_length=7, polyorder=3)
                sup_smooth = plot_smooth_curve(sup_subset['k'], sup_subset['f'], window_length=7, polyorder=3)
                plt.plot(inf_subset['k'], inf_smooth, label="carlos", color='blue', linewidth=2)
                plt.plot(sup_subset['k'], sup_smooth, label="carlos", color='red', linewidth=2)

                
                plt.ylim(bottom=0)
                plt.xlabel('População')
                plt.ylabel('Duração Média / População (nano segundos)')
                plt.title('Desempenho de Operações com Matrizes')
                plt.legend(bbox_to_anchor=(1.05, 1), loc='upper left')
                plt.grid(True, alpha=0.3)
                plt.tight_layout()

                # Mostrar gráfico
                out_dir = os.path.join(str(matrix_type), str(operation))
                os.makedirs(out_dir, exist_ok=True)
                outfile = os.path.join(out_dir, f"{assintotic_name}_{generator}_matrix_performance.png")
                plt.savefig(outfile)
                plt.clf()