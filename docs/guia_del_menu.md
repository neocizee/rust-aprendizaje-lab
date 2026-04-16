# Manual del Usuario: Menú Interactivo

Bienvenido a la versión 2.0 de la interfaz del Laboratorio de Fibonacci.

## 🕹️ Funciones del Menú

### 1. Generar nueva secuencia
Esta es la función principal. 
- Te pedirá cuánto números quieres.
- Guardará el resultado en un archivo de texto en la carpeta `temp/`.
- Creará un reporte de éxito mostrando el tiempo y la memoria exacta utilizada.

### 2. Eliminar un resultado
Si quieres limpiar espacio, puedes borrar los cálculos uno por uno.
- Puedes ingresar el número (ID) del archivo (ej: `1`) o el nombre completo (ej: `1.txt`).
- Borra tanto el archivo de datos como el reporte histórico.

### 3. Vista previa rápida
Útil para debugging rápido.
- No guarda nada en disco.
- Muestra el resultado directamente en los colores de la terminal.
- Está limitado a 50 elementos para que la consola no se sature.

### 4. Ver análisis histórico
Muestra una lista de todos los archivos generados y lo que dice su "Caja Negra" (el log de recursos). Podrás ver qué archivos consumieron más tiempo o memoria.

## 📂 Directorios de Importancia
- `temp/`: Tus resultados están aquí.
- `logs/`: Los reportes técnicos están aquí.
