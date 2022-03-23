// https://github.com/fnky/particle-life/blob/master/src/index.js

mod app;
mod counter;
mod hsv;
mod particle;
mod particle_types;
mod preset;
mod universe;

use macroquad::window::*;

use app::App;

const PHYSICS_SIMULATION_FPS: u32 = 100;
const PHYSICS_DELTA_TIME: f64 = 1.0 / PHYSICS_SIMULATION_FPS as f64;

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Life".to_owned(),
        high_dpi: true,
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut app = App::new(w as f32, h as f32);

    let mut accumulator = 0.0;
    let mut t = 0.0;
    let dt = PHYSICS_DELTA_TIME;

    let mut previous_state: f64 = 0.0;
    let current_state: f64 = 0.0;

    loop {
        let mut frame_time = macroquad::time::get_frame_time() as f64;

        if frame_time > 0.25 {
            frame_time = 0.25;
        }

        accumulator += frame_time;

        while accumulator >= dt {
            previous_state = current_state;
            app.update(t, dt * 2.0);
            t += dt;
            accumulator -= dt;
        }

        let alpha = accumulator / dt;
        let state = current_state * alpha + previous_state * (1.0 - alpha);

        app.draw(state, alpha);

        next_frame().await;
    }
}
