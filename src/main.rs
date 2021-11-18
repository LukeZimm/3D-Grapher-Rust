extern crate kiss3d;

use kiss3d::camera;
// use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3};
use kiss3d::window::Window;

use std::f64::consts;

/* TODO:
GUI - equation, range, step
plane intersection
*/

fn main() {
    let eye = Point3::new(10.0f32, 10.0, 10.0);
    let at = Point3::origin();
    let mut window = Window::new("3D Grapher");
    let _camera = camera::FirstPerson::new(eye, at);
    window.set_light(Light::StickToCamera);

    let x_range: (f64, f64) = (-1.0, 1.0);
    let y_range: (f64, f64) = (-1.0, 1.0);
    let x_step: f64 = 0.05;
    let y_step: f64 = 0.05;

    if x_range.0 > x_range.1 || y_range.0 > y_range.1 || x_step <= 0.0 || y_step <= 0.0 {
        panic!("invalid properties");
    }
    plot_point(0.5, 0.5, &mut window);

    while window.render() {
        // origin lines
        draw_origin_lines(x_range, x_step, y_range, x_step, &mut window);
        plot_points(x_range, x_step, y_range, y_step, &mut window);
        draw_lines(x_range, x_step, y_range, y_step, &mut window);
    }
}

fn plot_points(
    x_range: (f64, f64),
    x_step: f64,
    y_range: (f64, f64),
    y_step: f64,
    window: &mut Window,
) {
    for i in 0..((x_range.1 - x_range.0) / x_step) as i32 {
        let x = x_range.0 + i as f64 * x_step;
        for j in 0..((y_range.1 - y_range.0) / y_step) as i32 {
            let y = y_range.0 + j as f64 * y_step;
            let z = function(x, y);
            window.draw_point(
                &Point3::new(x as f32, z as f32, y as f32),
                &Point3::new(1.0, 1.0, 1.0),
            );
        }
    }
}
fn draw_lines(
    x_range: (f64, f64),
    x_step: f64,
    y_range: (f64, f64),
    y_step: f64,
    window: &mut Window,
) {
    let mut cached_x: f64 = x_range.0;
    let mut cached_y: f64;
    let mut cached_z: f64;
    for i in 1..((x_range.1 - x_range.0) / x_step) as i32 {
        let x = x_range.0 + i as f64 * x_step;
        cached_y = y_range.0;
        cached_z = function(x - x_step, y_range.0);
        for j in 0..((y_range.1 - y_range.0) / y_step) as i32 {
            let y = y_range.0 + j as f64 * y_step;
            let z = function(x, y);
            window.draw_line(
                &Point3::new(x as f32, z as f32, y as f32),
                &Point3::new(cached_x as f32, cached_z as f32, cached_y as f32),
                &Point3::new(1.0, 1.0, 1.0),
            );
            cached_x = x;
            cached_y = y;
            cached_z = z;
        }
    }
    let mut cached_x: f64;
    let mut cached_y: f64 = y_range.0;
    let mut cached_z: f64;
    for j in 1..((y_range.1 - y_range.0) / y_step) as i32 {
        let y = y_range.0 + j as f64 * y_step;
        cached_x = x_range.0;
        cached_z = function(x_range.0, y - y_step);
        for i in 0..((x_range.1 - x_range.0) / x_step) as i32 {
            let x = x_range.0 + i as f64 * x_step;
            let z = function(x, y);
            window.draw_line(
                &Point3::new(x as f32, z as f32, y as f32),
                &Point3::new(cached_x as f32, cached_z as f32, cached_y as f32),
                &Point3::new(1.0, 1.0, 1.0),
            );
            cached_x = x;
            cached_y = y;
            cached_z = z;
        }
    }
}

fn draw_origin_lines(
    x_range: (f64, f64),
    x_step: f64,
    y_range: (f64, f64),
    y_step: f64,
    window: &mut Window,
) {
    let z_range: (f64, f64) = ((x_range.0 + y_range.0) / 2.0, (x_range.1 + y_range.1) / 2.0);
    for a in 0..4 {
        window.draw_line(
            &Point3::new(
                if a == 0 { x_range.0 as f32 } else { 0.0 },
                if a == 2 { z_range.0 as f32 } else { 0.0 },
                if a == 1 { y_range.0 as f32 } else { 0.0 },
            ),
            &Point3::new(
                if a == 0 { x_range.1 as f32 } else { 0.0 },
                if a == 2 { z_range.1 as f32 } else { 0.0 },
                if a == 1 { y_range.1 as f32 } else { 0.0 },
            ),
            &Point3::new(
                if a == 0 { 1.0 } else { 0.0 },
                if a == 1 { 1.0 } else { 0.0 },
                if a == 2 { 1.0 } else { 0.0 },
            ),
        );
        let qty = if a == 0 {
            ((x_range.1 - x_range.0) / x_step / 5.0) as i32
        } else if a == 1 {
            ((y_range.1 - y_range.0) / y_step / 5.0) as i32
        } else {
            ((z_range.1 - z_range.0) / (x_step + y_step) / 10.0) as i32
        };
        for i in 0..((x_range.1 - x_range.0) / x_step / 5.0) as i32 as i32 + 1 {
            window.draw_line(
                &Point3::new(
                    if a == 0 {
                        (x_range.0 + x_step * i as f64 * 5.0) as f32
                    } else if a == 2 {
                        0.0
                    } else {
                        -0.025
                    },
                    if a == 0 || a == 1 {
                        0.0
                    } else {
                        (z_range.0 + (x_step + y_step) / 2.0 * i as f64 * 5.0) as f32
                    },
                    if a == 0 || a == 2 {
                        -0.025
                    } else if a == 1 {
                        (y_range.0 + y_step * i as f64 * 5.0) as f32
                    } else {
                        0.0
                    },
                ),
                &Point3::new(
                    if a == 0 {
                        (x_range.0 + x_step * i as f64 * 5.0) as f32
                    } else if a == 2 {
                        0.0
                    } else {
                        0.025
                    },
                    if a == 0 || a == 1 {
                        0.0
                    } else {
                        (z_range.0 + (x_step + y_step) / 2.0 * i as f64 * 5.0) as f32
                    },
                    if a == 0 || a == 2 {
                        0.025
                    } else if a == 1 {
                        (y_range.0 + y_step * i as f64 * 5.0) as f32
                    } else {
                        0.0
                    },
                ),
                &Point3::new(
                    if a == 0 { 1.0 } else { 0.0 },
                    if a == 1 { 1.0 } else { 0.0 },
                    if a == 2 || a == 3 { 1.0 } else { 0.0 },
                ),
            );
        }
    }
    /* window.draw_line(
        &Point3::new(x_range.0 as f32, 0.0, 0.0),
        &Point3::new(x_range.1 as f32, 0.0, 0.0),
        &Point3::new(1.0, 0.0, 0.0),
    );
    for i in 0..((x_range.1 - x_range.0) / x_step / 5.0) as i32 {
        let x = x_range.0 + x_step * i as f64 * 5.0;
        window.draw_line(
            &Point3::new(x as f32, 0.0, -0.025),
            &Point3::new(x as f32, 0.0, 0.025),
            &Point3::new(1.0, 0.0, 0.0),
        );
    }
    window.draw_line(
        &Point3::new(0.0, 0.0, y_range.0 as f32),
        &Point3::new(0.0, 0.0, y_range.1 as f32),
        &Point3::new(0.0, 1.0, 0.0),
    );
    window.draw_line(
        &Point3::new(0.0, (x_range.0 + y_range.0) as f32 / 2.0, 0.0),
        &Point3::new(0.0, (x_range.1 + y_range.1) as f32 / 2.0, 0.0),
        &Point3::new(0.0, 0.0, 1.0),
    ); */
}

fn plot_point(x: f64, y: f64, window: &mut Window) {
    let z = function(x, y);
    let mut c = window.add_sphere(0.015);
    c.append_translation(&Translation3::new(x as f32, z as f32, y as f32));
    c.set_color(1.0, 0.0, 0.0);
}

fn function(x: f64, y: f64) -> f64 {
    // 1.0 / (15.0 * (x.powi(2) + y.powi(2))) // tube

    // (0.4f64.powi(2) - (0.6 - (x.powi(2) + y.powi(2)).powf(0.5)).powi(2)).powf(0.5)
    // torus

    x.powi(2) - y.powi(2) // pringle

    // consts::E.powf(x) * y.sin() // crazy
    // (x.powi(2) + y.powi(2)).powf(x * y) - 0.75
}
