extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, MouseButton, MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent, PressEvent,
    ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

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
                .scale(camera.zoom, camera.zoom)
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

enum ZoomDirection {
    In,
    Out,
}

pub struct Camera {
    x: f64,
    y: f64,
    zoom: f64,
    step: f64,
}

impl Camera {
    const MIN_ZOOM: f64 = 0.125;
    const MAX_ZOOM: f64 = 8.;
    const ZOOM_STEPS: f64 = 101.;

    fn new() -> Camera {
        Camera {
            x: 0.,
            y: 0.,
            zoom: 1.,
            step: 51.,
        }
    }

    fn zoom_at(&mut self, x: f64, y: f64, zoom: ZoomDirection) {
        let old_zoom = self.zoom;

        self.step += match zoom {
            ZoomDirection::In => 1.,
            ZoomDirection::Out => -1.,
        };

        self.step = clamp(self.step, 0., Self::ZOOM_STEPS - 1.);

        let ln_min_zoom = f64::ln(Self::MIN_ZOOM);
        let ln_max_zoom = f64::ln(Self::MAX_ZOOM);

        self.zoom = f64::exp(
            ln_min_zoom + (ln_max_zoom - ln_min_zoom) * self.step / (Self::ZOOM_STEPS - 1.0)
        );

        self.x -= (x / old_zoom) - (x / self.zoom);
        self.y -= (y / old_zoom) - (y / self.zoom);
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
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.mouse_relative_args() {
            if left_down {
                camera.x += args[0];
                camera.y += args[1];
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

        if let Some(args) = e.mouse_scroll_args() {
            if args[1] > 0. {
                camera.zoom_at(x, y, ZoomDirection::In);
            } else if args[1] < 0. {
                camera.zoom_at(x, y, ZoomDirection::Out);
            }
        }

        if let Some(args) = e.mouse_cursor_args() {
            x = args[0] as f64;
            y = args[1] as f64;
        }
    }
}
