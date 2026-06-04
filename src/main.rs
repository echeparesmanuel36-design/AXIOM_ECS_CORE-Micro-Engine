use macroquad::prelude::*;
use rand::gen_range;
use rayon::prelude::*; // Multithreading real para la CPU

// 1. COMPONENTES (Paquetes de datos puros optimizados)
#[derive(Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy)]
struct Velocity {
    x: f32,
    y: f32,
}

struct ColorComponent {
    base_color: Color,
}

// 2. EL CONTENEDOR ECS SOBERANO
struct AxiomECS {
    positions: Vec<Position>,
    velocities: Vec<Velocity>,
    colors: Vec<ColorComponent>,
}

impl AxiomECS {
    fn new() -> Self {
        Self {
            positions: Vec::new(),
            velocities: Vec::new(),
            colors: Vec::new(),
        }
    }

    fn spawn_entity(&mut self, x: f32, y: f32, vx: f32, vy: f32, color: Color) {
        self.positions.push(Position { x, y });
        self.velocities.push(Velocity { x, vx });
        self.colors.push(ColorComponent { base_color: color });
    }

    // 🔥 SISTEMA DE FÍSICAS PARALELO + REACCIÓN KINÉTICA
    // Multiplica la velocidad por el pulso de audio generado en el bucle principal
    fn update_physics_parallel(&mut self, dt: f32, width: f32, height: f32, audio_pulse: f32) {
        self.positions.par_iter_mut()
            .zip(self.velocities.par_iter_mut())
            .for_each(|(pos, vel)| {
                // Si el "grave" del audio golpea, las partículas aceleran con furia
                let current_speed_modifier = 1.0 + (audio_pulse * 2.5);
                
                pos.x += vel.x * dt * 60.0 * current_speed_modifier;
                pos.y += vel.y * dt * 60.0 * current_speed_modifier;

                // Colisiones contra los bordes
                if pos.x < 0.0 || pos.x > width { vel.x *= -1.0; }
                if pos.y < 0.0 || pos.y > height { vel.y *= -1.0; }
            });
    }

    // 🔥 SISTEMA DE RENDERIZADO AUDIO-REACTIVO
    // Las partículas cambian de tamaño y de brillo neón según los graves virtuales
    fn render_system(&self, audio_pulse: f32) {
        let size = 2.0 + (audio_pulse * 4.0); // Se hacen más grandes con el ritmo
        
        for (pos, col) in self.positions.iter().zip(self.colors.iter()) {
            // Alteramos la intensidad del color según el pulso de audio
            let mut render_color = col.base_color;
            render_color.r = audio_pulse * 0.5; // Mete destellos rojos en el neón con los golpes
            
            draw_circle(pos.x, pos.y, size, render_color);
        }
    }
}

// 3. BUCLE PRINCIPAL (Axiom Engine Core)
#[macroquad::main("AXIOM ECS // KINETIC AUDIO-REACTIVE ENGINE")]
async fn main() {
    let mut ecs = AxiomECS::new();
    
    // Spawneamos 50.000 entidades listas para la acción
    for _ in 0..50000 {
        ecs.spawn_entity(
            gen_range(50.0, screen_width() - 50.0),
            gen_range(50.0, screen_height() - 50.0),
            gen_range(-2.5, 2.5),
            gen_range(-2.5, 2.5),
            Color::new(0.0, gen_range(0.7, 1.0), gen_range(0.3, 0.8), 1.0), // Verde-Cian Neón
        );
    }

    let mut time_accumulator = 0.0;

    loop {
        clear_background(Color::new(0.01, 0.01, 0.02, 1.0)); // Modo búnker profundo
        
        let dt = get_frame_time();
        time_accumulator += dt * 8.0; // Velocidad del ritmo musical

        // 🎼 SIMULADOR DE FRECUENCIA AUDIO-REACTIVA (Simula un bombo de Metal a 130 BPM)
        // Genera un pulso entre 0.0 y 1.0 simulando picos de graves
        let audio_bass = (time_accumulator.sin()).max(0.0).powf(3.0);

        // Ejecución de sistemas en paralelo con los datos de audio inyectados
        ecs.update_physics_parallel(dt, screen_width(), screen_height(), audio_bass);
        ecs.render_system(audio_bass);

        // HUD de Telemetría
        draw_rectangle(10.0, 10.0, 300.0, 95.0, Color::new(0.0, 0.0, 0.0, 0.75));
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 30.0, 20.0, GREEN);
        draw_text(&format!("CPU THREADS: MULTITHREADING ACTIVE"), 20.0, 50.0, 15.0, MAGENTA);
        draw_text(&format!("AUDIO FREQ REACTION: KINETIC PURE"), 20.0, 70.0, 15.0, YELLOW);
        draw_text(&format!("ENTITIES: {}", ecs.positions.len()), 20.0, 90.0, 16.0, CYAN);

        next_frame().await
    }
}
