extern crate kiss3d;

use kiss3d::camera;
use kiss3d::camera::Camera;
use kiss3d::event::{Action, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra::{Matrix4, Point2, Point3, Translation2, Translation3, Vector2};
use kiss3d::planar_camera;
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::window::Window;

use std::f64::consts;
use std::sync::Arc;

/* TODO:
GUI - equation, range, step
plane intersection
*/

fn main() {
    let eye = Point3::new(10.0f32, 10.0, 10.0);
    let at = Point3::origin();
    let mut window = Window::new("3D Grapher");
    let camera = camera::FirstPerson::new(eye, at);
    let camera_2d = planar_camera::FixedView::new();
    window.set_light(Light::StickToCamera);
    let mut last_pos = (0.0f32, 0.0f32);
    let mut sel_pos = (0.0f32, 0.0f32);

    let mut nodes: Vec<kiss3d::scene::PlanarSceneNode> = vec![];

    let x_range: (f64, f64) = (-5.0, 5.0);
    let y_range: (f64, f64) = (-5.0, 5.0);
    let x_step: f64 = 0.1;
    let y_step: f64 = 0.1;

    let button_size = 50.0;
    let mut uibutton = UIButton::new(
        (-button_size / 2.0, -button_size / 2.0),
        (button_size, button_size),
        Arc::new(|_| println!("pushed")),
    );

    if x_range.0 > x_range.1 || y_range.0 > y_range.1 || x_step <= 0.0 || y_step <= 0.0 {
        panic!("invalid properties");
    }
    plot_point(0.5, 0.5, &mut window);
    uibutton.draw(&mut window, &mut nodes);
    while window.render() {
        clear_ui(&mut window, &mut nodes);
        draw_origin_lines(x_range, x_step, y_range, x_step, &mut window);
        plot_points(x_range, x_step, y_range, y_step, &mut window);
        draw_lines(x_range, x_step, y_range, y_step, &mut window);
        uibutton.draw(&mut window, &mut nodes);
        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::CursorPos(x, y, _modif) => {
                    println!("{} {}", x, y);
                    let window_size = (window.size()[0] as f32, window.size()[1] as f32);
                    last_pos = (x as f32, y as f32);
                    sel_pos = (last_pos.0 - window_size.0, last_pos.1 - window_size.1);
                }
                WindowEvent::MouseButton(button, Action::Press, modif) => {
                    let window_size = window.size().data.0[0];
                    sel_pos = (
                        last_pos.0 - window_size[0] as f32 / 2.0,
                        -last_pos.1 + window_size[1] as f32 / 2.0,
                    );
                    println!("{:?}", sel_pos);
                    uibutton.check_click(sel_pos, modif);
                }
                _ => {}
            }
        }
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
                &Point3::new(0.5, 0.5, 0.5),
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
                &Point3::new(0.5, 0.5, 0.5),
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
}

fn ui(window: &mut Window, nodes: &mut Vec<kiss3d::scene::PlanarSceneNode>) {}

fn clear_ui(window: &mut Window, nodes: &mut Vec<kiss3d::scene::PlanarSceneNode>) {
    for i in 0..nodes.len() {
        window.remove_planar_node(&mut nodes[i]);
    }
    nodes.clear();
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

    // x.powi(2) - y.powi(2) // pringle

    consts::E.powf(x) * y.sin() // crazy
                                // (x.powi(2) + y.powi(2)).powf(x * y) - 0.75

    // (x * y.powi(2)).cos() / (x - y)
    // x.powi(2) - 2.0 * x * y + 2.0 * y
}

struct UIButton {
    center: (f32, f32),
    size: (f32, f32),
    func: Arc<dyn Fn(&mut UIButton) -> ()>,
    pos: (f32, f32),
    skewed_pos: (f32, f32),
}
impl UIButton {
    pub fn new(
        center: (f32, f32),
        size: (f32, f32),
        func: Arc<dyn Fn(&mut UIButton) -> ()>,
    ) -> UIButton {
        UIButton {
            center,
            size,
            func,
            pos: (0.0, 0.0),
            skewed_pos: (0.0, 0.0),
        }
    }
    pub fn draw(&mut self, window: &mut Window, nodes: &mut Vec<kiss3d::scene::PlanarSceneNode>) {
        let mut c = window.add_rectangle(self.size.0, self.size.1);
        let a1 = 0.4;
        let a2 = 0.506667;
        let a3 = 0.447143;
        let size = window.size().data.0[0];
        self.pos = (
            0.5 * size[0] as f32 + self.center.0,
            0.5 * size[1] as f32 + self.center.1,
        );
        self.skewed_pos = (
            a1 * size[0] as f32 + self.center.0,
            a1 * size[1] as f32 + self.center.1,
        );
        c.append_translation(&Translation2::new(self.skewed_pos.0, self.skewed_pos.1));
        nodes.push(c);
    }
    pub fn check_click(&mut self, pos: (f32, f32), modif: kiss3d::event::Modifiers) {
        let corners = self.corners();
        println!("{:?}", corners);
        if (pos.0 > corners.0 .0 && pos.0 < corners.1 .0)
            && (pos.1 > corners.0 .1 && pos.1 < corners.1 .1)
        {
            let f = self.func.clone();
            (f)(self);
        }
    }
    fn corners(&self) -> ((f32, f32), (f32, f32)) {
        (
            (
                self.pos.0 - self.size.0 * 0.75,
                self.pos.1 - self.size.1 * 0.75,
            ),
            (
                self.pos.0 + self.size.0 / 2.0,
                self.pos.1 + self.size.1 / 2.0,
            ),
        )
    }
}
