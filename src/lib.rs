use regex::Regex;

#[derive(Clone, Copy, Debug)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub fn safe_color(c: f32) -> f32 {
    filter_scalar(c, 0.0, 255.0)
}

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
    RGB { r, g, b }
}
pub fn color_hex(hex: &str) -> RGB {
    let hex = hex.trim_start_matches('#');

    let re_str = r"^([a-fA-F0-9]{2})([a-fA-F0-9]{2})([a-fA-F0-9]{2})$";
    let re = Regex::new(re_str).unwrap();

    let caps = re.captures(hex).ok_or("Invalid HEX color").unwrap();

    let r_str = caps.get(1).unwrap().as_str();
    let g_str = caps.get(2).unwrap().as_str();
    let b_str = caps.get(3).unwrap().as_str();

    let r = u8::from_str_radix(r_str, 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(g_str, 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(b_str, 16).unwrap() as f32 / 255.0;

    RGB { r, g, b }
}

pub fn add(a: RGB, b: RGB) -> RGB {
    RGB {
        r: a.r + b.r,
        g: a.g + b.g,
        b: a.b + b.b,
    }
}
pub fn sub(a: RGB, b: RGB) -> RGB {
    add(
        a,
        RGB {
            r: -b.r,
            g: -b.g,
            b: -b.b,
        },
    )
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
    RGB { r, g: 0.0, b: 0.0 }
}
pub fn only_g(g: f32) -> RGB {
    RGB { r: 0.0, g, b: 0.0 }
}
pub fn only_b(b: f32) -> RGB {
    RGB { r: 0.0, g: 0.0, b }
}
pub fn get_r(c: RGB) -> f32 {
    c.r
}
pub fn get_g(c: RGB) -> f32 {
    c.g
}
pub fn get_b(c: RGB) -> f32 {
    c.b
}
pub fn add_list(c_list: &[Option<RGB>]) -> RGB {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    for c in c_list {
        if let Some(rgb) = c {
            r += rgb.r;
            g += rgb.g;
            b += rgb.b;
        }
    }
    RGB { r, g, b }
}

pub fn xor_color(a: RGB, b: RGB) -> RGB {
    color((a.r - b.r).abs(), (a.g - b.g).abs(), (a.b - b.b).abs())
}

/// POSITION
#[derive(Clone, Copy)]
pub struct POS {
    pub x: f32,
    pub y: f32,
}

pub fn gradient_linear(pos: f32, from: f32, to: f32) -> f32 {
    let dist_prop = (pos - from) / (to - from);
    if dist_prop > 1.0 {
        return 1.0;
    }
    if dist_prop < 0.0 {
        return 0.0;
    }
    dist_prop
}

pub fn white() -> RGB {
    scalar(1.0)
}
pub fn black() -> RGB {
    scalar(0.0)
}
pub fn zero_but_one_in_square(pos: POS, top_left: POS, bottom_right: POS) -> f32 {
    if top_left.x <= pos.x
        && pos.x < bottom_right.x
        && top_left.y <= pos.y
        && pos.y < bottom_right.y
    {
        1.0
    } else {
        0.0
    }
}
pub fn get_distance(pos: POS, center: POS) -> f32 {
    let diff_x = pos.x - center.x;
    let diff_y = pos.y - center.y;
    f32::sqrt(diff_x * diff_x + diff_y * diff_y)
}
pub fn zero_but_one_in_circle(pos: POS, center: POS, radius: f32) -> f32 {
    if get_distance(pos, center) <= radius {
        1.0
    } else {
        0.0
    }
}
