
// https://www.youtube.com/watch?v=U-X51GsTAzA
// Video explicativo sobre este software

use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use vector::Vector;

fn main() {
    
    let window: Window = Window::new_centered("Pendulo do Matheus Eduardo Aver", (800, 480)).unwrap();
    let win: MyWindowHandler = MyWindowHandler{
        p: Pendulum::new(400.0, 0.0, 200.0),
        p2: Pendulum::new(400.0, 0.0, 350.0)
    };

    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum
}

impl WindowHandler for MyWindowHandler{
    fn on_draw(
            &mut self,
            helper: &mut WindowHelper<()>,
            graphics: &mut Graphics2D
        ) {
        
        graphics.clear_screen(Color::from_rgb(0.0, 1.0, 0.5));

        self.p.update();
        self.p.draw(graphics);

        self.p2.update();
        self.p2.draw(graphics);

        helper.request_redraw();
    }
}
struct Pendulum{
    //This vector is the position of the pendulum
    origin: Vector,
    
    //This vector is the position of the ball
    position: Vector,

    //this is the angle of pendulum
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, //The length of the pendulum
    m: f32, //The mass of the ball
    g: f32, //The gravity
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum { 
            //We need to set the origin of the pendulum
            origin: Vector::new(x, y), 
            //We will set the position when we update the pendulum
            //For now we will set it to a default value
            position: Vector::new(0.0, 0.0),
            angle: 1.0,                 //We will set the angle to 1.0 radian
            angular_velocity: 0.0,      //The pendulum is not moving in the beginning
            angular_acceleration: 0.0,  //The pendulum is not accelerating in the beginning
            r: r, 
            m: 1.0, //The mass of the ball is 1.0 for example
            g: 1.5  //The gravity is 1.5 for this example
        }
    }

    fn update(&mut self) {
        //We use the pendulum equation to calculate the angular acceleration
        self.angular_acceleration = -1.0 * (1.0 / self.m) * self.g * self.angle.sin() / self.r;

        //The angular velocity is the angular velocity plus the angular acceleration
        self.angular_velocity += self.angular_acceleration;

        //The angle is the angle plus the angular velocity
        self.angle += self.angular_velocity;

        //The position is the polar coordinates translated to cartesian
        self.position.set(self.r * self.angle.sin(), self.r * self.angle.cos());

        //The final position of the ball in the canvas
        //Pendulum plus the position vector
        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
         (self.position.x, self.position.y), 
         3.0, 
         Color::RED
        );
        graphics.draw_circle(
            (self.position.x,self.position.y), 
            30.0, 
            Color::BLUE);
    }
}


mod vector{
    pub struct Vector{
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector{
            Vector{ x, y }
        }

        pub fn add(&mut self, other: &Vector) -> &Vector{
            self.x += other.x;
            self.y += other.y;
            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector{
            self.x = x;
            self.y = y;
            self
        }
    }
}