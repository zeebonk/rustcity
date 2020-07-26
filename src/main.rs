extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod camera;
mod utils;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, MouseButton, MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent, PressEvent,
    ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;

use camera::{Camera, ZoomDirection};

pub struct App {
    gl: GlGraphics,
    rotation: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs, camera: &Camera) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [1., 1., 1., 1.];
        const FOREGROUND: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 300.0);
        let (square_x, square_y) = (300., 300.);
        let rotation = self.rotation;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);

            let transform = c
                .transform
                .scale(camera.zoom(), camera.zoom())
                .trans(camera.x, camera.y);

            let square_transform = transform
                .trans(square_x, square_y)
                .rot_rad(rotation)
                .trans(-150.0, -150.0);

            rectangle(FOREGROUND, square, square_transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("RustCity", [1024, 768])
        .graphics_api(opengl)
        .resizable(false)
        .vsync(true)
        .samples(4)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut camera = Camera::new();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut left_down = false;
    let mut x = 0.;
    let mut y = 0.;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &camera);
        } else if let Some(args) = e.update_args() {
            app.update(&args);
        } else if let Some(args) = e.mouse_relative_args() {
            if left_down {
                camera.x += args[0];
                camera.y += args[1];
            }
        } else if let Some(Button::Mouse(b)) = e.press_args() {
            if b == MouseButton::Left {
                left_down = true;
            }
        } else if let Some(Button::Mouse(b)) = e.release_args() {
            if b == MouseButton::Left {
                left_down = false;
            }
        } else if let Some(args) = e.mouse_scroll_args() {
            if args[1] > 0. {
                camera.zoom_at(x, y, ZoomDirection::In);
            } else if args[1] < 0. {
                camera.zoom_at(x, y, ZoomDirection::Out);
            }
        } else if let Some(args) = e.mouse_cursor_args() {
            x = args[0] as f64;
            y = args[1] as f64;
        }
    }
}
