









use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::{thread, time::Duration};


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

    //🖐️
    fn distance(&mut self, boid: Cow) -> f64 {
        let dist_x = self.x - boid.x;
        let dist_y = self.y - boid.x;
        (dist_x*dist_x + dist_y*dist_y).sqrt()
    }

    fn move_away(&mut self, boids: &Vec<Cow>, min_distance: f64) {
        let mut distance_x = 0.0;
        let mut distance_y = 0.0;
        let mut num_close = 0.0;

        for boid in boids.iter() {

            if boid.x == self.x && boid.y == self.y {
                continue;
            }

            let distance = self.distance(*boid);

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

    fn move_closer(&mut self, boids: &Vec<Cow>, distance: f64) {
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

    //🖐️
    fn move_with(&mut self, boids: &Vec<Cow>, distance: f64) {
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

                avg_x += boid.x_velocity;
                avg_y += boid.y_velocity;
            }

            avg_x /= boids.len() as f64;
            avg_y /= boids.len() as f64;

            let avg_distance = (avg_x*avg_x+avg_y*avg_y).sqrt() * 1.0;

            if avg_distance == 0.0 {
             
            } else {
                self.x_velocity = (self.x_velocity + (avg_x/avg_distance) * 0.05).min(self.constraints.max_velocity);
                self.y_velocity = (self.y_velocity + (avg_y/avg_distance) * 0.05).min(self.constraints.max_velocity);

            }

        }
    }

    fn draw(&mut self, context: &web_sys::CanvasRenderingContext2d, former_x: f64, former_y: f64) {

        let draw_circle = |circle: Circle|{
            context.begin_path();
            context.arc(
                circle.x.into(),
                circle.y.into(),
                circle.radius.into(),
                0.0,
                2.0 * std::f64::consts::PI,
            ).unwrap();

            context.set_fill_style(&circle.color.into());
            context.fill();

            context.set_line_width(circle.border_width.into());
            context.set_stroke_style(&circle.border_color.into());

            context.stroke();
            
        };

        let previous_circle = Circle {
            x: former_x,
            y: former_y,
            radius: 4.0,
            color: String::from("white"),
            border_width: 2.0,
            border_color: 
            String::from("white"),
        };

        let current_circle = Circle {
            x: former_x,
            y: former_y,
            radius: 4.0,
            color: String::from("green"),
            border_width: 1.0,
            border_color: 
            String::from("green"),
        };

        draw_circle(previous_circle);
        draw_circle(current_circle);


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

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;



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
        x:30.0,
        y:30.0,
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

    let mut cows_list: Vec<Cow> = vec![];

    cows_list.push(bob);
    cows_list.push(sam);


    /*
    let mut i = 0;

    while i < 30 {
        let cow = Cow {
            x:10.0,
            y:100.0,
            constraints: constraints, 
            x_velocity: 1.0, 
            y_velocity: -1.0
        };
        i = i + 1;
        cows_list.push(cow)

    }
    */


    body.append_child(&h1)?;
    body.append_child(&h2)?;

    // Try only with the existing Cow
    while true {

        let mut former_x = bob.x;
        let mut former_y = bob.y;
    
        bob.move_with(&cows_list, 300.0);
        bob.move_closer(&cows_list, 300.0);
        bob.move_away(&cows_list, 15.0);
        bob.moving();
    
        bob.draw(&context, former_x, former_y);
    
        former_x = sam.x;
        former_y = sam.y;
    
        sam.move_with(&cows_list, 300.0);
        sam.move_closer(&cows_list, 300.0);
        sam.move_away(&cows_list, 15.0);
        sam.moving();
    
        sam.draw(&context, former_x, former_y);
    
        thread::sleep(Duration::from_millis(100));

        /*
        let cows_iter = cows_list.iter();

        for cow in cows_iter {

            cow.move_with(&cows_list, 300.0);
            cow.move_closer(&cows_list, 300.0);

            cow.move_away(&cows_list, 15.0);
            cow.moving();
        
            cow.draw(&canvas, &context, former_x, former_y);
        }
        */
    }
    
    




    Ok(())
}
