use macroquad::prelude::*;
use rand::gen_range;
use rayon::prelude::*; // Mágia para el Multithreading Puro

// 1. COMPONENTES (Paquetes de datos puros alineados en memoria)
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
    color: Color,
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
        self.colors.push(ColorComponent { color });
    }

    // 🔥 SISTEMA DE FÍSICAS PARALELO (MULTITHREADING REAL)
    // Usamos Rayon para dividir el bucle entre todos los hilos de la CPU
    fn update_physics_parallel(&mut self, dt: f32, width: f32, height: f32) {
        // Combinamos las posiciones y velocidades en paralelo
        self.positions.par_iter_mut()
            .zip(self.velocities.par_iter_mut())
            .for_each(|(pos, vel)| {
                pos.x += vel.x * dt * 60.0;
                pos.y += vel.y * dt * 60.0;

                // Colisiones elásticas contra los bordes de la pantalla
                if pos.x < 0.0 || pos.x > width { vel.x *= -1.0; }
                if pos.y < 0.0 || pos.y > height { vel.y *= -1.0; }
            });
    }

    // SISTEMA DE RENDERIZADO EN GPU
    fn render_system(&self) {
        for (pos, col) in self.positions.iter().zip(self.colors.iter()) {
            draw_circle(pos.x, pos.y, 2.5, col.color);
        }
    }
}

// 3. EL BUCLE PRINCIPAL
#[macroquad::main("AXIOM ECS // MULTITHREADED ENGINE")]
async fn main() {
    let mut ecs = AxiomECS::new();
    
    // ¡Subimos la apuesta a 50.000 entidades para que se note el Multithreading!
    for _ in 0..50000 {
        ecs.spawn_entity(
            gen_range(50.0, screen_width() - 50.0),
            gen_range(50.0, screen_height() - 50.0),
            gen_range(-4.0, 4.0),
            gen_range(-4.0, 4.0),
            Color::new(0.0, gen_range(0.6, 1.0), gen_range(0.2, 0.7), 1.0), // Verde Neón Axiom
        );
    }

    loop {
        clear_background(Color::new(0.01, 0.01, 0.03, 1.0)); // Fondo negro búnker profundo
        
        let dt = get_frame_time();

        // Ejecución de físicas en paralelo usando todos los núcleos del procesador
        ecs.update_physics_parallel(dt, screen_width(), screen_height());
        
        // Dibujado
        ecs.render_system();

        // Panel de Control y Telemetría
        draw_rectangle(10.0, 10.0, 280.0, 80.0, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 30.0, 20.0, GREEN);
        draw_text(&format!("THREADS ACTIVE: PARALLEL CPU"), 20.0, 50.0, 16.0, MAGENTA);
        draw_text(&format!("ENTITIES: {}", ecs.positions.len()), 20.0, 70.0, 18.0, CYAN);

        next_frame().await
    }
}
