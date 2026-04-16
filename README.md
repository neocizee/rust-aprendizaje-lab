# Rust Aprendizaje Lab 🧪

## Descripción / Objetivo
Este repositorio es un laboratorio de aprendizaje dedicado a explorar y dominar el lenguaje de programación Rust. El objetivo principal es implementar algoritmos complejos, gestionar sistemas de archivos y entender profundamente la gestión de recursos (memoria y CPU) en un entorno de alto rendimiento.

## Funcionalidades / Áreas de Aprendizaje / Contexto
- **Algoritmia Avanzada**: Implementación de la secuencia de Fibonacci utilizando el método de duplicación rápida con complejidad $O(\log n)$.
- **Gestión de Memoria**: Monitorización en tiempo real del uso de memoria utilizando la librería `sysinfo`.
- **Persistencia de Datos**: Sistema de gestión de archivos para guardar, listar y eliminar resultados de cálculos extensos.
- **Arquitectura de Software**: Organización modular siguiendo las mejores prácticas de Rust (`main.rs`, `lib.rs`, módulos).
- **Tratamiento de Grandes Números**: Uso de aritmética de precisión arbitraria para manejar números de Fibonacci masivos que superan los límites de los tipos nativos de 64 bits.

## Stack / Librerías
Este proyecto utiliza las siguientes tecnologías y dependencias:

- **Lenguaje**: Rust (Edición 2024)
- **num-bigint (v0.4)**: Para cálculos con números enteros de tamaño arbitrario.
- **num-traits (v0.2)**: Rasgos numéricos genéricos.
- **sysinfo (v0.30)**: Monitorización de RAM.
- **clap (v4.4)**: Para gestión de flags de CPU y RAM.
- **rayon (v1.10)**: Para control de hilos de CPU.

## Ejecución con Límites de Recursos
El sistema ahora opera con **límites de seguridad hardcodeados por defecto** (2 núcleos CPU / 2048 MB RAM) para evitar bloqueos del sistema. Puedes sobrescribir estos valores usando las flags:

### Comando de ejemplo (Sobrescribiendo a 1 núcleo y 4GB de RAM):
```bash
cargo run -- --cpu 1 --ram 4096
```
*(Nota: Si no se pasan argumentos, el programa se limitará automáticamente a los valores seguros por defecto).*

## Documentación Detallada
Para más detalles sobre implementaciones específicas, consulta el directorio `docs/`:
- [Algoritmos de Fibonacci](docs/fibonacci_algoritmos.md)
- [Guía del Menú Principal](docs/guia_del_menu.md)

## Licencia
*Este software solo se permite utilizar, modificar y distribuir con permiso explícito y previo del autor. No se permite utilizar, modificar o distribuir este software sin el permiso concedido por el autor.*
