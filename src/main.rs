use crate::gpu_main::gpu_main;
use crate::projects::img_20230226_201501::img_20230226_201501;
use crate::projects::img_20230301_224920_two_images::img_20230301_224920_two_images;
use crate::projects::img_20230301_225057::img_20230301_225057;
use crate::projects::img_20231002_103537_three_images::img_20231002_103537_three_images;
use crate::projects::img_20240714_1958::img_20240714_1958;
use crate::projects::img_20241009_161110::img_20241009_161110;
use std::ops::Sub;
use std::time::Instant;

mod coord;
mod folding;
mod gpu;
mod gpu_main;
mod int;
mod projects;

fn main() {
    /*
    let cpu_before = Instant::now();
    img_20230226_201501();
    img_20230301_224920_two_images();
    img_20230301_225057();
    img_20231002_103537_three_images();
    img_20241009_161110();
    img_20240714_1958();
    // test_img_diffs();
    let cpu_duration = Instant::now().sub(cpu_before);
    println!("CPU duration: {}ms\n", cpu_duration.as_millis());
    */

    let gpu_before = Instant::now();
    pollster::block_on(gpu_main());
    let gpu_duration = Instant::now().sub(gpu_before);
    println!("GPU duration: {}ms\n", gpu_duration.as_millis());
}
