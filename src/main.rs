mod app;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston_window;
extern crate find_folder;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston::input::Input::*;
use piston_window::PistonWindow as Window;
use piston_window::*;
use opengl_graphics::{GlGraphics, OpenGL};

use app::App;

const APP_WIDTH : u32 = 1024;
const APP_HEIGHT : u32 = 720;
const SCORE_SIZE : f64 = 72.0;
fn main() {

    let mut window : Window = WindowSettings::new("Pong in Rust!", [APP_WIDTH,APP_HEIGHT])
    .opengl(OpenGL::V4_1)
    .exit_on_esc(true)
    .resizable(false)
    .build()
    .unwrap();

    let assets_folder = find_folder::Search::ParentsThenKids(2,2).for_folder("fonts").unwrap();
    let font = assets_folder.join("noto_mono.ttf");
    let factory = window.factory.clone();
    let mut text_glyph = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let graphics = GlGraphics::new(OpenGL::V4_1);

    
    let mut app = App::new(graphics);

    while let Some(e) = window.next() {

        if let Some(r) = e.render_args() {
            app.render(&r);
            window.draw_2d(&e, |c, g| {
                let (r_pts, l_pts) = app.get_points();
                let points_str = format!("{} {}", r_pts, l_pts);
                let position = c.transform.trans(1024.0 / 2.0 - (SCORE_SIZE * points_str.len() as f64) / 3.0, SCORE_SIZE);
                text::Text::new_color([1.0, 1.0, 1.0, 0.38], 72)
                .draw(points_str.as_ref(), &mut text_glyph, &c.draw_state, position, g).unwrap();
            });
        }

        if let Some(u) = e.update_args() {
            app.update(&u, APP_WIDTH, APP_HEIGHT);
        }

        if let Some(p) = e.press_args() {
            app.handle_press(&p);
        }

        if let Some(p) = e.release_args() {
            app.handle_release(&p);
        }
    } 
}
