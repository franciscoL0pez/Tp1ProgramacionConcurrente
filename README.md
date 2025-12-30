[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/VT9bB7CI)

# TP1 - Análisis de Datos de Twitch 

**Alumno:** López Francisco  
**Padrón:** 107614

## Dataset Utilizado

**Fuente:** [Top Streamers Chat Dataset - Kaggle](https://www.kaggle.com/datasets/girlazo/top-streamers-chat)

Una vez descargado el dataset de Kaggle, se debe descomprimir el archivo `10M_Messages.json.zip` para obtener el archivo `10M_Messages.json` que contiene los datos a analizar y colocarlo en la carpeta `data/dataset/`.

### Descripción del Dataset
Este dataset contiene información sobre mensajes de chats mas tops de streamers de Twitch. Cada registro representa un mensaje de chat con información completa sobre el autor, canal, contenido y contexto del stream. La estructura completa de cada mensaje es por ejempolo del tipo:

```json
{
  "_id": "3c27495c-ccec-499d-930c-64638b96b903",
  "category": "509658",
  "channel": "465131731",
  "author": "707707619",
  "text": "НУ НУ",
  "channelName": "shadowkekw",
  "authorName": "ddzp__",
  "hasEmotes": false,
  "isFirstMessage": false,
  "isMod": false,
  "isSubscriber": true,
  "hasMalware": false,
  "hasUrl": false,
  "hasBadWords": false,
  "hasBadDomain": false,
  "spamingText": false,
  "categoryName": "Just Chatting",
  "isMature": false,
  "language": "ru",
  "streamStartedAt": "2021-11-05T14:03:26Z",
  "streamTitle": "Показиваю квартиру гайки и делаю чупики",
  "streamId": "43719739356",
  "viewerCount": 7414,
  "createAt": {
    "$date": "2021-11-05T14:59:41.071Z"
  }
}
```

### Interpretación de Campos Utilizados
Para este análisis, decidí usar únicamente estos 3 campos del dataset porque son los más relevantes para las transformaciones que quería implementar:

- **channelName**: Nombre del canal de Twitch donde se envió el mensaje (ej: "shadowkekw")
- **language**: Código de idioma del mensaje (ej: "ru", "en", "es", "fr", "de")
- **viewerCount**: Número de viewers que tenía el stream en el momento del mensaje (ej: 7414)

### Decisiones de Interpretación

**Selección de campos:** Elegí estos 3 campos porque me permiten analizar patrones de actividad por idioma y audiencia. Los otros campos como `text` o `authorName` no aportan demasiado.

**Rangos de viewers:** Definí los rangos (0-100, 101-500, 501-1000, 1000+) basándome en una distribución de tamaños de audiencia en Twitch:
- 0-100: Streamers pequeños/emergentes
- 101-500: Streamers con audiencia establecida
- 501-1000: Streamers medianos con buena tracción
- 1000+: Streamers grandes y populares

**Criterio de transformaciones:** Las dos transformaciones que elegí (top canales por idioma y top idiomas por rango) me permiten identificar tanto los canales más activos en cada idioma como los idiomas más populares según el tamaño de audiencia, lo cual da una visión completa de los tops streamers de Twitch.

### Campos Adicionales Disponibles
El dataset contiene información adicional que no se utiliza en este análisis pero está disponible para futuras extensiones:
- **text**: Contenido del mensaje de chat
- **authorName**: Nombre del usuario que envió el mensaje
- **categoryName**: Categoría del stream (ej: "Just Chatting", "Games")
- **streamTitle**: Título del stream
- **hasEmotes**, **isSubscriber**, **isMod**: Metadatos del mensaje y usuario
- **streamStartedAt**, **createAt**: Timestamps del stream y mensaje

## Instrucciones de Ejecución

### Compilación
```bash
cargo build --release
```

### Ejecución
```bash
cargo run --release <ruta_archivo> <numero_threads> [tipo_analisis]
```

### Parámetros
- **ruta_archivo**: Ruta al archivo JSON con los datos de Twitch
- **numero_threads**: Número de threads para procesamiento paralelo (recomendado: número de cores del CPU)
- **tipo_analisis** (opcional):
  - `channels`: Solo análisis de top canales por idioma
  - `languages`: Solo análisis de top idiomas por rango de viewers  
  - `both`: Ambos análisis (por defecto)

### Ejemplos de Uso
```bash
# Ejecutar ambos análisis con 4 threads
cargo run --release data/dataset/10M_Messages.json 4


# Solo top canales por idioma con 8 threads
cargo run --release data/dataset/10M_Messages.json 8 top_channels

# Solo top idiomas por rango con 2 threads  
cargo run --release data/dataset/10M_Messages.json 2 top_languages
```

### Script de Benchmarking

El proyecto incluye un script automatizado para medir la performance del sistema:

```bash
# Ejecutar benchmarks automáticos
./benchmark.sh data/dataset/10M_Messages.json
```

**¿Qué hace el script?**
- Compila el proyecto en modo release
- Ejecuta el análisis con 2, 4 y 8 threads 
- Realiza 3 corridas por configuración para obtener promedios
- Mide tiempo de ejecución, uso de memoria y CPU
- Guarda todos los resultados en `benchmark_results.txt`

**Requisitos:**
- Dataset descargado en la ruta especificada
- Permisos de ejecución: `chmod +x benchmark.sh`

## Dependencias

El proyecto utiliza las siguientes dependencias externas:

| Crate | Versión | Propósito |
|-------|---------|-----------|
| **[serde](https://crates.io/crates/serde)** | 1.0 | Serialización y deserialización de estructuras de datos. Usado para convertir los mensajes JSON del dataset a estructuras Rust |
| **[serde_json](https://crates.io/crates/serde_json)** | 1.0 | Parser específico de JSON. Maneja el parsing de los mensajes de chat desde el archivo JSON |
| **[rayon](https://crates.io/crates/rayon)** | 1.8.0 | Paralelización automática con work-stealing. Proporciona el paralelismo Fork-Join para procesar chunks concurrentemente |
| **[tempfile](https://crates.io/crates/tempfile)** | 3.22.0 | Creación de archivos temporales para testing. Usado en los tests para generar datasets de prueba |

## Transformaciones Implementadas

### 1. Top 3 Canales por Idioma
**Objetivo:** Identificar los 3 canales con mayor actividad (más mensajes) para cada idioma.

**Algoritmo:**
- Agrupa mensajes por idioma y canal
- Cuenta mensajes por cada combinación idioma-canal
- Ordena canales por cantidad de mensajes (descendente)
- Selecciona top 3 por idioma

**Formato de Resultado:**
```
=== TOP 3 CHANNELS BY LANGUAGE ===
Language: en
  1. channel_name_1: 1500 messages
  2. channel_name_2: 1200 messages  
  3. channel_name_3: 950 messages
```

### 2. Top 5 Idiomas por Rango de Viewers
**Objetivo:** Encontrar los 5 idiomas más populares en cada rango de audiencia.

**Rangos de Viewers:**
- **0-100**: Streamers pequeños
- **101-500**: Streamers medianos
- **501-1000**: Streamers grandes
- **1000+**: Streamers muy grandes

**Algoritmo:**
- Clasifica cada mensaje según el viewer count en rangos
- Agrupa por rango de viewers y idioma
- Cuenta mensajes por combinación rango-idioma
- Ordena idiomas por cantidad de mensajes (descendente)
- Selecciona top 5 por rango

**Formato de Resultado:**
```
=== TOP 5 LANGUAGES BY VIEWER RANGE ===
Viewer Range: 0-100
  1. en: 2500 messages
  2. es: 1800 messages
  3. fr: 1200 messages
  4. de: 900 messages
  5. pt: 600 messages
```

## Análisis de Performance

### Optimizaciones Implementadas
1. **Procesamiento por Chunks**: División inteligente del archivo respetando límites de objetos JSON
2. **Memoria Constante**: ~3MB de uso independientemente del tamaño del dataset
3. **Agregación en Tiempo Real**: Procesa mensajes mediante callbacks sin almacenamiento intermedio
4. **Streaming Processing**: Procesa datos en tiempo real sin cargar todo en memoria


### Resultados de Benchmarks

Benchmarks ejecutados en dataset de **10.3M mensajes** (~8GB):

#### Resumen de Performance por Configuración

| Threads | Tiempo Promedio (seg) | Memoria Máxima (KB) | CPU Utilizada | Speedup |
|---------|----------------------|-------------------|---------------|---------|
| 2       | 27.90                | 2,956             | 196%          | 1.0x    |
| 4       | 16.45                | 3,583             | 387%          | 1.7x    |
| 8       | 9.78                 | 3,676             | 753%          | 2.9x    |

#### Análisis Detallado

**Configuración con 2 Threads:**
- Tiempo: 28.29s, 28.14s, 27.27s (promedio: 27.90s)
- Memoria: 2,988KB, 2,828KB, 3,052KB (promedio: 2,956KB)
- CPU: Utilización constante del 196%

**Configuración con 4 Threads:**
- Tiempo: 16.24s, 16.48s, 16.63s (promedio: 16.45s)
- Memoria: 3,668KB, 3,344KB, 3,736KB (promedio: 3,583KB)  
- CPU: Utilización promedio del 387%

**Configuración con 8 Threads:**
- Tiempo: 9.74s, 9.86s, 9.73s (promedio: 9.78s)
- Memoria: 3,760KB, 3,688KB, 3,580KB (promedio: 3,676KB)
- CPU: Utilización promedio del 753%

### Observaciones de Escalabilidad

1. **Escalabilidad Sub-lineal**: El speedup no es perfectamente lineal debido a:
   - Overhead de sincronización entre threads
   - Contención en estructuras de datos compartidas (`Arc<Mutex<HashMap>>`)
   - Limitaciones de I/O del disco

2. **Uso de Memoria Constante**: La memoria se mantiene bajo control (~3MB) independientemente del número de threads, confirmando la eficacia del streaming processing.

3. **Utilización de CPU**: Se observa un aprovechamiento eficiente de los cores disponibles, llegando al 753% con 8 threads.

## Aspectos Técnicos

### Concurrencia
- **Rayon**: Para paralelización automática .
- **Thread Safety**: Estructuras `Arc<Mutex<T>>` para compartir estado
- **Progress Tracking**: `AtomicUsize` para conteo thread-safe

### Manejo de Errores
- **CustomError**: Enum unificado para todos los tipos de errores
- **Error Propagation**: Uso de `Result<T, CustomError>` en toda la aplicación
- **Graceful Degradation**: Continúa procesando ante JSONs inválidos

### Estructura del Proyecto
```
.
├── Cargo.toml               # Configuración del proyecto
├── Cargo.lock              # Lockfile de dependencias
├── README.md               # Documentación del proyecto
├── TP1COncu.pdf            # Enunciado del trabajo práctico
├── benchmark.sh            # Script de benchmarking
├── benchmark_results.txt   # Resultados de performance
├── data/
│   ├── dataset/
│   │   └── 10M_Messages.json # Dataset principal de Twitch
│   └── test_data/           # Archivos de prueba pequeños
│       ├── test_10357.json
│       ├── test_10458.json
│       └── ...              # Múltiples archivos de test
├── src/
│   ├── lib.rs                    # Declaraciones de módulos
│   ├── main.rs                   # Punto de entrada
│   ├── parser.rs                 # Análisis y chunking de archivos
│   ├── transformations.rs        # Lógica de transformaciones
│   ├── custom_error.rs          # Manejo de errores
│   ├── chunk_info.rs            # Metadatos de chunks
│   ├── chat_message.rs          # Estructura de mensajes
│   ├── channel_message_count.rs # Conteo por canal
│   ├── language_message_count.rs# Conteo por idioma
│   ├── top_channels_result.rs   # Resultado top canales
│   ├── top_languages_result.rs  # Resultado top idiomas
│   └── streaming_aggregators.rs # Agregadores thread-safe
├── tests/
│   ├── integration_test.rs      # Tests de integración
│   ├── parser_tests.rs          # Tests del parser
│   ├── transformations_test.rs  # Tests de transformaciones
│   ├── streaming_test.rs        # Tests de streaming
│   ├── models_test.rs           # Tests de modelos de datos
│   └── errors_tests.rs          # Tests de manejo de errores
```

## Tests Automatizados

El proyecto incluye un conjunto de tests automatizados que validan la correcta implementación de las herramientas de concurrencia:

### Ejecución de Tests
```bash
# Ejecutar todos los tests
cargo test

# Ejecutar tests con output detallado
cargo test -- --nocapture
```

### Tipos de Tests Implementados

- **Tests de Integración** (`integration_test.rs`): Validan el flujo completo end-to-end con archivos reales
- **Tests del Parser** (`parser_tests.rs`): Verifican el parsing y chunking correcto de archivos JSON
- **Tests de Transformaciones** (`transformations_test.rs`): Prueban funciones individuales y casos limite
- **Tests de Streaming** (`streaming_test.rs`): Comprueban el procesamiento concurrente sin deadlocks
- **Tests de Modelos** (`models_test.rs`): Verifican las estructuras de datos y serialización
- **Tests de Errores** (`errors_tests.rs`): Validan el manejo de casos de error

### Validación de Concurrencia

Los tests incluyen validaciones específicas para:
- Asegurar ausencia de deadlocks
- Thread-safety de las estructuras compartidas (`Arc<Mutex<HashMap>>`)
- Correctitud de resultados con múltiples threads
- Manejo de errores en contexto concurrente

## Documentación del Código

### Generar Documentación
```bash
# Generar y abrir documentación en el navegador
cargo doc --open

# Solo generar documentación
cargo doc
```

## Conclusiones

Durante el desarrollo de este trabajo practico se pudo implementar un sistema de procesamiento concurrente basado en el modelo Fork-Join para analizar grandes volúmenes de datos de nuestro dataset de Twitch. Con los resultados obtenidos, vemos que al paralelizar el procesamiento utilizando múltiples threads, se logra una mejora significativa en el tiempo de ejecución, aunque con rendimientos decrecientes debido a la contención en las estructuras de datos compartidas y el overhead de sincronización.

### Análisis de Rendimiento y Escalabilidad

Los benchmarks realizados demuestran que el enfoque de paralelización funciona bien en la práctica. Al comparar la ejecución con 2 threads versus 8 threads, se observa un speedup de 2.9x, lo cual es una mejora significativa.

Un aspecto destacable es que el consumo de memoria se mantuvo constante en ~3MB independientemente del número de threads utilizados. Esto confirma que la estrategia de streaming processing cumple su objetivo de procesar datasets grandes sin requerir cargar todo en memoria.

La escalabilidad, mostró mejoras consistentes hasta 8 threads. 

### Decisiones Arquitecturales Efectivas

La implementación de streaming processing resultó clave para manejar el dataset de 8GB. En lugar de cargar todo en memoria y luego procesarlo, el enfoque de procesar chunk por chunk permite trabajar con archivos arbitrariamente grandes.

El uso de `Arc<Mutex<HashMap>>` para garantizar thread-safety funcionó correctamente sin deadlocks.

### Limitaciones Encontradas

El principal problema identificado es la contención en los mutex cuando múltiples threads intentan actualizar los HashMaps compartidos simultáneamente. Esto se vuelve más evidente conforme aumenta el número de threads.

El I/O del disco también representa una limitación, especialmente porque el procesamiento de JSON es relativamente rápido comparado con la lectura secuencial del archivo. Finalmente, el overhead de crear y sincronizar threads se puede notar con configuraciones de muchos threads.

### Posibles mejoras

**Para reducir la contención de locks:**
- Reemplazar `Arc<Mutex<HashMap>>` con `DashMap`, que es una implementación lock-free optimizada para acceso concurrente
- Implementar estructuras de datos lock-free usando el crate `crossbeam`
- Considerar un approach de thread-local aggregation seguido de merge final

**Para optimizar I/O:**
- Implementar lectura asíncrona usando `tokio` para overlappear I/O con procesamiento
- Evaluar el uso de memory-mapped files con `memmap2` para archivos muy grandes
- Implementar compresión para reducir el volumen de datos a leer



