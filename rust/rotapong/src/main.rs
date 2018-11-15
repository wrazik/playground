extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use rand::prelude::*;

 
const WHITE: [f32; 4] = [0.3, 0.3, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WIDTH: f64 = 640.;
const HEIGHT: f64 = 480.;

pub struct center {
    x: f64,
    y: f64
}

pub struct Sprite {
    center: center,
    velocity: [f64; 2],
    speed: f64
}

impl Sprite {

    fn up(&mut self) {
        self.velocity = [0., -self.speed]
    }
    fn down(&mut self) {
        self.velocity = [0., self.speed]
    }
    fn stop(&mut self) {
        self.velocity = [0., 0.]
    }

    fn update(&mut self) {
        self.center.x += self.velocity[0];

        if self.center.x < 0. {
            self.center.x = 0.
        }
        else if self.center.x > WIDTH {
            self.center.x = WIDTH;
        }

        self.center.y += self.velocity[1];
        if self.center.y < 0. {
            self.center.y = 0.
        }

        else if self.center.y > HEIGHT {
            self.center.y = HEIGHT;
        }
    }
}

pub struct Pad {
    sprite: Sprite,
    height: f64,
    width: f64
}

pub struct Ball {
    sprite: Sprite,
    radius: f64,
}

impl Ball {

    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = (self.sprite.center.x,
                      self.sprite.center.y);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            ellipse(
                WHITE,
                graphics::ellipse::circle(0.,0., self.radius), 
                transform, 
                gl
            );
        });
    }
    
    fn bounce_x(&mut self) {
       self.sprite.velocity[0] = -self.sprite.velocity[0]*1.1;

    }

    fn bounce_y(&mut self) {
       self.sprite.velocity[1] = -self.sprite.velocity[1];
    }

    fn update(&mut self) {
        self.sprite.update();
    }
    
    fn reset(&mut self) {
        self.sprite.center.x = 400.;
        self.sprite.center.y = 300.;
        let mut rng = thread_rng();
        let mut x_speed: f64 = rng.gen_range(0.5, 1.5);
        if rng.gen_range(0, 10) > 5 {
            x_speed = -x_speed;
        } 
        self.sprite.velocity[0] = x_speed;
        let mut y_speed: f64 = rng.gen_range(0.8, 1.3);
        if rng.gen_range(0, 10) > 5 {
            y_speed = -y_speed;
        } 
        self.sprite.velocity[1] = y_speed;
    }
}

impl Pad {

    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = (0., 0.);

        let pad: [f64; 4] = [self.sprite.center.x, self.sprite.center.y, self.width, self.height];

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            rectangle(WHITE, pad, transform, gl);
        });
    }

    fn up(&mut self) {
        self.sprite.up();
    }

    fn down(&mut self) {
        self.sprite.down();
    }

    fn stop(&mut self) {
        self.sprite.stop();
    }

    fn update(&mut self) {
        self.sprite.update();
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    left_pad: Pad,  
    right_pad: Pad,  
    ball: Ball,  
    width: f64,
    height: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });

        self.left_pad.draw(&mut self.gl, args);
        self.right_pad.draw(&mut self.gl, args);
        self.ball.draw(&mut self.gl, args);
    }

    fn score(&mut self) {
        self.ball.reset();
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.left_pad.update();
        self.right_pad.update();
        self.ball.update();

        let x = self.ball.sprite.center.x;
        let y = self.ball.sprite.center.y;
        let radius = self.ball.radius;

        let lx = self.left_pad.sprite.center.x;
        let ly = self.left_pad.sprite.center.y;
        let lh = self.left_pad.height;
        let lw = self.left_pad.width;

        let rx = self.right_pad.sprite.center.x;
        let ry = self.right_pad.sprite.center.y;
        let rh = self.right_pad.height;
        let rw = self.right_pad.width;
        
        if ((x - radius) < (lw + lx)) {
            if ((y > ly) && (y < (ly + lh))) {
                self.ball.bounce_x(); 
            }
            else {
                self.score();
            }
        }
        else if ((x + radius) > rx) {
            if ((y > ry) && (y < (ry + rh))) {
                self.ball.bounce_x(); 
            }
            else {
                self.score();
            }
        }


        if (self.ball.sprite.center.y < self.ball.radius ) {
            self.ball.bounce_y();
        }

        else if (self.ball.sprite.center.y + self.ball.radius > self.height) {
            self.ball.bounce_y();
        }

    }

    fn reset(&mut self) {
        self.ball.reset();
    }

    fn release(&mut self, key: &Button) {
        match key {
            Button::Keyboard(k) => {
                match k {
                    Key::W =>  {
                        self.left_pad.stop();
                    },
                    Key::S => { 
                        self.left_pad.stop();
                    },
                    Key::Up =>  {
                        self.right_pad.stop();
                    },
                    Key::Down => { 
                        self.right_pad.stop();
                    },
                    _ => {} 
                }
            }
            _ => {}
        }
    }

    fn key_pressed(&mut self, key: &Button) {
        // Rotate 2 radians per second.
        match key {
            Button::Keyboard(k) => {
                match k {
                    Key::W =>  {
                        self.left_pad.up();
                    },
                    Key::S => { 
                        self.left_pad.down();
                    },
                    Key::Up =>  {
                        self.right_pad.up();
                    },
                    Key::Down => { 
                        self.right_pad.down();
                    },
                    _ => {} 
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "rota-pong",
            [WIDTH as u32, HEIGHT as u32]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut rng = thread_rng();
    let mut x_speed: f64 = rng.gen_range(0.5, 1.5);
    if (rng.gen_range(0, 10) > 5) {
        x_speed = -x_speed;
    } 
    let mut y_speed: f64 = rng.gen_range(0.5, 1.);
    if (rng.gen_range(0, 10) > 5) {
        y_speed = -y_speed;
    } 

    let mut app = App {
        gl: GlGraphics::new(opengl),
        width: WIDTH, 
        height: HEIGHT,
        left_pad: Pad {
            sprite: Sprite {
                center: center { 
                    x: 20./2., 
                    y: HEIGHT/2. - 30.,
                },
                velocity: [0., 0.],
                speed: 3.0
            },
            height: 60.,
            width: 20.
        },
        right_pad: Pad {
            sprite: Sprite {
                center: center { 
                    x: WIDTH-20./2., 
                    y: HEIGHT/2. - 30.,
                },
                velocity: [0., 0.],
                speed: 3.0
            },
            height: 60.,
            width: 20.
        },
        ball: Ball {
            sprite: Sprite {
                center: center { 
                    x: WIDTH/2.,
                    y: HEIGHT/2.,
                },
                velocity: [x_speed, y_speed],
                speed: 3.0
            },
            radius: 10.0
        },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(k) = e.press_args() {
            app.key_pressed(&k);
        }
        if let Some(rel_args) = e.release_args() {
            app.release(&rel_args);
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
