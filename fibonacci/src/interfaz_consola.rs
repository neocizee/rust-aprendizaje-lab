use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;
use sysinfo::{System, Pid, ProcessRefreshKind, CpuRefreshKind};
use crate::fibonacci::EstadisticasEjecucion;
use colored::*;
use std::thread;

const COMPUTER: &str = "💻 ";
const RAM_CHIP: &str = "📟 ";
const SUCCESS: &str = "✅ ";

/// Clase principal para el monitoreo de recursos y progreso en tiempo real.
pub struct MonitorConsola {
    multi: MultiProgress,
    pb_progreso: ProgressBar,
    pb_recursos: ProgressBar,
}

impl MonitorConsola {
    /// Inicializa un nuevo monitor con una cantidad objetivo de elementos.
    pub fn nuevo(cantidad: u64) -> Self {
        let multi = MultiProgress::new();
        
        let pb_progreso = multi.add(ProgressBar::new(cantidad));
        pb_progreso.set_style(
            ProgressStyle::with_template(
                "{prefix:>12.cyan.bold} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}"
            )
            .unwrap()
            .progress_chars("█▓░"),
        );
        pb_progreso.set_prefix("Progreso");
        pb_progreso.set_message("Calculando...");

        let pb_recursos = multi.add(ProgressBar::new_spinner());
        pb_recursos.set_style(
            ProgressStyle::with_template("{prefix:>12.yellow.bold} {spinner} {msg}")
                .unwrap()
        );
        pb_recursos.set_prefix("Recursos");
        pb_recursos.enable_steady_tick(Duration::from_millis(100));

        Self {
            multi,
            pb_progreso,
            pb_recursos,
        }
    }

    /// Actualiza el progreso actual y los recursos del sistema.
    pub fn actualizar(&self, actual: u64, sistema: &mut System, pid: Pid) {
        self.pb_progreso.set_position(actual);
        
        // Actualizar solo recursos específicos para eficiencia
        sistema.refresh_processes_specifics(ProcessRefreshKind::new().with_memory());
        sistema.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());

        if let Some(proceso) = sistema.process(pid) {
            let cpu_uso = proceso.cpu_usage();
            let memoria_mb = proceso.memory() as f64 / 1024.0;
            
            self.pb_recursos.set_message(format!(
                "{} CPU: {:.1}% | {} RAM: {:.2} MB",
                COMPUTER.yellow(),
                cpu_uso,
                RAM_CHIP.magenta(),
                memoria_mb
            ));
        }
    }

    /// Finaliza el monitoreo y muestra las estadísticas finales.
    pub fn finalizar(&self, stats: &EstadisticasEjecucion) {
        self.pb_progreso.finish_with_message("¡Completado!");
        self.pb_recursos.finish_and_clear();
        
        println!("\n{} {}", SUCCESS.green(), "Reporte Final de Recursos:".bold().green());
        println!("   {} Tiempo total: {} ms", "⏱️".cyan(), stats.tiempo_ms);
        println!("   {} RAM Pico:     {:.2} MB", "🚀".magenta(), stats.memoria_pico_kb as f64 / 1024.0);
        println!("   {} RAM Promedio: {:.2} MB", "📊".yellow(), stats.memoria_promedio_kb as f64 / 1024.0);
        println!("   {} Velocidad:    {:.0} nums/seg", 
            "⚡".cyan(),
            (self.pb_progreso.length().unwrap_or(1) as f64 / (stats.tiempo_s + 0.001))
        );
    }
}

/// Función auxiliar para simular un proceso con monitoreo (útil para pruebas).
pub fn simular_monitoreo(cantidad: u64) {
    let monitor = MonitorConsola::nuevo(cantidad);
    let mut sistema = System::new_all();
    let pid = Pid::from(std::process::id() as usize);

    for i in 0..=cantidad {
        monitor.actualizar(i, &mut sistema, pid);
        thread::sleep(Duration::from_millis(10));
    }
    
    let stats = EstadisticasEjecucion {
        tiempo_ms: cantidad as u128 * 10,
        tiempo_s: cantidad as f64 * 0.01,
        memoria_pico_kb: 1024,
        memoria_promedio_kb: 512,
    };
    monitor.finalizar(&stats);
}
