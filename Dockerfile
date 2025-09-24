# Socket Server - A WebSocket server implementation in Rust
# Copyright (C) 2024 Paul Werner (paulpwo)
# Licensed under GNU Affero General Public License v3.0
# Repository: https://github.com/paulpwo/sockets_server_rust

# Multi-stage build para optimizar el tamaño de la imagen
# Etapa de construcción
FROM rust:alpine AS builder

WORKDIR /app

# Instalar musl-dev para linking
RUN apk add --no-cache musl-dev

# Copiar archivos de dependencias primero para aprovechar cache de Docker
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Compilar en modo release
RUN cargo build --release

# Etapa de runtime
FROM alpine:latest

# Instalar ca-certificates para posibles conexiones HTTPS (opcional)
RUN apk --no-cache add ca-certificates

# Crear usuario no-root para seguridad
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

# Copiar el binario compilado desde la etapa de construcción
COPY --from=builder /app/target/release/socket_server /usr/local/bin/socket_server

# Cambiar a usuario no-root
USER appuser

# Exponer puerto 3030
EXPOSE 3030

# Comando para ejecutar el servidor
CMD ["/usr/local/bin/socket_server"]
