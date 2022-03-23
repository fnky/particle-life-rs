use macroquad::prelude::*;

#[derive(Debug)]
pub struct ParticleTypes {
    colors: Vec<Color>,
    attract: Vec<f32>,
    min_r: Vec<f32>,
    max_r: Vec<f32>,
}

impl ParticleTypes {
    pub fn new(size: usize) -> Self {
        Self {
            colors: vec![WHITE; size],
            attract: vec![0.0; size * size],
            min_r: vec![0.0; size * size],
            max_r: vec![0.0; size * size],
        }
    }

    pub fn resize(&mut self, size: usize) {
        self.colors.resize(size, WHITE);
        self.attract.resize(size * size, 0.0);
        self.min_r.resize(size * size, 0.0);
        self.max_r.resize(size * size, 0.0);
    }

    pub fn size(&self) -> usize {
        self.colors.len()
    }

    pub fn get_color(&self, index: usize) -> Option<&Color> {
        self.colors.get(index)
    }

    pub fn get_color_mut(&mut self, index: usize) -> Option<&mut Color> {
        self.colors.get_mut(index)
    }

    pub fn set_color(&mut self, index: usize, value: Color) {
        let color = self.colors.get_mut(index).expect("no color at index");
        *color = value;
    }

    pub fn get_attract(&self, i: usize, j: usize) -> Option<&f32> {
        self.attract.get(i * self.size() + j)
    }

    pub fn set_attract(&mut self, i: usize, j: usize, value: f32) {
        let index = i * self.size() + j;
        let attract = self.attract.get_mut(index).expect("no attract at index");
        *attract = value;
    }

    pub fn get_min_r(&self, i: usize, j: usize) -> Option<&f32> {
        self.min_r.get(i * self.size() + j)
    }

    pub fn set_min_r(&mut self, i: usize, j: usize, value: f32) {
        let index = i * self.size() + j;
        let min_r = self.min_r.get_mut(index).expect("no min_r at index");
        *min_r = value;
    }

    pub fn get_max_r(&self, i: usize, j: usize) -> Option<&f32> {
        self.max_r.get(i * self.size() + j)
    }

    pub fn set_max_r(&mut self, i: usize, j: usize, value: f32) {
        let index = i * self.size() + j;
        let max_r = self.max_r.get_mut(index).expect("no max_r at index");
        *max_r = value;
    }
}
