use color_manipulator_rust::POS;

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}
impl Coord {
    pub fn to_pos(&self, width: usize, height: usize) -> POS {
        POS {
            x: self.x as f32 / width as f32,
            y: self.y as f32 / height as f32,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct CoordSquare {
    // Both incl.
    pub top_left: Coord,
    pub bottom_right: Coord,
}
impl CoordSquare {
    pub fn get_width(&self) -> usize {
        self.bottom_right.x - self.top_left.x + 1
    }
    pub fn get_height(&self) -> usize {
        self.bottom_right.y - self.top_left.y + 1
    }
}

pub fn get_coord_chunks(chunks_count: usize, width: usize, height: usize) -> Vec<CoordSquare> {
    let mut result: Vec<CoordSquare> = Vec::new();

    let chunk_width_approx = width / chunks_count; // Floor.

    let mut next_x_ready = 0;
    for _ in 0..(chunks_count - 1) {
        let prev_x = next_x_ready;
        next_x_ready += chunk_width_approx;
        result.push(CoordSquare {
            top_left: Coord { x: prev_x, y: 0 },
            bottom_right: Coord {
                x: next_x_ready - 1,
                y: height - 1,
            },
        });
    }
    result.push(CoordSquare {
        top_left: Coord {
            x: next_x_ready,
            y: 0,
        },
        bottom_right: Coord {
            x: width - 1,
            y: height - 1,
        },
    });

    /*
    // For debug only.
    for result_item in &result {
        println!(
            "{:?}, width={}, height={}",
            result_item,
            result_item.get_width(),
            result_item.get_height()
        );
    }
    */

    result
}

pub fn get_distance_coord(pos: Coord, center: Coord) -> usize {
    let diff_x = (pos.x - center.x) as f64;
    let diff_y = (pos.y - center.y) as f64;
    (diff_x * diff_x + diff_y * diff_y).sqrt() as usize
}
