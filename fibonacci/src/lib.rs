pub mod fibonacci;
pub mod interfaz_consola;

/// Re-exportación de las API principales para facilitar su uso desde el binario.
pub use fibonacci::{
    generar_archivo_fibonacci, 
    eliminar_archivo_fibonacci, 
    fibonacci_log_n, 
    EstadisticasEjecucion, 
    obtener_resumen_reportes
};
