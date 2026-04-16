use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fs;
use std::path::Path;
use std::io::{self, Write, BufWriter};
use std::time::Instant;
use sysinfo::{System, Pid};
use crate::interfaz_consola::MonitorConsola;

/// Estructura para almacenar las métricas de rendimiento de una ejecución.
#[derive(Debug, Clone)]
pub struct EstadisticasEjecucion {
    pub tiempo_ms: u128,
    pub tiempo_s: f64,
    pub memoria_pico_kb: u64,
    pub memoria_promedio_kb: u64,
}

/// Calcula el n-ésimo número de Fibonacci en tiempo O(log n).
pub fn fibonacci_log_n(n: u64) -> BigInt {
    if n == 0 {
        return BigInt::zero();
    }
    
    fn duplicacion_rapida(k: u64) -> (BigInt, BigInt) {
        if k == 0 {
            return (BigInt::zero(), BigInt::one());
        }
        
        let (f_k, f_kp1) = duplicacion_rapida(k >> 1);
        
        let dos_f_kp1 = &f_kp1 << 1;
        let diferencia = &dos_f_kp1 - &f_k;
        let f_2k = &f_k * &diferencia;
        
        let f_2kp1 = &f_kp1 * &f_kp1 + &f_k * &f_k;
        
        if k % 2 == 0 {
            (f_2k, f_2kp1)
        } else {
            let siguiente_fib = &f_2k + &f_2kp1;
            (f_2kp1, siguiente_fib)
        }
    }

    duplicacion_rapida(n).0
}

/// Genera una secuencia de Fibonacci con la opción de limitar la RAM.
pub fn generar_archivo_fibonacci(
    cantidad: u64, 
    limite_ram_mb: Option<u64>,
    mostrar_progreso: bool
) -> io::Result<(String, EstadisticasEjecucion)> {
    let dir_temporal = Path::new("temp");
    if !dir_temporal.exists() {
        fs::create_dir_all(dir_temporal)?;
    }

    let siguiente_id = obtener_siguiente_id(dir_temporal);
    let nombre_archivo = format!("{}.txt", siguiente_id);
    let ruta_archivo = dir_temporal.join(&nombre_archivo);

    let mut sistema = System::new_all();
    let pid = Pid::from(std::process::id() as usize);
    
    let tiempo_inicio = Instant::now();

    let archivo = fs::File::create(&ruta_archivo)?;
    let mut escritor = BufWriter::new(archivo);

    let mut a: BigInt = Zero::zero();
    let mut b: BigInt = One::one();

    let mut memoria_acumulada_kb: u64 = 0;
    let mut muestras_memoria: u64 = 0;
    let mut memoria_pico_kb: u64 = 0;

    let monitor = if mostrar_progreso {
        Some(MonitorConsola::nuevo(cantidad))
    } else {
        None
    };

    for i in 0..cantidad {
        if i > 0 {
            escritor.write_all(b", ")?;
        }
        write!(escritor, "{}", a)?;
        let siguiente = &a + &b;
        a = std::mem::replace(&mut b, siguiente);

        // Actualización de monitor e hilos cada N elementos para no penalizar el rendimiento
        if i % 50 == 0 {
            if let Some(ref m) = monitor {
                m.actualizar(i, &mut sistema, pid);
            }

            sistema.refresh_process(pid);
            if let Some(proceso) = sistema.process(pid) {
                let memoria_actual = proceso.memory(); // sysinfo devuelve KB o Bytes? sysinfo 0.30 devuelve KB en memory()
                memoria_acumulada_kb += memoria_actual;
                muestras_memoria += 1;
                if memoria_actual > memoria_pico_kb {
                    memoria_pico_kb = memoria_actual;
                }

                if let Some(limite) = limite_ram_mb {
                    let memoria_mb = proceso.memory() as f64 / 1024.0;
                    if memoria_mb > limite as f64 {
                        return Err(io::Error::new(
                            io::ErrorKind::Other, 
                            format!("Límite de RAM excedido: {} MB", memoria_mb)
                        ));
                    }
                }
            }
        }
    }
    escritor.flush()?;

    let duracion = tiempo_inicio.elapsed();
    
    let memoria_promedio = if muestras_memoria > 0 {
        memoria_acumulada_kb / muestras_memoria
    } else {
        memoria_pico_kb
    };

    let estadisticas = EstadisticasEjecucion {
        tiempo_ms: duracion.as_millis(),
        tiempo_s: duracion.as_secs_f64(),
        memoria_pico_kb,
        memoria_promedio_kb: memoria_promedio,
    };

    registrar_ejecucion(siguiente_id, cantidad, &estadisticas)?;

    if let Some(ref m) = monitor {
        m.actualizar(cantidad, &mut sistema, pid);
        m.finalizar(&estadisticas);
    }

    Ok((nombre_archivo, estadisticas))
}

fn obtener_siguiente_id(directorio: &Path) -> u32 {
    let mut siguiente_id = 1;
    if let Ok(entradas) = fs::read_dir(directorio) {
        let mut ids: Vec<u32> = entradas
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                e.file_name()
                    .to_str()?
                    .strip_suffix(".txt")?
                    .parse::<u32>()
                    .ok()
            })
            .collect();
        ids.sort_unstable();
        if let Some(&ultimo_id) = ids.last() {
            siguiente_id = ultimo_id + 1;
        }
    }
    siguiente_id
}

fn registrar_ejecucion(id: u32, n: u64, estadisticas: &EstadisticasEjecucion) -> io::Result<()> {
    let dir_logs = Path::new("logs");
    if !dir_logs.exists() {
        fs::create_dir_all(dir_logs)?;
    }
    let ruta_log = dir_logs.join(format!("{}_registro_recursos.txt", id));
    let mut archivo_log = fs::File::create(ruta_log)?;
    
    writeln!(archivo_log, "--- Reporte de Ejecución (ID: {}) ---", id)?;
    writeln!(archivo_log, "Elementos Generados: {}", n)?;
    writeln!(archivo_log, "Tiempo Transcurrido: {} ms ({:.3} s)", estadisticas.tiempo_ms, estadisticas.tiempo_s)?;
    writeln!(archivo_log, "Uso de Memoria RAM (Pico): {:.2} MB", estadisticas.memoria_pico_kb as f64 / 1024.0)?;
    writeln!(archivo_log, "Uso de Memoria RAM (Promedio): {:.2} MB", estadisticas.memoria_promedio_kb as f64 / 1024.0)?;
    
    Ok(())
}

pub fn obtener_resumen_reportes() -> Vec<String> {
    let dir_temporal = Path::new("temp");
    let mut resumen = Vec::new();

    if let Ok(entradas) = fs::read_dir(dir_temporal) {
        for entrada in entradas.filter_map(|e| e.ok()) {
            let nombre_archivo = entrada.file_name();
            let nombre_str = nombre_archivo.to_str().unwrap_or("");
            if nombre_str.ends_with(".txt") {
                let id = nombre_str.strip_suffix(".txt").unwrap_or("");
                let nombre_log = format!("{}_registro_recursos.txt", id);
                let ruta_log = Path::new("logs").join(&nombre_log);
                
                let analisis = if ruta_log.exists() {
                    fs::read_to_string(ruta_log).unwrap_or_else(|_| "Registro inaccesible".to_string())
                } else {
                    "Sin archivo de registro".to_string()
                };

                resumen.push(format!("Archivo: {} | Análisis: {}", nombre_str, analisis.replace('\n', " ")));
            }
        }
    }
    resumen
}

pub fn eliminar_archivo_fibonacci(id_o_nombre: &str) -> io::Result<()> {
    let nombre_archivo = if id_o_nombre.ends_with(".txt") {
        id_o_nombre.to_string()
    } else {
        format!("{}.txt", id_o_nombre)
    };

    let p1 = Path::new("temp").join(&nombre_archivo);
    let id = nombre_archivo.strip_suffix(".txt").unwrap_or("");
    let p2 = Path::new("logs").join(format!("{}_registro_recursos.txt", id));

    if p1.exists() { fs::remove_file(p1)?; }
    if p2.exists() { fs::remove_file(p2)?; }
    Ok(())
}
