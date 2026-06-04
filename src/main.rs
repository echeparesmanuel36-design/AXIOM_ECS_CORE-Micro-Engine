use macroquad::prelude::*;
use rand::gen_range;

// 1. COMPONENTES (Datos puros, alineados en memoria para que la CPU vuele)
struct Position {
    x: f32,
    y: f32,
}

struct Velocity {
    x: f32,
    y: f32,
}

struct ColorComponent {
    color: Color,
}

// 2. EL CONTENEDOR ECS (El chasis del motor)
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

    // Método para spawnear entidades masivamente
    fn spawn_entity(&mut self, x: f32, y: f32, vx: f32, vy: f32, color: Color) {
        self.positions.push(Position { x, y });
        self.velocities.push(Velocity { x, vx });
        self.colors.push(ColorComponent { color });
    }

    // SISTEMA DE FÍSICAS (Bare-Metal: procesa todos los movimientos en un bucle compacto)
    fn update_physics(&mut self, dt: f32, width: f32, height: f32) {
        for (pos, vel) in self.positions.iter_mut().zip(self.velocities.iter_mut()) {
            pos.x += vel.x * dt * 60.0;
            pos.y += vel.y * dt * 60.0;

            // Colisiones contra los bordes de la pantalla (Rebote elástico)
            if pos.x < 0.0 || pos.x > width { vel.x *= -1.0; }
            if pos.y < 0.0 || pos.y > height { vel.y *= -1.0; }
        }
    }

    // SISTEMA DE RENDERIZADO (Dibuja todo de golpe aprovechando la GPU)
    fn render_system(&self) {
        for (pos, col) in self.positions.iter().zip(self.colors.iter()) {
            draw_circle(pos.x, pos.y, 3.0, col.color);
        }
    }
}

// 3. EL BUCLE PRINCIPAL DEL MOTOR
#[macroquad::main("AXIOM ECS // ENGINE CORE v1.0")]
async fn main() {
    let mut ecs = AxiomECS::new();
    
    // ¡Spawneamos 20.000 entidades de golpe para demostrar el poder de Rust!
    for _ in 0..20000 {
        ecs.spawn_entity(
            gen_range(100.0, screen_width() - 100.0),
            gen_range(100.0, screen_height() - 100.0),
            gen_range(-3.0, 3.0),
            gen_range(-3.0, 3.0),
            Color::new(0.0, gen_range(0.5, 1.0), gen_range(0.2, 0.6), 1.0), // Tonos verde/azul neón (Estilo Axiom)
        );
    }

    loop {
        // Limpiamos pantalla con el tono oscuro búnker de siempre
        clear_background(Color::new(0.02, 0.02, 0.04, 1.0));
        
        let dt = get_frame_time();

        // Ejecutamos los sistemas del motor
        ecs.update_physics(dt, screen_width(), screen_height());
        ecs.render_system();

        // HUD de telemetría de rendimiento
        draw_rectangle(10.0, 10.0, 250.0, 60.0, Color::new(0.0, 0.0, 0.0, 0.6));
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 30.0, 20.0, GREEN);
        draw_text(&format!("ENTIDADES: {}", ecs.positions.len()), 20.0, 50.0, 20.0, CYAN);

        next_frame().await
    }
}
