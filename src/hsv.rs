pub trait HSV {
    fn from_hsv(h: f32, s: f32, v: f32) -> Self;
}

impl HSV for macroquad::color::Color {
    fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let i = (h * 6.0).round();
        let f = h * 6.0 - i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        let (r, g, b) = match i as i32 % 6 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            5 => (v, p, q),
            _ => unreachable!(),
        };

        Self { r, g, b, a: 1.0 }
    }
}
