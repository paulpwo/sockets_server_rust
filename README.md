# Rust Socket.IO MicroServ

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Socket.io](https://img.shields.io/badge/Socket.io-black?style=for-the-badge&logo=socket.io&badgeColor=010101)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/paulpwo/sockets_server_rust)
[![Docker Image](https://img.shields.io/badge/docker-ready-blue.svg)](https://github.com/paulpwo/sockets_server_rust)

🚀 **Microservicio de alta performance en Rust con Socket.IO**

*Diseñado para manejar miles de conexiones concurrentes con broadcasting en tiempo real*

</div>

---

## 📋 Tabla de Contenidos

- [🚀 Inicio Rápido](#-inicio-rápido)
- [✨ Características](#-características-principales)
- [🛠️ Stack Tecnológico](#️-stack-tecnológico)
- [📦 Instalación](#-instalación)
- [🐳 Docker](#-docker)
- [🧪 Pruebas de Carga](#-pruebas-de-carga)
- [⚙️ Configuración](#️-configuración)
- [🤝 Contribuciones](#-contribuciones)
- [📄 Licencia](#-licencia)

---

## 🚀 Inicio Rápido

```bash
# Clona el repositorio
git clone https://github.com/paulpwo/sockets_server_rust.git
cd rust-socketio-microserv

# Compila y ejecuta
cargo run --release

# El servidor estará disponible en http://localhost:3000
```

---

## 📖 Descripción

Microservicio WebSocket de alto rendimiento implementado en Rust usando Axum y Socket.IO. Diseñado para manejar miles de conexiones concurrentes con broadcasting en tiempo real, métricas de salud integradas y monitoreo completo del sistema.

### ✨ Características Principales

- 🔥 **Alto Rendimiento**: Implementado en Rust para máxima eficiencia
- 🌐 **Socket.IO Compatible**: Soporte completo para clientes Socket.IO
- 📡 **Broadcasting**: Envío de mensajes a múltiples canales simultáneamente
- 📊 **Métricas Integradas**: Endpoints de salud y métricas en tiempo real
- 🐳 **Docker Ready**: Imagen optimizada multi-stage
- ⚡ **Escalable**: Diseñado para miles de conexiones concurrentes
- 🔒 **Seguro**: Licencia AGPL-3.0 y mejores prácticas de seguridad

### 🛠️ Stack Tecnológico

- **Rust** - Lenguaje de sistemas de alto rendimiento
- **Axum** - Framework web moderno y rápido
- **Socket.IO** - Comunicación bidireccional en tiempo real
- **Tokio** - Runtime asíncrono
- **Docker** - Contenedorización

---

## 📦 Instalación

### Requisitos Previos

- Rust 1.70 o superior
- Cargo (incluido con Rust)
- Git

### 🔧 Pasos de Instalación

1. **Clona el repositorio:**
   ```bash
   git clone https://github.com/paulpwo/sockets_server_rust.git
   ```

2. **Navega al directorio del proyecto:**
   ```bash
   cd rust-socketio-microserv
   ```

3. **Compila el proyecto:**
   ```bash
   cargo build --release
   ```

4. **Ejecuta el servidor:**
   ```bash
   cargo run --release
   ```

El servidor se iniciará en `http://localhost:3000` por defecto.

---

## 🐳 Docker

### 🐳 Uso de Docker

**Construye la imagen:**
```bash
docker build -t rust-socketio-microserv .
```

**Ejecuta el contenedor:**
```bash
docker run -p 3000:3000 rust-socketio-microserv
```

**Con variables de entorno:**
```bash
docker run -p 3000:3000 -e PORT=3000 rust-socketio-microserv
```

---

## ⚙️ Configuración

### 🌐 Endpoints Disponibles

| Endpoint | Método | Descripción |
|----------|--------|-------------|
| `/` | GET | Información básica del servicio |
| `/health` | GET | Estado de salud del servidor |
| `/metrics` | GET | Métricas del sistema en JSON |
| `/ws` | WebSocket | Conexión WebSocket principal |
| `/SendWebsocketEvent` | POST | Enviar eventos a clientes conectados |

### 📊 Ejemplo de Uso

**Conectar cliente Socket.IO:**
```javascript
const io = require('socket.io-client');
const socket = io('http://localhost:3000');

socket.on('connect', () => {
    console.log('Conectado al servidor');
    socket.emit('message', 'Hola servidor!');
});

socket.on('broadcast', (data) => {
    console.log('Mensaje recibido:', data);
});
```

**Enviar evento via HTTP:**
```bash
curl -X POST http://localhost:3000/SendWebsocketEvent \
  -H "Content-Type: application/json" \
  -d '{"event": "broadcast", "data": "Mensaje para todos"}'
```

---

---

## 🧪 Pruebas de Carga

### 📋 Descripción del Test

El proyecto incluye un cliente de prueba Socket.IO desarrollado en Node.js que permite realizar pruebas de carga exhaustivas para evaluar el rendimiento del servidor bajo diferentes condiciones de estrés.

### 🚀 Configuración Rápida

**1. Instala las dependencias:**
```bash
cd loadtest
npm install
```

**2. Ejecuta las pruebas:**
```bash
# Prueba básica (100 conexiones, 10 segundos)
npm test

# Prueba personalizada
npm test -- --connections 500 --duration 30 --interval 100
```

### 📊 Parámetros de Configuración

| Parámetro | Descripción | Valor por Defecto |
|-----------|-------------|-------------------|
| `--connections` | Número de conexiones simultáneas | 100 |
| `--duration` | Duración de la prueba (segundos) | 10 |
| `--interval` | Intervalo entre mensajes (ms) | 1000 |
| `--server` | URL del servidor | http://localhost:3000 |

### 📈 Interpretación de Métricas

Durante la ejecución verás métricas en tiempo real:

```
⏱️  1:30 | 🔗 500 | 📤 750 | 📥 750 | ❌ 0 | 📊 8.3 msg/s
```

- `⏱️ 1:30`: Tiempo transcurrido (minutos:segundos)
- `🔗 500`: Conexiones activas
- `📤 750`: Mensajes enviados
- `📥 750`: Mensajes recibidos
- `❌ 0`: Errores de conexión
- `📊 8.3 msg/s`: Throughput actual (mensajes por segundo)

### 🎯 Casos de Uso Recomendados

```bash
# Prueba de estrés básica
npm test -- --connections 1000 --duration 60

# Prueba de latencia
npm test -- --connections 50 --interval 100 --duration 30

# Prueba de resistencia
npm test -- --connections 500 --duration 300 --interval 2000
```

### 💻 Requisitos del Sistema

- **Node.js 14+** 
- **Servidor Rust** ejecutándose en el puerto especificado
- **Memoria suficiente** para las conexiones simultáneas
- **Descriptores de archivo** adecuados (`ulimit -n`)

---

## 🏗️ Arquitectura

### 📐 Diagrama de Componentes

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Load Balancer │    │   Rust Server   │    │   Monitoring    │
│                 │    │                 │    │                 │
│  ┌───────────┐  │    │  ┌───────────┐  │    │  ┌───────────┐  │
│  │  Nginx    │  │◄──►│  │   Axum    │  │◄──►│  │ Metrics   │  │
│  │  HAProxy  │  │    │  │ Framework │  │    │  │ /health   │  │
│  └───────────┘  │    │  └───────────┘  │    │  └───────────┘  │
└─────────────────┘    │  ┌───────────┐  │    └─────────────────┘
                       │  │ Socket.IO │  │
┌─────────────────┐    │  │  Engine   │  │    ┌─────────────────┐
│    Clients      │    │  └───────────┘  │    │   System Info   │
│                 │    │  ┌───────────┐  │    │                 │
│  ┌───────────┐  │◄──►│  │   Tokio   │  │◄──►│  ┌───────────┐  │
│  │ Browser   │  │    │  │  Runtime  │  │    │  │    CPU    │  │
│  │ Node.js   │  │    │  └───────────┘  │    │  │  Memory   │  │
│  │ Mobile    │  │    └─────────────────┘    │  │  Network  │  │
│  └───────────┘  │                           │  └───────────┘  │
└─────────────────┘                           └─────────────────┘
```

### 🔄 Flujo de Datos

1. **Conexión**: Cliente se conecta via Socket.IO
2. **Autenticación**: Validación opcional de credenciales
3. **Broadcasting**: Mensajes distribuidos a canales específicos
4. **Métricas**: Recolección continua de estadísticas
5. **Monitoreo**: Endpoints de salud y métricas disponibles

### ⚡ Características de Rendimiento

- **Conexiones Concurrentes**: Hasta 10,000+ conexiones simultáneas
- **Latencia**: < 1ms para mensajes locales
- **Throughput**: 50,000+ mensajes/segundo
- **Memoria**: ~50MB base + ~1KB por conexión
- **CPU**: Optimizado para múltiples cores

---

## ☸️ Kubernetes & Producción

### 🐳 Optimizaciones para Kubernetes

- **Imagen Docker**: Multi-stage build para reducir el tamaño de la imagen final
- **Usuario no-root**: El contenedor ejecuta como usuario no-privilegiado para mayor seguridad
- **Resource Limits**: Configura límites basados en el uso observado:
  ```yaml
  resources:
    requests:
      cpu: 100m
      memory: 128Mi
    limits:
      cpu: 500m
      memory: 512Mi
  ```
- **Health Checks**: Usa el endpoint `/health` para readiness y liveness probes
- **Scaling**: Soporta escalado horizontal según la carga
- **Configuración**: Usa ConfigMaps para variables de entorno

### 📊 Ejemplo de Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-socketio-microserv
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rust-socketio-microserv
  template:
    metadata:
      labels:
        app: rust-socketio-microserv
    spec:
      containers:
      - name: server
        image: rust-socketio-microserv:latest
        ports:
        - containerPort: 3000
        env:
        - name: PORT
          value: "3000"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          requests:
            cpu: 100m
            memory: 128Mi
          limits:
            cpu: 500m
            memory: 512Mi
```

---

## 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Por favor:

1. Fork el repositorio
2. Crea una rama para tu feature (`git checkout -b feature/nueva-funcionalidad`)
3. Commit tus cambios (`git commit -am 'Agrega nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/nueva-funcionalidad`)
5. Abre un Pull Request

## 📄 Licencia

Este proyecto está licenciado bajo la **GNU Affero General Public License v3.0** - ver el archivo [LICENSE](LICENSE) para más detalles.

## 👨‍💻 Autor

**Paul Werner** - [@paulpwo](https://github.com/paulpwo)

---

⭐ **¡Si te gusta este proyecto, dale una estrella!** ⭐

