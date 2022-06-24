use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub author: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub text: String,
    pub author: String,
    pub message_text: String,
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
    color: String,
    border_width: f64,
    border_color: String
}

#[derive(Clone, Copy)]
struct Constraints {
    border: f64,
    width: f64,
    height: f64,
    max_velocity: f64
}

#[derive(Clone, Copy)]
struct Cow {
    //nick_name: String,
    x: f64,
    y: f64,
    constraints: Constraints,
    x_velocity: f64,
    y_velocity: f64
}


impl Cow {
    fn moving(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        if self.x <= self.constraints.border || self.x >= self.constraints.width - self.constraints.border {
            self.x -= self.x_velocity;
            self.x = self.x.max(self.constraints.border);
            self.x = self.x.min(self.constraints.width - self.constraints.border);
            self.x_velocity = -self.x_velocity;
            self.x += self.x_velocity;
        }

        if self.y <= self.constraints.border || self.y >= self.constraints.height - self.constraints.border {
            self.y -= self.y_velocity;
            self.y = self.y.max(self.constraints.border);
            self.y = self.y.min(self.constraints.height - self.constraints.border);
            self.y_velocity = -self.y_velocity;
            self.y += self.y_velocity
        }

    }

    fn distance(&mut self, boid: Cow) -> f64 {
        let dist_x = self.x - boid.x;
        let dist_y = self.y - boid.x;
        (dist_x*dist_x + dist_y*dist_y).sqrt()
    }

    fn move_away(&mut self, boids: Vec<Cow>, min_distance: f64) {
        let mut distance_x = 0.0;
        let mut distance_y = 0.0;
        let mut num_close = 0.0;

        for boid in boids.iter() {

            if boid.x == self.x && boid.y == self.y {
                continue;
            }

            let mut distance = self.distance(*boid);

            if distance < min_distance {
                num_close +=1.0;
                let mut xdiff = self.x - boid.x;
                let mut ydiff = self.y - boid.y;

                if xdiff >= 0.0 {
                    xdiff = min_distance.sqrt() - xdiff;
                } else if xdiff < 0.0 {
                    xdiff = -min_distance.sqrt() - xdiff;
                }
    
                if ydiff >= 0.0 {
                    ydiff = min_distance.sqrt() - ydiff;
                } else if ydiff < 0.0 {
                    ydiff = -min_distance.sqrt() - ydiff;
                }

                distance_x += xdiff;
                distance_y += ydiff;

            }

            if num_close == 0.0 {

            } else {
                self.x_velocity -= distance_x / 5.0;
                self.y_velocity -= distance_y / 5.0;
            }
        }
    }

    fn move_closer(&mut self, boids: Vec<Cow>, distance: f64) {
        if boids.len() < 1 {
            // do nothing
        } else {
            let mut avg_x = 0.0;
            let mut avg_y = 0.0;

            for boid in boids.iter() {
                if boid.x == self.x && boid.y == self.y {
                    continue;
                }
                if self.distance(*boid) > distance {
                    continue;
                }

                avg_x += self.x - boid.x;
                avg_y += self.y - boid.y;
            }

            avg_x /= boids.len() as f64;
            avg_y /= boids.len() as f64;

            let avg_distance = (avg_x*avg_x+avg_y*avg_y).sqrt() * -1.0;

            if avg_distance == 0.0 {
             
            } else {
                self.x_velocity = (self.x_velocity + (avg_x/avg_distance) * 0.15).min(self.constraints.max_velocity);
                self.y_velocity = (self.y_velocity + (avg_y/avg_distance) * 0.15).min(self.constraints.max_velocity);

            }

        }
    }



}



// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let constraints = Constraints {
        border:      5.0,
		width:       800.0,
		height:      800.0,
		max_velocity: 5.0
    };
    let mut bob = Cow {
        //nick_name: String::from("Bob"),
        x:10.0,
        y:10.0,
        constraints: constraints, 
        x_velocity: 1.0, 
        y_velocity: -1.0
    };
    
    bob.moving();
    //bob.moving();
    //bob.moving();

    let mut sam = Cow {
        //nick_name: String::from("Sam"),
        x:10.0,
        y:10.0,
        constraints: constraints, 
        x_velocity: 1.0, 
        y_velocity: -1.0
    };


    //bob.x =23.0;
    //bob.x_velocity = 4.0;

    //let message = String::from(bob.x.to_string());

    // Manufacture the element we're gonna append
    let h1 = document.create_element("h1")?;
    h1.set_text_content(Some("👋 Hello from Rust! 🦀"));

    let h2 = document.create_element("h2")?;
    h2.set_text_content(Some(&bob.x.to_string()));

    let cows_list = vec![bob, sam];
    bob.move_away(cows_list, 15.0);

    let h2_again = document.create_element("h2")?;
    h2_again.set_text_content(Some(&bob.distance(sam).to_string()));

    

    //let mut cows_list: Vec<Cow> = vec![bob];

    


    body.append_child(&h1)?;
    body.append_child(&h2)?;
    body.append_child(&h2_again)?;


    Ok(())
}
