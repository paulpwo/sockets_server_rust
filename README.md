# Rush Server

## Descripción

Servidor WebSocket implementado en Rust usando Axum, con soporte para broadcasting de mensajes, métricas de salud y monitoreo.

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

## Pruebas

Para probar el servidor con el cliente Socket.IO:

Asegúrate de que el servidor esté ejecutándose, luego ejecuta:

```bash
node ../test_socketio_client.js
```

Esto ejecutará pruebas de conexión y envío de mensajes.

## Notas sobre Optimizaciones para Kubernetes

- **Imagen Docker**: El Dockerfile utiliza multi-stage build para reducir el tamaño de la imagen final.
- **Usuario no-root**: El contenedor ejecuta como usuario no-privilegiado para mayor seguridad.
- **Resource Limits**: En Kubernetes, configura límites de CPU y memoria basados en el uso observado (ej. requests: 100m CPU, 128Mi RAM; limits: 500m CPU, 512Mi RAM).
- **Health Checks**: Usa el endpoint `/health` para readiness y liveness probes.
- **Scaling**: El servidor soporta múltiples conexiones concurrentes; escala horizontalmente según la carga.
- **Configuración**: Considera usar ConfigMaps para variables de entorno como el puerto.

## Contribución

[Instrucciones si aplica]

## Licencia

[Licencia si aplica]