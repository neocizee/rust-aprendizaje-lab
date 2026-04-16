use fibonacci::{generar_archivo_fibonacci, eliminar_archivo_fibonacci, obtener_resumen_reportes};
use std::io::{self, Write};
use std::fs;
use std::path::Path;
use num_traits::{Zero, One};
use colored::*;
use clap::Parser;

/// Aplicación de Laboratorio Fibonacci con gestión estricta de recursos.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Número de núcleos de CPU a utilizar (Hilos de Rayon). Por defecto se limita a 2 para seguridad.
    #[arg(short, long, default_value_t = 2)]
    cpu: usize,

    /// Límite máximo de RAM en Megabytes (MB). Por defecto se limita a 2048 MB (2GB).
    #[arg(short, long, default_value_t = 2048)]
    ram: u64,
}

fn main() {
    let args = Args::parse();

    // Configuración MANDATORIA del límite de CPU
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.cpu)
        .build_global()
        .unwrap_or_else(|_| eprintln!("{}", "Aviso: El pool de hilos ya estaba configurado.".yellow()));

    println!("{}", "\n  ╔══════════════════════════════════════════╗".cyan());
    println!("{}", "  ║       LABORATORIO DE FIBONACCI V2.2      ║".cyan().bold());
    println!("{}", "  ╚══════════════════════════════════════════╝".cyan());
    
    println!("  ⚙️  Límite de CPU SEGURO: {} núcleos", args.cpu.to_string().green().bold());
    println!("  💾 Límite de RAM SEGURO: {} MB", args.ram.to_string().green().bold());
    println!("{}", "  (Límites aplicados por defecto para proteger el sistema)".bright_black().italic());

    loop {
        mostrar_estado_almacenamiento();
        
        println!("\n{}", "--- MENÚ DE OPERACIONES ---".yellow().bold());
        println!("1. {} Generar nueva secuencia", "✨".green());
        println!("2. {} Eliminar un resultado", "🗑️".red());
        println!("3. {} Vista previa rápida", "🖥️".blue());
        println!("4. {} Ver historial técnico", "📋".magenta());
        println!("5. {} Salir", "🚪".white());
        print!("\n{} ", "Selección >".bold().cyan());
        io::stdout().flush().unwrap();

        let mut seleccion = String::new();
        io::stdin().read_line(&mut seleccion).unwrap();
        let seleccion = seleccion.trim();

        match seleccion {
            "1" => operacion_generar(Some(args.ram)),
            "2" => operacion_eliminar(),
            "3" => operacion_vista_previa(),
            "4" => operacion_historial(),
            "5" => {
                println!("{}", "¡Saliendo del sistema seguro!".green());
                break;
            }
            _ => println!("{}", "Opción inválida.".red()),
        }
    }
}

fn operacion_generar(limite_ram: Option<u64>) {
    print!("{}", "Cantidad de números a generar: ".bold());
    io::stdout().flush().unwrap();
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    
    if let Ok(n) = n_str.trim().parse::<u64>() {
        println!("\n{}", "🚀 Iniciando proceso de generación segura...".cyan());
        match generar_archivo_fibonacci(n, limite_ram, true) {
            Ok((nombre, _stats)) => {
                println!("\n{}", "✨ El archivo ha sido guardado exitosamente.".green());
                println!("📂 Ubicación: {}", format!("temp/{}", nombre).bright_white());
            },
            Err(e) => {
                eprintln!("\n{}", "❌ ERROR CRÍTICO DURANTE LA GENERACIÓN".red().bold());
                eprintln!("{} {}", "Detalle:".red(), e);
            }
        }
    } else {
        println!("{}", "Error: Ingrese un número válido.".red());
    }
}

fn operacion_eliminar() {
    print!("{}", "ID de archivo a eliminar: ".bold());
    io::stdout().flush().unwrap();
    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str).unwrap();
    let id = id_str.trim();
    
    if let Ok(_) = eliminar_archivo_fibonacci(id) {
        println!("{}", "Limpieza realizada.".blue());
    }
}

fn operacion_vista_previa() {
    print!("{}", "Longitud (máx 50): ".bold());
    io::stdout().flush().unwrap();
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).unwrap();
    
    if let Ok(n) = n_str.trim().parse::<u64>() {
        let n_limitado = if n > 50 { 50 } else { n };
        
        let mut a: num_bigint::BigInt = Zero::zero();
        let mut b: num_bigint::BigInt = One::one();
        
        print!("{}", "Secuencia: ".yellow());
        for i in 0..n_limitado {
            if i > 0 { print!(", "); }
            print!("{}", a);
            let siguiente = &a + &b;
            a = std::mem::replace(&mut b, siguiente);
        }
        if n > 50 { print!("{}", " ... (truncado)".bright_black()); }
        println!();
    }
}

fn operacion_historial() {
    println!("\n{}", "--- REPORTE TÉCNICO DE EJECUCIONES ---".magenta().bold());
    let reportes = obtener_resumen_reportes();
    if reportes.is_empty() {
        println!("{}", "No se encontraron registros.".italic());
    } else {
        for (i, r) in reportes.iter().enumerate() {
            println!("{}. {}", i + 1, r);
        }
    }
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
        if vacio {
            println!("  {}", "(Directorio vacío)".bright_black());
        }
    } else {
        println!("  {}", "(Carpeta temporal aún no creada)".bright_black());
    }
}
