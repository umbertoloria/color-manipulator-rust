#[derive(Clone, Copy)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub fn safe_color(c: f32) -> f32 { filter_scalar(c, 0.0, 255.0) }

pub fn filter_scalar(scalar: f32, min: f32, max: f32) -> f32 {
    if scalar < min {
        min
    } else {
        if scalar > max {
            max
        } else {
            scalar
        }
    }
}
pub fn filter_color(c: RGB, min: f32, max: f32) -> RGB {
    RGB {
        r: filter_scalar(c.r, min, max),
        g: filter_scalar(c.g, min, max),
        b: filter_scalar(c.b, min, max),
    }
}
pub fn filter_scalar_and_stretch(scalar: f32, min: f32, max: f32) -> f32 {
    (filter_scalar(scalar, min, max) - min) * (1.0 / (max - min))
}


pub fn color(r: f32, g: f32, b: f32) -> RGB {
    RGB {
        r,
        g,
        b,
    }
}

pub fn add(a: RGB, b: RGB) -> RGB {
    RGB {
        r: a.r + b.r,
        g: a.g + b.g,
        b: a.b + b.b,
    }
}
pub fn sub(a: RGB, b: RGB) -> RGB {
    add(a, RGB {
        r: -b.r,
        g: -b.g,
        b: -b.b,
    })
}

pub fn mult(a: RGB, b: RGB) -> RGB {
    RGB {
        r: a.r * b.r,
        g: a.g * b.g,
        b: a.b * b.b,
    }
}

pub fn scalar(scalar: f32) -> RGB {
    RGB {
        r: scalar,
        g: scalar,
        b: scalar,
    }
}


pub fn only_r(r: f32) -> RGB {
    RGB {
        r,
        g: 0.0,
        b: 0.0,
    }
}
pub fn only_g(g: f32) -> RGB {
    RGB {
        r: 0.0,
        g,
        b: 0.0,
    }
}
pub fn only_b(b: f32) -> RGB {
    RGB {
        r: 0.0,
        g: 0.0,
        b,
    }
}
pub fn get_r(c: RGB) -> f32 { c.r }
pub fn get_g(c: RGB) -> f32 { c.g }
pub fn get_b(c: RGB) -> f32 { c.b }

pub fn xor_color(a: RGB, b: RGB) -> RGB {
    color(
        (a.r - b.r).abs(),
        (a.g - b.g).abs(),
        (a.b - b.b).abs(),
    )
}
