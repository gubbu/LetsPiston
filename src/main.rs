extern crate piston_window;
mod gamewindow;
mod fontrender;

struct Gametest{alive: f64}

//const MESSAGE: String = String::from("HELLO WORLD!+-*/ WORLD: HOW ARE YOU\nI AM NOT OK.");

impl gamewindow::Gametrait for Gametest{
    fn onstart(&mut self){println!("starting")}

    fn update(&mut self, dt: f64){
        //println!("delta time {}", dt)
        self.alive += dt*5.0;
    }

    fn render(&self, g: &mut piston_window::G2d, transform: [[f64; 3]; 2]){
        println!("transform {:?}", transform);
        piston_window::clear([0.0;4], g);
        fontrender::drawstring("TEST=35/5=7", self.alive, self.alive, 2.0, 4.0, [0.0, 1.0, 0.0, 0.1], transform, g);
        fontrender::drawstring("HELLO\n33", 10.0, 10.0, 2.0, 4.0, [0.25;4], transform, g);
        fontrender::drawstring("TEST:   35/5=7", self.alive, self.alive+10., 2.0, 4.0, [1.0, 0.0, 0.0, 0.1], transform, g);
    }

    fn shouldquit(&self)->bool{return false;}
    fn onquit(&mut self){println!("QUITING!");}
    fn keyboard(&mut self, ispressed: bool, keychar: char){
        if ispressed{
            println!("you are pressing {}", keychar);
        }
    }    
}

fn main() {
    gamewindow::makegame("hello world", [400,100], 30, 30, &mut Gametest{alive: 0.0});
}
