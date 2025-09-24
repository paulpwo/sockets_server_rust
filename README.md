# Rush Server

## Descripción

Servidor WebSocket implementado en Rust usando Axum y Socket.IO, con soporte para broadcasting de mensajes, métricas de salud y monitoreo de sistema.

## Requisitos Previos

- Rust 1.70 o superior
- Node.js 14+ (para ejecutar pruebas con test_socketio_client.js)
- Docker (opcional, para contenedorización)

## Instalación de Rust

Si no tienes Rust instalado, ejecuta:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verifica la instalación:

```bash
rustc --version
cargo --version
```

## Compilación del Proyecto

Navega al directorio del proyecto:

```bash
cd rush_server
```

Compila en modo debug:

```bash
cargo build
```

Para modo release (optimizado):

```bash
cargo build --release
```

## Ejecución Local

Ejecuta el servidor:

```bash
cargo run
```

El servidor estará disponible en http://localhost:3030

Endpoints disponibles:
- `/ws`: Conexión WebSocket
- `/health`: Estado de salud del servidor
- `/metrics`: Métricas en formato JSON
- `/`: Información básica del servicio
- `/SendWebsocketEvent`: Enviar eventos a clientes conectados (POST)

## Uso de Docker

Construye la imagen:

```bash
docker build -t rush-server .
```

Ejecuta el contenedor:

```bash
docker run -p 3030:3030 rush-server
```

## Pruebas de Carga

El proyecto incluye un sistema completo de pruebas de carga usando Socket.IO para validar el rendimiento del servidor WebSocket.

### Configuración de las Pruebas

Las pruebas están ubicadas en el directorio `loadtest/` e incluyen:

- **package.json**: Configuración de dependencias y scripts de prueba
- **test_socketio_client.js**: Cliente de prueba Socket.IO con métricas avanzadas

### Instalación de Dependencias

Navega al directorio de pruebas e instala las dependencias:

```bash
cd loadtest
npm install
```

### Ejecución de Pruebas

#### Prueba Rápida (Configuración por Defecto)

```bash
npm test
```

Esto ejecuta: 500 conexiones simultáneas durante 10 segundos contra `http://localhost:3030`

#### Pruebas Personalizadas

```bash
node test_socketio_client.js -url=<URL> -connections=<NUM> -duration=<SEC> [-rate=<NUM>]
```

**Parámetros obligatorios:**
- `-url=<URL>`: URL del servidor Socket.IO (ej: http://localhost:3030)
- `-connections=<NUM>`: Número de conexiones simultáneas
- `-duration=<SEC>`: Duración de la prueba en segundos

**Parámetros opcionales:**
- `-rate=<NUM>`: Mensajes por segundo por conexión (default: 1)

#### Ejemplos de Uso

```bash
# Prueba básica con 100 conexiones por 30 segundos
node test_socketio_client.js -url=http://localhost:3030 -connections=100 -duration=30

# Prueba intensiva con 1000 conexiones y 2 mensajes por segundo
node test_socketio_client.js -url=http://localhost:3030 -connections=1000 -duration=60 -rate=2

# Prueba de estrés con 2000 conexiones
node test_socketio_client.js -url=http://localhost:3030 -connections=2000 -duration=120
```

### Métricas Reportadas

El sistema de pruebas proporciona métricas en tiempo real:

- **Conexiones activas**: Número de conexiones Socket.IO establecidas
- **Mensajes enviados**: Total de mensajes transmitidos al servidor
- **Mensajes recibidos**: Total de mensajes recibidos del servidor
- **Errores**: Número de errores de conexión o transmisión
- **Throughput**: Mensajes por segundo (enviados y recibidos)
- **Tasa de error**: Porcentaje de errores sobre el total de operaciones

### Interpretación de Resultados

```
⏱️  1:30 | 🔗 500 | 📤 750 | 📥 750 | ❌ 0 | 📊 8.3 msg/s
```

- `1:30`: Tiempo transcurrido (minutos:segundos)
- `🔗 500`: Conexiones activas
- `📤 750`: Mensajes enviados
- `📥 750`: Mensajes recibidos
- `❌ 0`: Errores
- `📊 8.3 msg/s`: Throughput actual

### Requisitos del Sistema

Para ejecutar las pruebas necesitas:

- Node.js 14+ 
- Servidor Rust ejecutándose en el puerto especificado
- Suficiente memoria y descriptores de archivo para las conexiones simultáneas

### Recomendaciones de Prueba

1. **Pruebas graduales**: Comienza con pocas conexiones y aumenta gradualmente
2. **Monitoreo del servidor**: Observa el uso de CPU y memoria del servidor Rust
3. **Límites del sistema**: Verifica los límites de descriptores de archivo (`ulimit -n`)
4. **Red local**: Para mejores resultados, ejecuta las pruebas en la misma máquina o red local

## Notas sobre Optimizaciones para Kubernetes

- **Imagen Docker**: El Dockerfile utiliza multi-stage build para reducir el tamaño de la imagen final.
- **Usuario no-root**: El contenedor ejecuta como usuario no-privilegiado para mayor seguridad.
- **Resource Limits**: En Kubernetes, configura límites de CPU y memoria basados en el uso observado (ej. requests: 100m CPU, 128Mi RAM; limits: 500m CPU, 512Mi RAM).
- **Health Checks**: Usa el endpoint `/health` para readiness y liveness probes.
- **Scaling**: El servidor soporta múltiples conexiones concurrentes; escala horizontalmente según la carga.
- **Configuración**: Considera usar ConfigMaps para variables de entorno como el puerto.

