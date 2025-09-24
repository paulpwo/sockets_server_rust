const { io } = require('socket.io-client');

// Configurar argumentos de línea de comandos
const args = process.argv.slice(2);
let CONNECTIONS, DURATION, MESSAGE_RATE, SERVER_URL;

// Función para mostrar ayuda
function showHelp() {
    console.log('🚀 Prueba de Carga Socket.IO');
    console.log('');
    console.log('Uso: node test_socketio_client.js -url=<URL> -connections=<NUM> -duration=<SEC> [-rate=<NUM>]');
    console.log('');
    console.log('Parámetros obligatorios:');
    console.log('  -url=<URL>        URL del servidor Socket.IO (ej: http://localhost:3030)');
    console.log('  -connections=<NUM> Número de conexiones simultáneas');
    console.log('  -duration=<SEC>    Duración de la prueba en segundos');
    console.log('');
    console.log('Parámetros opcionales:');
    console.log('  -rate=<NUM>        Mensajes por segundo por conexión (default: 1)');
    console.log('');
    console.log('Ejemplos:');
    console.log('  node test_socketio_client.js -url=http://localhost:3030 -connections=500 -duration=30');
    console.log('  node test_socketio_client.js -url=http://localhost:3030 -connections=1000 -duration=60 -rate=2');
    process.exit(1);
}

// Parsear argumentos
for (let arg of args) {
    if (arg.startsWith('-url=')) {
        SERVER_URL = arg.substring(5);
    } else if (arg.startsWith('-connections=')) {
        CONNECTIONS = parseInt(arg.substring(13));
    } else if (arg.startsWith('-duration=')) {
        DURATION = parseInt(arg.substring(10));
    } else if (arg.startsWith('-rate=')) {
        MESSAGE_RATE = parseInt(arg.substring(6));
    } else if (arg === '-h' || arg === '--help') {
        showHelp();
    }
}

// Validar parámetros obligatorios
if (!SERVER_URL) {
    console.log('❌ Error: URL del servidor es obligatoria');
    showHelp();
}

if (!CONNECTIONS || CONNECTIONS <= 0) {
    console.log('❌ Error: Número de conexiones es obligatorio y debe ser mayor a 0');
    showHelp();
}

if (!DURATION || DURATION <= 0) {
    console.log('❌ Error: Duración es obligatoria y debe ser mayor a 0');
    showHelp();
}

// Valores por defecto para parámetros opcionales
MESSAGE_RATE = MESSAGE_RATE || 1;

let stats = {
    connected: 0,
    messagesSent: 0,
    messagesReceived: 0,
    errors: 0,
    startTime: Date.now()
};

const connections = [];

console.log('🚀 Iniciando prueba de carga Socket.IO');
console.log(`📡 Servidor: ${SERVER_URL}`);
console.log(`🔗 Conexiones: ${CONNECTIONS}`);
console.log(`⏱️  Duración: ${DURATION} segundos`);
console.log(`📨 Rate: ${MESSAGE_RATE} msg/seg por conexión`);
console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');

// Crear conexiones Socket.IO
for (let i = 0; i < CONNECTIONS; i++) {
    const socket = io(SERVER_URL, {
        transports: ['websocket'], // Forzar WebSocket transport para mejor performance
        upgrade: true,
        rememberUpgrade: true,
        timeout: 5000
    });

    socket.on('connect', () => {
        stats.connected++;
        console.log(`✅ Conexión Socket.IO ${stats.connected}/${CONNECTIONS} establecida`);

        // Enviar mensajes periódicamente
        const interval = setInterval(() => {
            if (socket.connected) {
                const message = {
                    data: `Mensaje desde conexión ${i} - ${Date.now()}`,
                    timestamp: Date.now()
                };

                try {
                    socket.emit('message', message);
                    stats.messagesSent++;
                } catch (error) {
                    stats.errors++;
                    clearInterval(interval);
                }
            } else {
                clearInterval(interval);
            }
        }, 1000 / MESSAGE_RATE);

        // Limpiar interval al desconectar
        socket.on('disconnect', () => {
            clearInterval(interval);
        });
    });

    socket.on('message', (data) => {
        try {
            stats.messagesReceived++;
        } catch (error) {
            stats.errors++;
        }
    });

    socket.on('connect_error', (error) => {
        stats.errors++;
        console.error(`❌ Error de conexión ${i}:`, error.message);
    });

    socket.on('disconnect', (reason) => {
        stats.connected--;
        console.log(`🔌 Conexión Socket.IO ${i} desconectada: ${reason}`);
    });

    connections.push(socket);
}

// Mostrar estadísticas cada segundo
const statsInterval = setInterval(() => {
    const elapsed = (Date.now() - stats.startTime) / 1000;
    const throughput = stats.messagesSent / elapsed;
    const receivedThroughput = stats.messagesReceived / elapsed;
    
    console.log(`⏱️  ${Math.floor(elapsed / 60)}:${String(Math.floor(elapsed % 60)).padStart(2, '0')} | 🔗 ${stats.connected} | 📤 ${stats.messagesSent} | 📥 ${stats.messagesReceived} | ❌ ${stats.errors} | 📊 ${throughput.toFixed(1)} msg/s`);
}, 1000);

// Detener la prueba después de la duración especificada
setTimeout(() => {
    clearInterval(statsInterval);

    // Cerrar todas las conexiones Socket.IO
    connections.forEach(socket => {
        if (socket.connected) {
            socket.disconnect();
        }
    });

    const totalTime = (Date.now() - stats.startTime) / 1000;
    const avgThroughput = stats.messagesSent / totalTime;
    const receivedThroughput = stats.messagesReceived / totalTime;
    const errorRate = (stats.errors / (stats.messagesSent + stats.messagesReceived)) * 100;

    console.log('✅ Prueba completada');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('📊 ESTADÍSTICAS FINALES');
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log(`⏱️  Duración total: ${totalTime.toFixed(2)} segundos`);
    console.log(`🔗 Conexiones configuradas: ${CONNECTIONS}`);
    console.log(`📤 Mensajes enviados: ${stats.messagesSent}`);
    console.log(`📥 Mensajes recibidos: ${stats.messagesReceived}`);
    console.log(`❌ Errores: ${stats.errors}`);
    console.log(`📊 Throughput promedio: ${avgThroughput.toFixed(2)} msg/seg`);
    console.log(`📈 Throughput recibido: ${receivedThroughput.toFixed(2)} msg/seg`);
    console.log(`⚠️  Tasa de error: ${errorRate.toFixed(2)}%`);
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('🎉 ¡Socket.IO compatible - Servidor Rust actualizado!');

    process.exit(0);
}, DURATION * 1000);

// Manejo de señales para cierre graceful
process.on('SIGINT', () => {
    console.log('\n🛑 Deteniendo prueba...');
    clearInterval(statsInterval);
    connections.forEach(socket => {
        if (socket.connected) {
            socket.disconnect();
        }
    });
    process.exit(0);
});
