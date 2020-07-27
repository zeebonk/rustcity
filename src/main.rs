extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod camera;
mod utils;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonState, MouseButton, RenderArgs, UpdateArgs};
use piston::window::WindowSettings;

use piston::{Event, Input, Loop, Motion};
use std::path::Path;

use camera::{Camera, ZoomDirection};

pub struct AppState {
    rotation: f64,
    texture: Texture,
}

fn render(app_state: &AppState, args: &RenderArgs, camera: &Camera, gl: &mut GlGraphics) {
    use graphics::*;

    const BACKGROUND: [f32; 4] = [1., 1., 1., 1.];

    gl.draw(args.viewport(), |c, gl| {
        clear(BACKGROUND, gl);

        let transform = c
            .transform
            .scale(camera.zoom(), camera.zoom())
            .trans(camera.x, camera.y);

        let tile_transform = transform
            .trans(300., 300.)
            .rot_rad(app_state.rotation)
            .trans(-50., -32.5);

        image(&app_state.texture, tile_transform, gl);
    });
}

fn update(app_state: &mut AppState, args: &UpdateArgs) {
    app_state.rotation += 2.0 * args.dt;
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("RustCity", [1024, 768])
        .graphics_api(opengl)
        .resizable(false)
        .vsync(true)
        .samples(4)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut camera = Camera::new();

    let mut app_state = AppState {
        rotation: 0.0,
        texture: Texture::from_path(Path::new("assets/slopeE.png"), &TextureSettings::new())
            .unwrap(),
    };

    let mut gl = GlGraphics::new(opengl);

    let mut left_down = false;
    let mut x = 0.;
    let mut y = 0.;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        match e {
            Event::Loop(Loop::Update(args)) => update(&mut app_state, &args),
            Event::Loop(Loop::Render(args)) => render(&app_state, &args, &camera, &mut gl),
            Event::Input(Input::Move(Motion::MouseScroll(args)), _) => {
                if args[1] > 0. {
                    camera.zoom_at(x, y, ZoomDirection::In);
                } else if args[1] < 0. {
                    camera.zoom_at(x, y, ZoomDirection::Out);
                }
            }
            Event::Input(Input::Move(Motion::MouseCursor(args)), _) => {
                x = args[0] as f64;
                y = args[1] as f64;
            }
            Event::Input(Input::Move(Motion::MouseRelative(args)), _) => {
                if left_down {
                    camera.x += args[0] / camera.zoom();
                    camera.y += args[1] / camera.zoom();
                }
            }
            Event::Input(Input::Button(args), _) => {
                if args.button == Button::Mouse(MouseButton::Left) {
                    left_down = args.state == ButtonState::Press;
                }
            }
            _ => {}
        };
    }
}
