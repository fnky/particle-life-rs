use phf::phf_ordered_map;

#[derive(Debug)]
pub struct Population {
    pub particle_types: usize,
    pub particles: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Seed {
    pub attract_mean: f32,
    pub attract_std: f32,
    pub min_r_lower: f32,
    pub min_r_upper: f32,
    pub max_r_lower: f32,
    pub max_r_upper: f32,
    pub friction: f32,
    pub flat_force: bool,
}

#[derive(Debug)]
pub struct Preset {
    pub population: Population,
    pub seed: Seed,
}

pub static PRESETS: phf::OrderedMap<&'static str, Preset> = phf_ordered_map! {
    "Balanced" => Preset {
        population: Population {
            particle_types: 9,
            particles: 400
        },
        seed: Seed {
            attract_mean: -0.02,
            attract_std: 0.06,
            min_r_lower: 0.0,
            min_r_upper: 20.0,
            max_r_lower: 20.0,
            max_r_upper: 70.0,
            friction: 0.05,
            flat_force: false
        },
    },
    "Chaos" => Preset {
        population: Population {
            particle_types: 6,
            particles: 400
        },
        seed: Seed {
            attract_mean: 0.02,
            attract_std: 0.04,
            min_r_lower: 0.0,
            min_r_upper: 30.0,
            max_r_lower: 30.0,
            max_r_upper: 100.0,
            friction: 0.01,
            flat_force: false
        },
    },
    "Diversity" => Preset {
        population: Population {
            particle_types: 12,
            particles: 400
        },
        seed: Seed {
            attract_mean: -0.01,
            attract_std: 0.04,
            min_r_lower: 0.0,
            min_r_upper: 20.0,
            max_r_lower: 10.0,
            max_r_upper: 60.0,
            friction: 0.05,
            flat_force: true
        },
    },
    "Frictionless" => Preset {
        population: Population {
            particle_types: 6,
            particles: 300
        },
        seed: Seed {
            attract_mean: 0.01,
            attract_std: 0.005,
            min_r_lower: 10.0,
            min_r_upper: 10.0,
            max_r_lower: 10.0,
            max_r_upper: 60.0,
            friction: 0.0,
            flat_force: true
        },
    },
    "Gliders" => Preset {
        population: Population {
            particle_types: 6,
            particles: 400
        },
        seed: Seed {
            attract_mean: 0.0,
            attract_std: 0.06,
            min_r_lower: 0.0,
            min_r_upper: 20.0,
            max_r_lower: 10.0,
            max_r_upper: 50.0,
            friction: 0.1,
            flat_force: true
        },
    },
    "Homogeneity" => Preset {
        population: Population {
            particle_types: 4,
            particles: 400
        },
        seed: Seed {
            attract_mean: 0.0,
            attract_std: 0.04,
            min_r_lower: 10.0,
            min_r_upper: 10.0,
            max_r_lower: 10.0,
            max_r_upper: 80.0,
            friction: 0.05,
            flat_force: true
        },
    },
    "Large Clusters" => Preset {
        population: Population {
            particle_types: 6,
            particles: 400
        },
        seed: Seed {
            attract_mean: 0.025,
            attract_std: 0.02,
            min_r_lower: 0.0,
            min_r_upper: 30.0,
            max_r_lower: 30.0,
            max_r_upper: 100.0,
            friction: 0.2,
            flat_force: false
        },
    },
    "Medium Clusters" => Preset {
        population: Population {
            particle_types: 6,
            particles: 400
        },
        seed: Seed {
            attract_mean: 0.02,
            attract_std: 0.05,
            min_r_lower: 0.0,
            min_r_upper: 20.0,
            max_r_lower: 20.0,
            max_r_upper: 50.0,
            friction: 0.05,
            flat_force: false
        },
    },
    "Quiescence" => Preset {
        population: Population {
            particle_types: 6,
            particles: 300
        },
        seed: Seed {
            attract_mean: -0.02,
            attract_std: 0.1,
            min_r_lower: 10.0,
            min_r_upper: 20.0,
            max_r_lower: 20.0,
            max_r_upper: 60.0,
            friction: 0.2,
            flat_force: false
        },
    },
    "Small Clusters" => Preset {
        population: Population {
            particle_types: 6,
            particles: 600
        },
        seed: Seed {
            attract_mean: -0.005,
            attract_std: 0.01,
            min_r_lower: 10.0,
            min_r_upper: 10.0,
            max_r_lower: 20.0,
            max_r_upper: 50.0,
            friction: 0.01,
            flat_force: false
        },
    },
};

pub static PRESETS_COUNT: usize = PRESETS.len();
