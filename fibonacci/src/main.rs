use fibonacci::{generar_archivo_fibonacci, eliminar_archivo_fibonacci, obtener_resumen_reportes};
use std::io::{self, Write};
use std::fs;
use std::path::Path;
use num_traits::{Zero, One};
use colored::*;

fn main() {
    println!("{}", "\n  ╔══════════════════════════════════════════╗".cyan());
    println!("{}", "  ║       LABORATORIO DE FIBONACCI V2.0      ║".cyan().bold());
    println!("{}", "  ╚══════════════════════════════════════════╝".cyan());

    loop {
        mostrar_estado_almacenamiento();
        
        println!("\n{}", "--- MENÚ DE OPERACIONES ---".yellow().bold());
        println!("1. {} Generar nueva secuencia (+ Log Recursos)", "✨".green());
        println!("2. {} Eliminar un resultado", "🗑️".red());
        println!("3. {} Vista previa rápida en consola", "🖥️".blue());
        println!("4. {} Ver análisis histórico de reportes", "📋".magenta());
        println!("5. {} Salir", "🚪".white());
        print!("\n{} ", "Selección >".bold().cyan());
        io::stdout().flush().unwrap();

        let mut seleccion = String::new();
        io::stdin().read_line(&mut seleccion).unwrap();
        let seleccion = seleccion.trim();

        match seleccion {
            "1" => operacion_generar(),
            "2" => operacion_eliminar(),
            "3" => operacion_vista_previa(),
            "4" => operacion_historial(),
            "5" => {
                println!("{}", "¡Gracias por usar el laboratorio! Saliendo...".green().italic());
                break;
            }
            _ => println!("{}", "Opción no válida. Intente de nuevo.".red()),
        }
    }
}

fn operacion_generar() {
    print!("{}", "Cantidad de números a generar: ".bold());
    io::stdout().flush().unwrap();
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    
    if let Ok(n) = n_str.trim().parse::<u64>() {
        match generar_archivo_fibonacci(n) {
            Ok((nombre, stats)) => {
                println!("\n{}", "✅ OPERACIÓN EXITOSA".green().bold());
                println!("📂 Archivo: {}", nombre.bright_white());
                println!("⏱️  Tiempo: {} ms ({:.3} s)", stats.tiempo_ms, stats.tiempo_s);
                println!("💾 RAM: {:.2} MB", stats.memoria_usada_kb as f64 / 1024.0);
            },
            Err(e) => eprintln!("{} {}", "ERROR:".red().bold(), e),
        }
    } else {
        println!("{}", "Error de formato: Ingrese un número entero positivo.".red());
    }
}

fn operacion_eliminar() {
    print!("{}", "ID o nombre del archivo a eliminar: ".bold());
    io::stdout().flush().unwrap();
    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str).unwrap();
    let id = id_str.trim();
    
    match eliminar_archivo_fibonacci(id) {
        Ok(_) => println!("{}", "Archivo eliminado (si no existía, no se realizó acción).".blue()),
        Err(e) => eprintln!("{} {}", "Error al intentar eliminar:".red(), e),
    }
}

fn operacion_vista_previa() {
    print!("{}", "Longitud de la secuencia a mostrar: ".bold());
    io::stdout().flush().unwrap();
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    
    if let Ok(n) = n_str.trim().parse::<u64>() {
        mostrar_secuencia_consola(n);
    } else {
        println!("{}", "Entrada no válida.".red());
    }
}

fn operacion_historial() {
    println!("\n{}", "--- HISTORIAL DE RENDIMIENTO ---".magenta().bold());
    let reportes = obtener_resumen_reportes();
    if reportes.is_empty() {
        println!("{}", "No hay reportes previos.".italic().white());
    } else {
        for (i, r) in reportes.iter().enumerate() {
            println!("{}. {}", i + 1, r);
        }
    }
}

fn mostrar_secuencia_consola(cantidad: u64) {
    let mut a: num_bigint::BigInt = Zero::zero();
    let mut b: num_bigint::BigInt = One::one();
    
    print!("{}", "Secuencia: ".yellow());
    for i in 0..cantidad {
        if i > 0 { print!(", "); }
        print!("{}", a);
        if i > 50 && cantidad > 60 { // Limite menor para no inundar la terminal
            print!("{}", " ... (truncado por legibilidad)".italic().bright_black());
            break;
        }
        let siguiente = &a + &b;
        a = std::mem::replace(&mut b, siguiente);
    }
    println!();
}

fn mostrar_estado_almacenamiento() {
    println!("\n{}", "📦 ESTADO DE DISCO (temp/)".bold());
    let dir_temp = Path::new("temp");
    if let Ok(entradas) = fs::read_dir(dir_temp) {
        let mut vacio = true;
        for entrada in entradas.filter_map(|e| e.ok()) {
            if let Some(nombre) = entrada.file_name().to_str() {
                if nombre.ends_with(".txt") {
                    println!("  {} {}", "•".cyan(), nombre);
                    vacio = false;
                }
            }
        }
        if vacio { println!("  {}", "(Directorio vacío)".bright_black()); }
    } else {
        println!("  {}", "(Carpeta temporal aún no creada)".bright_black());
    }
}
