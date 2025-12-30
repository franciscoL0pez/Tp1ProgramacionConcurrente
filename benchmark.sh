
#!/bin/bash

# Script para medir performance del análisis de datos de Twitch
# Uso: ./benchmark.sh <ruta_dataset>

if [ $# -ne 1 ]; then
    echo "Uso: $0 <ruta_dataset>"
    exit 1
fi

DATASET=$1
THREADS=(2 4 8)
RESULTS_FILE="benchmark_results.txt"

# Verificar que el dataset existe
if [ ! -f "$DATASET" ]; then
    echo "Error: El archivo $DATASET no existe"
    exit 1
fi

# Compilar en modo release
echo "Compilando en modo release..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Error en la compilación"
    exit 1
fi

# Limpiar archivo de resultados previos
echo "=== BENCHMARK RESULTS ===" > $RESULTS_FILE
echo "Dataset: $DATASET" >> $RESULTS_FILE
echo "Fecha: $(date)" >> $RESULTS_FILE
echo "" >> $RESULTS_FILE

echo "Iniciando benchmarks..."

for threads in "${THREADS[@]}"; do
    echo "=== EJECUTANDO CON $threads THREADS ===" | tee -a $RESULTS_FILE
    
    # Ejecutar 3 veces para obtener promedio
    for run in {1..3}; do
        echo "  Run $run:" | tee -a $RESULTS_FILE
        
        # Usar /usr/bin/time para obtener estadísticas detalladas
        /usr/bin/time -f "    Tiempo real: %e segundos\n    Memoria máxima: %M KB\n    CPU utilizada: %P" \
            ./target/release/TP0ProgramacionConcurrente "$DATASET" $threads both 2>&1 | \
            tee -a $RESULTS_FILE
        
        echo "" >> $RESULTS_FILE
    done
    
    echo "----------------------------------------" >> $RESULTS_FILE
    echo ""
done

echo "Benchmarks completados. Resultados guardados en $RESULTS_FILE"