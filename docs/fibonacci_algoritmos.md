# Algoritmos de Fibonacci y Funciones

Este documento detalla la lógica implementada para el cálculo de la secuencia de Fibonacci en este laboratorio.

## 1. Algoritmo Fast Doubling ($O(\log n)$)
La función `fibonacci_log_n` implementa el método de **duplicación rápida**. A diferencia de la iteración simple que toma tiempo lineal $O(n)$, este método aprovecha identidades matemáticas para saltar pasos en la secuencia.

### Identidades Matemáticas:
Para calcular $F_{2k}$ y $F_{2k+1}$ a partir de $F_k$ y $F_{k+1}$:
- $F_{2k} = F_k \times (2F_{k+1} - F_k)$
- $F_{2k+1} = F_{k+1}^2 + F_k^2$

### Implementación:
Se utiliza recursividad con división y conquista. Cada paso reduce el problema a la mitad del tamaño, lo que permite calcular el número de Fibonacci un millón en milisegundos.

## 2. Generación Iterativa de Archivos
La función `generar_archivo_fibonacci` utiliza un bucle iterativo tradicional para generar una secuencia completa y guardarla en un archivo `.txt`.

- **Optimización**: Se utiliza `BufWriter` para minimizar las llamadas al sistema de escritura en disco, lo que acelera significativamente la creación de archivos con miles de elementos.
- **Tipos de Datos**: Se usa `num_bigint::BigInt` para que los números puedan crecer indefinidamente sin causar desbordamiento (overflow).

## 3. Registro de Recursos
Cada vez que se genera un archivo, se llama a `registrar_ejecucion`. Esta función:
1. Captura el tiempo transcurrido con `std::time::Instant`.
2. Captura el uso de memoria del proceso actual a través de `sysinfo`.
3. Guarda un reporte detallado en el directorio `logs/`.

## 4. Gestión de Reportes
- `obtener_resumen_reportes`: Escanea el directorio `logs/` y extrae la información de cada cálculo previo para mostrar un historial al usuario.
- `eliminar_archivo_fibonacci`: Limpia tanto el archivo de resultados en `temp/` como su reporte correspondiente en `logs/`.

## 5. Control Técnico de Recursos (Flags)
El programa permite restringir el uso de hardware mediante argumentos de arranque:

- **CPU (`--cpu <N>`)**: Utiliza `rayon` para configurar el pool de hilos global. Si se establece en 1, todas las operaciones internas se forzarán a ejecutarse de forma secuencial en un único hilo.
- **RAM (`--ram <MB>`)**: El motor de generación verifica el uso de memoria cada 100 elementos procesados. Si el proceso de Rust supera el umbral definido, la ejecución se aborta inmediatamente para proteger la estabilidad del sistema, devolviendo un error de "Recursos Excedidos".
