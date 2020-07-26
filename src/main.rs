extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, MouseButton, MouseRelativeEvent, PressEvent, ReleaseEvent, RenderArgs, RenderEvent,
    UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    camera_offset: [f64; 2],
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [1., 1., 1., 1.];
        const FOREGROUND: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 300.0);
        let (square_x, square_y) = (300., 300.);
        let rotation = self.rotation;
        let (camera_x, camera_y) = (self.camera_offset[0], self.camera_offset[1]);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);

            let transform = c.transform.trans(-camera_x, -camera_y);

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

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        camera_offset: [0., 0.],
    };

    let mut left_down = false;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.mouse_relative_args() {
            if left_down {
                app.camera_offset = [
                    app.camera_offset[0] - args[0],
                    app.camera_offset[1] - args[1],
                ];
            }
        }

        if let Some(Button::Mouse(b)) = e.press_args() {
            if b == MouseButton::Left {
                left_down = true;
            }
        }

        if let Some(Button::Mouse(b)) = e.release_args() {
            if b == MouseButton::Left {
                left_down = false;
            }
        }
    }
}
