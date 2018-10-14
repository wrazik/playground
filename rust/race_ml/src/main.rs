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
 

pub struct Position {
    x: f64,
    y: f64
}

pub struct Sprite {
    position: Position,
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
        self.position.x += self.velocity[0];

        if (self.position.x < 0.) {
            self.position.x = 0.
        }
        else if (self.position.x > 800.) {
            self.position.x = 800.;
        }
        self.position.y += self.velocity[1];
        if (self.position.y < 0.) {
            self.position.y = 0.
        }
        else if (self.position.y >  600.) {
            self.position.y = 600.;
        }
    }
}

pub struct Pad {
    sprite: Sprite
}

pub struct Ball {
    sprite: Sprite
}


impl Ball {
    const WHITE: [f32; 4] = [0.6, 0.0, 0.6, 1.0];

    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        let RADIUS = 10.0;

        let (x, y) = (self.sprite.position.x,
                      self.sprite.position.y);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            ellipse(
                Ball::WHITE,
                graphics::ellipse::circle(0.,0., 20.0), 
                transform, 
                gl
            );
        });
    }
    
    fn bounce_x(&mut self) {
       self.sprite.velocity[0] = -self.sprite.velocity[0];
    }

    fn bounce_y(&mut self) {
       self.sprite.velocity[1] = -self.sprite.velocity[1];
    }

    fn update(&mut self) {
        self.sprite.update();
    }
}

impl Pad {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const WHITE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    const PAD: [f64; 4] = [0.0, 0.0, 10.0, 50.0];

    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let (x, y) = (self.sprite.position.x,
                      self.sprite.position.y);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(x, y);

            rectangle(Pad::WHITE, Pad::PAD, transform, gl);
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
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });

        self.left_pad.draw(&mut self.gl, args);
        self.right_pad.draw(&mut self.gl, args);
        self.ball.draw(&mut self.gl, args);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.left_pad.update();
        self.right_pad.update();
        self.ball.update();
        if (self.ball.sprite.position.x > 780.) {
            self.ball.bounce_x();
        }
        if (self.ball.sprite.position.x < 10.) {
            self.ball.bounce_x();
        }
        if (self.ball.sprite.position.y < 10.) {
            self.ball.bounce_y();
        }

        if (self.ball.sprite.position.y > 580.) {
            self.ball.bounce_y();
        }
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
            "spinning-square",
            [800, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

//    let mut rng = thread_rng();
//    let x_speed: f64 = rng.gen_range(3., 5.);
//    let y_speed: f64 = rng.gen_range(-2., -3.);
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        left_pad: Pad {
            sprite: Sprite {
                position: Position { 
                    x: 10.0, 
                    y: 280.0
                },
                velocity: [0., 0.],
                speed: 3.0
            }
        },
        right_pad: Pad {
            sprite: Sprite {
                position: Position { 
                    x: 775.0, 
                    y: 280.0
                },
                velocity: [0., 0.],
                speed: 3.0
            }
        },
        ball: Ball {
            sprite: Sprite {
                position: Position { 
                    x: 200.0, 
                    y: 200.0
                },
                velocity: [2.0, 1.0],
                speed: 3.0
            }
        }
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(k) = e.press_args() {
            app.key_pressed(&k);
        }
        if let Some(rel_args) = e.release_args() {
            app.release(&rel_args)
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
