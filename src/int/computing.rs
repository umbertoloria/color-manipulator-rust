use crate::int::int_color::Color;
use crate::int::int_layer::{create_canvas_layer, CanvasLayer, IntAbsLayer};

pub fn create_mean_canvas_layer_from_int_layer(layer: &dyn IntAbsLayer) -> CanvasLayer {
    let mut canvas = create_canvas_layer(layer.width(), layer.height());
    // Top Left
    {
        let x = 0;
        let y = 0;
        canvas.set_color(
            x,
            y,
            color_mean(&[
                layer.get_color(x + 1, y),     // Right
                layer.get_color(x + 1, y + 1), // Right Bottom
                layer.get_color(x, y + 1),     // Bottom
            ]),
        );
    }
    // Top Right
    {
        let x = canvas.width() - 1;
        let y = 0;
        canvas.set_color(
            x,
            y,
            color_mean(&[
                layer.get_color(x - 1, y),     // Left
                layer.get_color(x - 1, y + 1), // Left Bottom
                layer.get_color(x, y + 1),     // Bottom
            ]),
        );
    }
    // Bottom Right
    let x = canvas.width() - 1;
    let y = canvas.height() - 1;
    canvas.set_color(
        x,
        y,
        color_mean(&[
            layer.get_color(x, y - 1),     // Top
            layer.get_color(x - 1, y - 1), // Top Left
            layer.get_color(x - 1, y),     // Left
        ]),
    );
    // Bottom Left
    let x = 0;
    let y = canvas.height() - 1;
    canvas.set_color(
        x,
        y,
        color_mean(&[
            layer.get_color(x, y - 1),     // Top
            layer.get_color(x + 1, y - 1), // Top Right
            layer.get_color(x + 1, y),     // Right
        ]),
    );
    // Top Corner
    let y = 0;
    for x in 1..canvas.width() - 1 {
        canvas.set_color(
            x,
            y,
            color_mean(&[
                layer.get_color(x - 1, y),
                layer.get_color(x - 1, y + 1),
                layer.get_color(x, y + 1),
                layer.get_color(x + 1, y + 1),
                layer.get_color(x + 1, y),
            ]),
        );
    }
    // Right Corner
    let x = canvas.width() - 1;
    for y in 1..canvas.height() - 1 {
        canvas.set_color(
            x,
            y,
            color_mean(&[
                layer.get_color(x, y - 1),
                layer.get_color(x - 1, y - 1),
                layer.get_color(x - 1, y),
                layer.get_color(x - 1, y + 1),
                layer.get_color(x, y + 1),
            ]),
        );
    }
    // Bottom Corner
    let y = canvas.height() - 1;
    for x in 1..canvas.width() - 1 {
        canvas.set_color(
            x,
            y,
            color_mean(&[
                layer.get_color(x - 1, y),
                layer.get_color(x - 1, y - 1),
                layer.get_color(x, y - 1),
                layer.get_color(x + 1, y - 1),
                layer.get_color(x + 1, y),
            ]),
        );
    }
    // Left Corner
    for y in 1..canvas.height() - 1 {
        let x = 0;
        let c = color_mean(&[
            layer.get_color(x, y - 1),
            layer.get_color(x + 1, y - 1),
            layer.get_color(x + 1, y),
            layer.get_color(x + 1, y + 1),
            layer.get_color(x, y + 1),
        ]);
        canvas.set_color(x, y, c);
    }
    // All other colors...
    for x in 1..canvas.width() - 1 {
        for y in 1..canvas.height() - 1 {
            canvas.set_color(
                x,
                y,
                color_mean(&[
                    layer.get_color(x - 1, y - 1),
                    layer.get_color(x, y - 1),
                    layer.get_color(x + 1, y - 1),
                    layer.get_color(x - 1, y),
                    layer.get_color(x, y),
                    layer.get_color(x + 1, y),
                    layer.get_color(x - 1, y + 1),
                    layer.get_color(x, y + 1),
                    layer.get_color(x + 1, y + 1),
                ]),
            );
        }
    }
    canvas
}

pub fn color_mean(c_list: &[Color]) -> Color {
    let mut mean_r: u128 = 0;
    let mut mean_g: u128 = 0;
    let mut mean_b: u128 = 0;
    for c in c_list {
        mean_r += c.r as u128;
        mean_g += c.g as u128;
        mean_b += c.b as u128;
    }
    let mean_r = (mean_r as f64 / c_list.len() as f64).round() as u8;
    let mean_g = (mean_g as f64 / c_list.len() as f64).round() as u8;
    let mean_b = (mean_b as f64 / c_list.len() as f64).round() as u8;
    Color {
        r: mean_r,
        g: mean_g,
        b: mean_b,
    }
}
