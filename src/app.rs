extern crate piston_window;

use piston::input::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use piston_window::*;
use math::*;

const ball_size : f64 = 25.0;
const paddle_height : f64 = 125.0;
const paddle_width : f64 = ball_size;
const paddle_move_speed : f64 = 1000.0;
const ball_move_speed: f64 = 300.0;
const increase_per_sec : f64 = 10.0;

fn clamp(val: f64, min: f64, max: f64) -> f64 {
    if val < min {
    println!("Clamping min");
        return min
    }
    if val > max {

    println!("Clamping max");
        return max
    }
    val
}

pub struct App {

    l_paddle_y_pos : Scalar,
    r_paddle_y_pos : Scalar,
    l_paddle_dir: Scalar,
    r_paddle_dir: Scalar,
    
    ball_x_dir: Scalar,
    ball_y_dir: Scalar,
    ball_pos: [Scalar; 2],

    l_points: u32,
    r_points: u32,

    ball_speed_delta: f64,

    gl: GlGraphics,

}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App {
            l_paddle_y_pos: 0.0,
            r_paddle_y_pos: 0.0,
            l_paddle_dir:0.0,
            r_paddle_dir:0.0,
            ball_x_dir: -1.0,
            ball_y_dir: 1.0,
            ball_pos: [0.0; 2],
            ball_speed_delta : 0.0,
            l_points: 0,
            r_points: 0,
            gl
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.draw_paddles(args);
    }

    fn draw_paddles(&mut self, args: &RenderArgs) {
        let ball_square = rectangle::rectangle_by_corners(0.0, 0.0, ball_size, ball_size);
        let paddle_figure = rectangle::rectangle_by_corners(0.0, 0.0, paddle_width, paddle_height);

        let b_x = self.ball_pos[0];
        let b_y = self.ball_pos[1];

        let render_x = (args.width / 2.0) - paddle_width;

        let lp_x = -render_x;
        let lp_y = self.l_paddle_y_pos;

        let rp_x = render_x;
        let rp_y = self.r_paddle_y_pos;

        self.gl.draw(args.viewport(), |c, g| {
            clear([0.0; 4], g);

            let ball_transform = c.transform.trans(b_x, b_y)
            .trans(args.width / 2.0, args.height / 2.0)
            .trans(-ball_size / 2.0, -ball_size / 2.0);

            let lpaddle_transform = c.transform.trans(lp_x, lp_y)
            .trans(args.width / 2.0, args.height / 2.0)
            .trans(-paddle_width / 2.0, -paddle_height / 2.0);
            
            let rpaddle_transform = c.transform.trans(rp_x, rp_y)
            .trans(args.width / 2.0, args.height / 2.0)
            .trans(-paddle_width / 2.0, -paddle_height / 2.0);

            rectangle(color::WHITE, ball_square, ball_transform, g);
            rectangle(color::WHITE, paddle_figure, lpaddle_transform, g);
            rectangle(color::WHITE, paddle_figure, rpaddle_transform, g);
        });
    }

    pub fn get_points(&self) -> (u32, u32) {
        (self.l_points, self.r_points)
    }


    fn move_paddles(&mut self, dt: f64, width: u32, height: u32) {
        let border : f64 = (height as f64 / 2.0) - (paddle_height / 2.0); 
        self.l_paddle_y_pos -= paddle_move_speed * dt * self.l_paddle_dir;
        self.r_paddle_y_pos -= paddle_move_speed * dt * self.r_paddle_dir;

        self.l_paddle_y_pos = clamp(self.l_paddle_y_pos, -border, border);
        self.r_paddle_y_pos = clamp(self.r_paddle_y_pos, -border, border);
        
    }

    fn move_ball(&mut self, dt: f64, width: u32, height: u32) {
        self.ball_pos[0] += (ball_move_speed + self.ball_speed_delta)* dt * self.ball_x_dir;
        self.ball_pos[1] += (ball_move_speed + self.ball_speed_delta) * dt * self.ball_y_dir;
        self.ball_speed_delta += increase_per_sec * dt;
    }

    pub fn update(&mut self, args : &UpdateArgs, width: u32, height: u32) {
        self.move_paddles(args.dt, width, height);
        self.move_ball(args.dt, width, height);
        self.check_collisions(width, height);
    }

    fn check_collisions(&mut self, width: u32, height: u32) {

        let ball_left = self.ball_pos[0] - ball_size / 2.0 ;
        let ball_right = self.ball_pos[0] + ball_size / 2.0;
        let ball_top = self.ball_pos[1] + ball_size / 2.0;
        let ball_bottom = self.ball_pos[1] - ball_size / 2.0;

        let r_paddle_front = width as f64 / 2.0 - paddle_width;
        let r_paddle_bottom = self.r_paddle_y_pos - paddle_height / 2.0;
        let r_paddle_top = self.r_paddle_y_pos + paddle_height / 2.0;

        let l_paddle_front = -(width as f64 / 2.0 - paddle_width);
        let l_paddle_bottom = self.l_paddle_y_pos - paddle_height / 2.0;
        let l_paddle_top = self.l_paddle_y_pos + paddle_height / 2.0;

        let border : f64 = (height as f64 / 2.0) - (ball_size ); // Change this

        if self.ball_pos[1] > border {
            self.ball_y_dir = -1.0;
        }
        if self.ball_pos[1] < -border {
            self.ball_y_dir = 1.0;
        }

        if ball_right > r_paddle_front && ball_top <= r_paddle_top && ball_bottom >= r_paddle_bottom {
            self.ball_x_dir = -1.0;
        }
        if ball_left < l_paddle_front && ball_top <= l_paddle_top && ball_bottom >= l_paddle_bottom {
            self.ball_x_dir = 1.0;
        }

        if ball_left < - (width as f64 / 2.0) {
            self.ball_pos[0] = 0.0;
            self.ball_pos[1] = 0.0;
            self.r_points += 1;
            self.ball_speed_delta = 0.0;
        }   
  
        if ball_right > (width as f64 / 2.0) {
            self.ball_pos[0] = 0.0;
            self.ball_pos[1] = 0.0;
            self.l_points += 1;
            self.ball_speed_delta = 0.0;
        }
    }

    pub fn handle_press(&mut self, args: &Button) {
        use Key::*;
        if let Button::Keyboard(k) = args {
            match k {
                W => { self.l_paddle_dir = 1.0 },
                S => { self.l_paddle_dir = -1.0 },
                Up => { self.r_paddle_dir = 1.0 },
                Down => { self.r_paddle_dir = -1.0 },
                _ => {} 
            }
        }
    }

    pub fn handle_release(&mut self, args: &Button) {
        use Key::*;
        if let Button::Keyboard(k) = args {
            match k {
                W | S => { self.l_paddle_dir = 0.0 },
                Up | Down => { self.r_paddle_dir = 0.0 },
                _ => {} 
            }
        }
    }
}
