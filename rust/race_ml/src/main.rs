extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Position {
    x: f64,
    y: f64
}

pub struct Sprite {
    position: Position,
    speed: f64
}

impl Sprite {
    fn up(&mut self) {
        self.position.y -= 1.0*self.speed;
    }
    fn down(&mut self) {
        self.position.y += 1.0*self.speed;
    }
}

pub struct Pad {
    sprite: Sprite
}



impl Pad {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
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
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    left_pad: Pad,  
    right_pad: Pad,  
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

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        left_pad: Pad {
            sprite: Sprite {
                position: Position { 
                    x: 10.0, 
                    y: 280.0
                },
                speed: 30.0
            }
        },
        right_pad: Pad {
            sprite: Sprite {
                position: Position { 
                    x: 775.0, 
                    y: 280.0
                },
                speed: 30.0
            }
        },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(k) = e.press_args() {
            app.key_pressed(&k);
        }
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
    }
}
