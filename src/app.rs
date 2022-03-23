use macroquad::prelude::*;

use crate::counter::BoundedCounter;
use crate::preset::{Preset, PRESETS, PRESETS_COUNT};
use crate::universe::Universe;

struct AppState<'a> {
    universe: Universe,
    preset_counter: BoundedCounter,
    preset_keys: Vec<&'a str>,
    selected_preset: Option<&'a Preset>,
}

impl<'a> AppState<'a> {
    pub fn new(mut universe: Universe, initial_preset: &'a Preset) -> Self {
        let preset_keys_max_index = PRESETS_COUNT.checked_sub(1).unwrap_or(0);
        let preset_keys = PRESETS.keys().cloned().collect::<Vec<_>>();

        universe.load_preset(initial_preset);

        Self {
            universe,
            preset_keys,
            preset_counter: BoundedCounter {
                upper: preset_keys_max_index,
                continious: true,
                ..Default::default()
            },
            selected_preset: Some(initial_preset),
        }
    }

    pub fn current_preset(&self) -> Option<(&str, &Preset)> {
        if let Some(preset) = self.selected_preset {
            Some((self.preset_keys[self.preset_counter.current()], preset))
        } else {
            None
        }
    }

    pub fn load_next_preset(&mut self) {
        let index = self.preset_counter.increment();
        self.selected_preset = PRESETS.get(self.preset_keys[index]);
        self.load_current_preset();
    }

    pub fn load_prev_preset(&mut self) {
        let index = self.preset_counter.decrement();
        self.selected_preset = PRESETS.get(self.preset_keys[index]);
        self.load_current_preset();
    }

    pub fn load_current_preset(&mut self) {
        if let Some(preset) = self.selected_preset {
            self.universe.load_preset(preset);
        }
    }
}

const STEPS_PER_FRAME_LOW: usize = 1;
const STEPS_PER_FRAME_HIGH: usize = 10;

pub struct App<'a> {
    state: AppState<'a>,
    steps_per_frame: usize,
}

impl<'a> App<'a> {
    pub fn new(width: f32, height: f32) -> Self {
        let initial_preset = PRESETS.get("Chaos").unwrap();

        Self {
            state: AppState::new(Universe::new(width as f32, height as f32), initial_preset),
            steps_per_frame: STEPS_PER_FRAME_LOW,
        }
    }

    pub fn update(&mut self, _t: f64, _delta: f64) {
        self.handle_input();
    }

    pub fn draw(&mut self, _state: f64, _alpha: f64) {
        clear_background(BLACK);

        for i in 0..self.steps_per_frame {
            let opacity = (i + 1) as f32 / self.steps_per_frame as f32;
            self.state.universe.step();
            self.state.universe.draw(opacity);
        }

        self.draw_fps_counter();
        self.draw_preset_status();
    }

    fn draw_fps_counter(&self) {
        draw_text(&format!("{:.1} FPS", get_fps()), 20.0, 20.0, 20.0, DARKGRAY);
    }

    fn draw_preset_status(&self) {
        let preset = self.state.current_preset();
        let title = preset.map(|x| x.0).unwrap_or("");
        draw_text(&format!("< {} >", title), 20.0, 40.0, 20.0, WHITE);
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Right) {
            self.state.load_next_preset();
        }

        if is_key_pressed(KeyCode::Left) {
            self.state.load_prev_preset();
        }

        if is_key_down(KeyCode::Space) {
            self.steps_per_frame = STEPS_PER_FRAME_HIGH;
        }

        if is_key_released(KeyCode::Space) {
            self.steps_per_frame = STEPS_PER_FRAME_LOW;
        }
    }
}
