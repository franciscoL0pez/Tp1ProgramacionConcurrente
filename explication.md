# Fork-Join vs Threads - Explicación del TP

## ¿Qué diferencia hay entre Fork-Join y Threads normales?

### **Threads normales** 🧵
Son la **herramienta básica** - como tener empleados trabajando

```rust
// Crear threads para tareas diferentes
let handle1 = thread::spawn(|| descargar_archivo());
let handle2 = thread::spawn(|| procesar_base_datos());
let handle3 = thread::spawn(|| enviar_emails());

// Cada uno hace algo completamente diferente
```

### **Fork-Join** 🌳
Es un **patrón/estrategia** - cómo organizar el trabajo

```rust
// Tomar UNA tarea grande y dividirla
let dataset_completo = cargar_datos();

// FORK: dividir en pedazos
let chunk1 = dataset[0..1000];
let chunk2 = dataset[1000..2000]; 
let chunk3 = dataset[2000..3000];

// Todos hacen LA MISMA tarea, pero con datos diferentes
let t1 = thread::spawn(|| procesar_chunk(chunk1));
let t2 = thread::spawn(|| procesar_chunk(chunk2));
let t3 = thread::spawn(|| procesar_chunk(chunk3));

// JOIN: combinar resultados
let resultado_final = combinar(t1.join(), t2.join(), t3.join());
```

## Diferencia clave:

### **Threads normales:**
- Tareas **diferentes**
- Trabajos **independientes**
- No necesariamente se combinan

### **Fork-Join:**
- **Misma tarea**, datos diferentes
- Trabajo **coordinado** hacia un objetivo común
- Siempre se **combinan** los resultados

**Fork-Join USA threads, pero de manera específica.**

---

## Ejemplo visual del Fork-Join:

```
Dataset completo (139 elementos)
         ↓ FORK
    ┌─────────┬─────────┬─────────┐
Thread 1   Thread 2   Thread 3   Thread 4
(0-34)     (35-69)    (70-104)   (105-138)
    │         │          │          │
   ↓ procesa ↓ procesa  ↓ procesa  ↓ procesa
    │         │          │          │
Resultado1 Resultado2 Resultado3 Resultado4
    └─────────┬─────────┬─────────┘
              ↓ JOIN
         Resultado Final
```

## Con Rayon es automático:
- **Rayon decide** cuántos threads usar
- **Rayon divide** el trabajo automáticamente
- **Rayon combina** los resultados automáticamente

Tu solo defines:
1. **Qué hacer** con cada elemento (`map`)
2. **Cómo combinar** resultados (`reduce`)

---

## Dataset del TP: Chats de Twitch (8GB)

### Transformaciones elegidas:

#### **Transformación 1: Análisis por idioma**
- Contar mensajes por idioma (`language`)
- Mostrar los idiomas más populares ordenados

#### **Transformación 2: Canales con mayor viewer count promedio**
- Calcular promedio de `viewerCount` por `channelName`
- Mostrar los top 10 canales ordenados por promedio

### Ejemplo con datos pequeños:

**Dataset mini:**
```json
[
  {"channelName": "streamer1", "language": "en", "viewerCount": 1000},
  {"channelName": "streamer1", "language": "en", "viewerCount": 1200},
  {"channelName": "streamer2", "language": "es", "viewerCount": 500}
]
```

**Transformación 1: Idiomas**
- Input: Los 3 mensajes
- Proceso: Contar por `language`
- Output: `en: 2 mensajes, es: 1 mensaje`

**Transformación 2: Promedio viewers**
- Input: Los 3 mensajes
- Proceso: Agrupar por `channelName`, calcular promedio
- Output: `streamer1: 1100.0 promedio, streamer2: 500.0 promedio`

---

## Pasos del TP:

1. **Planificación técnica** 📋 - Definir fork-join strategy
2. **Diseño de la aplicación** 🏗️ - Structs y módulos
3. **Implementación base** 💻 - Parsear JSON y lógica básica
4. **Implementación concurrente** ⚡ - Agregar Rayon
5. **Testing y optimización** 🧪 - Medir performance con 1,2,4 CPUs
6. **Documentación** 📝 - README.md e informe