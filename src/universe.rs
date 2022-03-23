use ::rand::prelude::ThreadRng;
use macroquad::color::Color;
use macroquad::prelude::*;
use rand_distr::{Distribution, Normal, Uniform};

use crate::hsv::HSV;
use crate::particle::Particle;
use crate::particle_types::ParticleTypes;
use crate::preset::{Preset, Seed};

const RADIUS: f32 = 5.0;
const DIAMETER: f32 = 2.0 * RADIUS;
const R_SMOOTH: f64 = 2.0;

#[derive(Debug)]
pub struct Universe {
    types: ParticleTypes,
    particles: Vec<Particle>,
    width: f32,
    height: f32,
    center_x: f32,
    center_y: f32,
    pub zoom: f32,
    attract_mean: f32,
    attract_std: f32,
    min_r_lower: f32,
    min_r_upper: f32,
    max_r_lower: f32,
    max_r_upper: f32,
    friction: f32,
    flat_force: bool,
    wrap: bool,
    rng: ThreadRng,
}

impl Universe {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            types: ParticleTypes::new(0),
            particles: vec![Particle::default(); 0],
            width,
            height,
            center_x: width * 0.5,
            center_y: height * 0.5,
            zoom: 1.0,
            attract_mean: 0.0,
            attract_std: 0.0,
            min_r_lower: 0.0,
            min_r_upper: 0.0,
            max_r_lower: 0.0,
            max_r_upper: 0.0,
            friction: 0.0,
            flat_force: false,
            wrap: false,
            rng: ThreadRng::default(),
        }
    }

    pub fn reseed(&mut self, seed: &Seed) {
        self.attract_mean = seed.attract_mean;
        self.attract_std = seed.attract_std;
        self.min_r_lower = seed.min_r_lower;
        self.min_r_upper = seed.min_r_upper;
        self.max_r_lower = seed.max_r_lower;
        self.max_r_upper = seed.max_r_upper;
        self.friction = seed.friction;
        self.flat_force = seed.flat_force;
        self.set_random_types();
        self.set_random_particles();
    }

    pub fn set_population(&mut self, num_types: usize, num_particles: usize) {
        self.types.resize(num_types);
        self.particles.resize(num_particles, Particle::default());
    }

    pub fn load_preset(&mut self, preset: &Preset) {
        self.set_population(
            preset.population.particle_types,
            preset.population.particles,
        );
        self.reseed(&preset.seed);
    }

    // pub fn set_engine<R: RngCore>(&mut self, new_engine: R) {}

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_random_types(&mut self) {
        let rand_attr = Normal::new(self.attract_mean, self.attract_std).unwrap();
        let rand_min_r = Uniform::new_inclusive(self.min_r_lower, self.min_r_upper);
        let rand_max_r = Uniform::new_inclusive(self.max_r_lower, self.max_r_upper);

        for i in 0..self.types.size() {
            self.types.set_color(
                i,
                Color::from_hsv(
                    i as f32 / self.types.size() as f32,
                    1.0,
                    (i as f32 % 2.0) * 0.5 + 0.5,
                ),
            );

            for j in 0..self.types.size() {
                if i == j {
                    self.types
                        .set_attract(i, j, -rand_attr.sample(&mut self.rng).abs());
                    self.types.set_min_r(i, j, DIAMETER);
                } else {
                    self.types
                        .set_attract(i, j, rand_attr.sample(&mut self.rng));
                    self.types
                        .set_min_r(i, j, rand_min_r.sample(&mut self.rng).max(DIAMETER))
                }

                self.types.set_max_r(
                    i,
                    j,
                    rand_max_r
                        .sample(&mut self.rng)
                        .max(*self.types.get_min_r(i, j).unwrap()),
                );

                // Keep radii symmetric
                self.types
                    .set_max_r(j, i, *self.types.get_max_r(i, j).unwrap());
                self.types
                    .set_min_r(j, i, *self.types.get_min_r(i, j).unwrap());
            }
        }
    }

    pub fn set_random_particles(&mut self) {
        let rand_type = Uniform::new(0, self.types.size() - 1);
        let rand_uni = Uniform::new(0.0, 1.0);
        let rand_norm = Normal::new(0.0, 1.0).unwrap();

        // for i in 0..self.particles.len() {}
        for mut p in self.particles.iter_mut() {
            p.particle_type = rand_type.sample(&mut self.rng);
            p.x = (rand_uni.sample(&mut self.rng) * 0.5 + 0.25) * self.width as f64;
            p.y = (rand_uni.sample(&mut self.rng) * 0.5 + 0.25) * self.height as f64;

            p.vx = rand_norm.sample(&mut self.rng) * 0.2;
            p.vy = rand_norm.sample(&mut self.rng) * 0.2;
        }
    }

    pub fn step(&mut self) {
        let size = self.particles.len();

        // TODO: Rewrite in a rust-way?
        for i in 0..size {
            // Interactions
            for j in 0..size {
                // Cannot overlap
                if i == j {
                    continue;
                }

                // Wtf is this?
                let first = i.min(j);
                let second = i.max(j);
                let (head, tail) = self.particles.split_at_mut(second);

                // Current particle
                let p;
                // Other particle
                let q;
                if i < j {
                    p = &mut head[first];
                    q = &mut tail[0];
                } else {
                    p = &mut tail[0];
                    q = &mut head[first];
                }

                // Get deltas
                let mut dx = q.x - p.x;
                let mut dy = q.y - p.y;

                if self.wrap {
                    if dx > self.width as f64 * 0.5 {
                        dx -= self.width as f64;
                    } else if dx < -self.width as f64 * 0.5 {
                        dx += self.width as f64;
                    }

                    if dy > self.height as f64 * 0.5 {
                        dy -= self.height as f64;
                    } else if dy < -self.height as f64 * 0.5 {
                        dy += self.height as f64;
                    }
                }

                // Get distance squared
                let r2 = dx * dx + dy * dy;
                let min_r = *self
                    .types
                    .get_min_r(p.particle_type, q.particle_type)
                    .unwrap();
                let max_r = *self
                    .types
                    .get_max_r(p.particle_type, q.particle_type)
                    .unwrap();

                if r2 > max_r as f64 * max_r as f64 || r2 < 0.01 {
                    continue;
                }

                // Normalize displacement
                let r = r2.sqrt();
                dx /= r;
                dy /= r;

                // Calculate force
                let mut f = 0.0;
                if r > min_r as f64 {
                    if self.flat_force {
                        f = *self
                            .types
                            .get_attract(p.particle_type, q.particle_type)
                            .unwrap() as f64;
                    } else {
                        let numer = 2.0 * (r - 0.5 * (max_r as f64 + min_r as f64)).abs();
                        let denom = max_r as f64 - min_r as f64;
                        f = *self
                            .types
                            .get_attract(p.particle_type, q.particle_type)
                            .unwrap() as f64
                            * (1.0 - numer / denom);
                    }
                } else {
                    f = R_SMOOTH
                        * min_r as f64
                        * (1.0 / (min_r as f64 + R_SMOOTH) - 1.0 / (r + R_SMOOTH));
                }

                (*p).vx += f * dx;
                (*p).vy += f * dy;
            }
        }

        // Update position
        for i in 0..size {
            // Current particle
            let p = &mut self.particles[i];

            // Update position and velocity
            (*p).x += p.vx;
            (*p).y += p.vy;
            (*p).vx *= 1.0 - self.friction as f64;
            (*p).vy *= 1.0 - self.friction as f64;

            // Check for wall collision
            if self.wrap {
                if p.x < 0.0 {
                    (*p).x += self.width as f64;
                } else if p.x >= self.width as f64 {
                    (*p).x -= self.width as f64;
                }

                if p.y < 0.0 {
                    (*p).y += self.height as f64;
                } else if p.y >= self.height as f64 {
                    (*p).y -= self.height as f64;
                }
            } else {
                if p.x <= DIAMETER as f64 {
                    (*p).vx = -p.vx;
                    (*p).x = DIAMETER as f64;
                } else if p.x >= self.width as f64 - DIAMETER as f64 {
                    (*p).vx = -p.vx;
                    (*p).x = self.width as f64 - DIAMETER as f64;
                }

                if p.y <= DIAMETER as f64 {
                    (*p).vy = -p.vy;
                    (*p).y = DIAMETER as f64;
                } else if p.y >= self.height as f64 - DIAMETER as f64 {
                    (*p).vy = -p.vy;
                    (*p).y = self.height as f64 - DIAMETER as f64;
                }
            }
        }
    }

    pub fn draw(&mut self, opacity: f32) {
        let circle_radius = RADIUS * self.zoom;
        for p in self.particles.iter() {
            let x = (p.x as f32 - self.center_x) * self.zoom + self.width / 2.0;
            let y = (p.y as f32 - self.center_y) * self.zoom + self.height / 2.0;

            let color = self.types.get_color_mut(p.particle_type).unwrap();
            color.a = opacity;
            draw_circle(x, y, circle_radius, *color);
        }
    }

    // pub fn get_index(&self, x: f64, y: f64) -> Option<usize> {
    //     let [cx, cy] = self.to_center(x, y);

    //     for (index, p) in self.particles.iter().enumerate() {
    //         let dx = p.x - cx;
    //         let dy = p.y - cx;

    //         if dx * dx + dy * dy < RADIUS as f64 * RADIUS as f64 {
    //             return Some(index);
    //         }
    //     }

    //     None
    // }

    // pub fn get_particle_x(&self, index: usize) -> Option<f32> {
    //     self.particles.get(index).map(|p| p.x)
    // }

    // pub fn get_particle_y(&self, index: usize) -> Option<f32> {
    //     self.particles.get(index).map(|p| p.y)
    // }

    // pub fn to_center(&self, x: f32, y: f32) -> [f32; 2] {
    //     let cx = self.center_x + (x - self.width / 2.0) / self.zoom;
    //     let cy = self.center_y + (y - self.height / 2.0) / self.zoom;
    //     [cx, cy]
    // }

    pub fn set_zoom(&mut self, cx: f32, cy: f32, zoom: f32) {
        // Apply the zoom
        self.center_x = cx;
        self.center_y = cy;
        self.zoom = zoom.max(1.0);

        // Clamp to make sure camera doesn't go out of bounds
        self.center_x = self
            .center_x
            .min(self.width * (1.0 - 0.5 / self.zoom))
            .max(self.width * (0.5 / self.zoom));
        self.center_y = self
            .center_y
            .min(self.height * (1.0 - 0.5 / self.zoom))
            .max(self.height * (0.5 / self.zoom));
    }
}
