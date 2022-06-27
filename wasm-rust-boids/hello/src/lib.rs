use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
//use std::{thread, time::Duration};
use std::collections::LinkedList;

use web_sys;

// Imports console.log()
#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearInterval(id: i32);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


const ANIM_DELAY: f64 = 0.025;  // 0.025 sec -> 40 FPS


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
    id: u64,
    x: f64,
    y: f64,
    former_x: f64,
    former_y: f64,
    constraints: Constraints,
    x_velocity: f64,
    y_velocity: f64
}


impl Cow {

    fn distance(&mut self, boid: Cow) -> f64 {
        let dist_x = self.x - boid.x;
        let dist_y = self.y - boid.x;
        (dist_x*dist_x + dist_y*dist_y).sqrt()
    }

}

fn draw(context: &web_sys::CanvasRenderingContext2d, cow: Cow) {

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
      x: cow.former_x,
      y: cow.former_y,
      radius: 4.0,
      color: String::from("white"),
      border_width: 2.0,
      border_color: 
      String::from("white"),
  };

  let current_circle = Circle {
      x: cow.x,
      y: cow.y,
      radius: 4.0,
      color: String::from("green"),
      border_width: 1.0,
      border_color: 
      String::from("green"),
  };

  draw_circle(previous_circle);
  let mut i = 0;
  while i < 10000 {
      i = i + 1;

  }
  draw_circle(current_circle);

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


    let mut cows_list: LinkedList<Cow> = LinkedList::new();

    let constraints = Constraints {
        border:      5.0,
        width:       800.0,
        height:      800.0,
        max_velocity: 5.0
    };
    
    let mut i = 0;
    
    while i < 10 {
        let cow = Cow {
            id:i,
            x:10.0,
            y:10.0,
            former_x:10.0,
            former_y:10.0,
            constraints: constraints,
            x_velocity: 1.0,
            y_velocity: -1.0,
        };
        i = i + 1;
        cows_list.push_back(cow)
    }

    let cb = Closure::wrap(Box::new(move || {
      log("interval elapsed!");
    }) as Box<dyn FnMut()>);


    let mut index = 0;
    while index < 30000 {
    //  while true {
      //============================================
  
      let cows_list_move_with: LinkedList<Cow> = cows_list.iter().map(|cow| {
  
        let mut current_cow = Cow { // self
          id:cow.id,
          x:cow.x,
          y:cow.y,
          former_x:cow.former_x,
          former_y:cow.former_y,
          constraints: cow.constraints,
          x_velocity: cow.x_velocity,
          y_velocity: cow.y_velocity,
        };
  
        let mut avg_x = 0.0;
        let mut avg_y = 0.0;
  
        // calculate the distance
        cows_list.iter().for_each(|cow| {
          if cow.x == current_cow.x && cow.y == current_cow.y {
            //println!("1️⃣");
            if current_cow.distance(*cow) > 300.0 {
              //println!("2️⃣");
              avg_x += cow.x_velocity;
              avg_y += cow.y_velocity;
            }
          }
        });
  
        avg_x /= cows_list.len() as f64;
        avg_y /= cows_list.len() as f64;
  
        //println!("3️⃣ {} {}",avg_x, avg_y);
  
        let avg_distance = (avg_x*avg_x+avg_y*avg_y).sqrt() * 1.0;
  
        //println!("🖐️ avg_distance {}", avg_distance);
  
        if avg_distance == 0.0 {
  
        } else {
          current_cow.x_velocity = (current_cow.x_velocity + (avg_x/avg_distance) * 0.05).min(current_cow.constraints.max_velocity);
          current_cow.y_velocity = (current_cow.y_velocity + (avg_y/avg_distance) * 0.05).min(current_cow.constraints.max_velocity);
  
          //println!("🖐️ x_velocity {} y_velocity {}",current_cow.y_velocity, current_cow.y_velocity);
        }
  
        Cow {
          id:cow.id,
          x:cow.x,
          y:cow.y,
          former_x:cow.former_x,
          former_y:cow.former_y,
          constraints: cow.constraints,
          x_velocity: current_cow.x_velocity,
          y_velocity: current_cow.y_velocity,
        }
      }).collect();
  
      
      cows_list_move_with.iter().for_each(|cow| {
        //draw(&context, *cow);
        //println!("🤖 {}: {} {}", cow.id, cow.x, cow.y);

      });
      
  
      let cows_list_move_closer: LinkedList<Cow> = cows_list_move_with.iter().map(|cow| {
  
        let mut current_cow = Cow { // self
          id:cow.id,
          x:cow.x,
          y:cow.y,
          former_x:cow.former_x,
          former_y:cow.former_y,
          constraints: cow.constraints,
          x_velocity: cow.x_velocity,
          y_velocity: cow.y_velocity,
        };
  
        let mut avg_x = 0.0;
        let mut avg_y = 0.0;
  
        // calculate the distance
        cows_list.iter().for_each(|cow| {
          if cow.x == current_cow.x && cow.y == current_cow.y {
            //println!("1️⃣");
            if current_cow.distance(*cow) > 300.0 {
              //println!("2️⃣");
              //avg_x += cow.x_velocity;
              //avg_y += cow.y_velocity;
  
              avg_x += current_cow.x - cow.x;
              avg_y += current_cow.y - cow.y;
            }
          }
        });
  
        avg_x /= cows_list.len() as f64;
        avg_y /= cows_list.len() as f64;
  
        //println!("3️⃣ {} {}",avg_x, avg_y);
  
        //let avg_distance = (avg_x*avg_x+avg_y*avg_y).sqrt() * 1.0;
        let avg_distance = (avg_x*avg_x+avg_y*avg_y).sqrt() * -1.0;
  
  
        //println!("🖐️ avg_distance {}", avg_distance);
  
        if avg_distance == 0.0 {
  
        } else {
          //current_cow.x_velocity = (current_cow.x_velocity + (avg_x/avg_distance) * 0.05).min(current_cow.constraints.max_velocity);
          //current_cow.y_velocity = (current_cow.y_velocity + (avg_y/avg_distance) * 0.05).min(current_cow.constraints.max_velocity);
          current_cow.x_velocity = (current_cow.x_velocity + (avg_x/avg_distance) * 0.15).min(current_cow.constraints.max_velocity);
          current_cow.y_velocity = (current_cow.y_velocity + (avg_y/avg_distance) * 0.15).min(current_cow.constraints.max_velocity);
  
          //println!("🖐️ x_velocity {} y_velocity {}",current_cow.y_velocity, current_cow.y_velocity);
        }
  
        Cow {
          id:cow.id,
          x:cow.x,
          y:cow.y,
          former_x:cow.former_x,
          former_y:cow.former_y,
          constraints: cow.constraints,
          x_velocity: current_cow.x_velocity,
          y_velocity: current_cow.y_velocity,
        }
  
      }).collect();
  
      
      cows_list_move_closer.iter().for_each(|cow| {
        //println!("🤖 {}: {} {}", cow.id, cow.x, cow.y);
        //draw(&context, *cow);
      });
      
  
  
      let cows_list_move_away: LinkedList<Cow> = cows_list_move_closer.iter().map(|cow| {
  
        let mut current_cow = Cow { // self
          id:cow.id,
          x:cow.x,
          y:cow.y,
          former_x:cow.former_x,
          former_y:cow.former_y,
          constraints: cow.constraints,
          x_velocity: cow.x_velocity,
          y_velocity: cow.y_velocity,
        };
  
        let mut distance_x = 0.0;
        let mut distance_y = 0.0;
        let mut num_close = 0.0;
  
        cows_list.iter().for_each(|cow| {
  
          if cow.x == current_cow.x && cow.y == current_cow.y {
            let min_distance= 15.0;
            let distance = current_cow.distance(*cow);
  
            if distance < min_distance {
              num_close +=1.0;
              let mut xdiff = current_cow.x - cow.x;
              let mut ydiff = current_cow.y - cow.y;
  
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
              current_cow.x_velocity -= distance_x / 5.0;
              current_cow.y_velocity -= distance_y / 5.0;
            }
          }
  
        });
  
        Cow {
          id:cow.id,
          x:cow.x,
          y:cow.y,
          former_x:cow.former_x,
          former_y:cow.former_y,
          constraints: cow.constraints,
          x_velocity: current_cow.x_velocity,
          y_velocity: current_cow.y_velocity,
        }
  
      }).collect();
  
      
      cows_list_move_away.iter().for_each(|cow| {
        //println!("🤖 {}: {} {}", cow.id, cow.x, cow.y);
        //draw(&context, *cow);
      });
      
  
      let cows_list_moving: LinkedList<Cow> = cows_list_move_away.iter().map(|cow| {
        let mut current_cow = Cow { // self
          id:cow.id,
          x:cow.x,
          y:cow.y,
          former_x:cow.former_x,
          former_y:cow.former_y,
          constraints: cow.constraints,
          x_velocity: cow.x_velocity,
          y_velocity: cow.y_velocity,
        };
        current_cow.x += current_cow.x_velocity;
        current_cow.y += current_cow.y_velocity;
  
        if current_cow.x <= current_cow.constraints.border || current_cow.x >= current_cow.constraints.width - current_cow.constraints.border {
          current_cow.x -= current_cow.x_velocity;
          current_cow.x = current_cow.x.max(current_cow.constraints.border);
          current_cow.x = current_cow.x.min(current_cow.constraints.width - current_cow.constraints.border);
          current_cow.x_velocity = -current_cow.x_velocity;
          current_cow.x += current_cow.x_velocity;
        }
        if current_cow.y <= current_cow.constraints.border || current_cow.y >= current_cow.constraints.height - current_cow.constraints.border {
          current_cow.y -= current_cow.y_velocity;
          current_cow.y = current_cow.y.max(current_cow.constraints.border);
          current_cow.y = current_cow.y.min(current_cow.constraints.height - current_cow.constraints.border);
          current_cow.y_velocity = -current_cow.y_velocity;
          current_cow.y += current_cow.y_velocity
        }
  
        Cow {
          id:current_cow.id,
          x:current_cow.x,
          y:current_cow.y,
          former_x:cow.former_x,
          former_y:cow.former_y,
          constraints: current_cow.constraints,
          x_velocity: current_cow.x_velocity,
          y_velocity: current_cow.y_velocity,
        }
  
      }).collect();
      
      //thread::sleep(Duration::from_millis(100));

      cows_list_moving.iter().for_each(|cow| {
        draw(&context, *cow);
        log("interval elapsed!");

        //println!("🤖 {}: {} {} {} {}", cow.id, cow.x, cow.y, cow.x_velocity, cow.y_velocity);

        //println!("🤖 {}: {} {} {} {}", cow.id, cow.x, cow.y, cow.x_velocity, cow.y_velocity);
      });
  

      /*
      let cb = Closure::wrap(Box::new(move || {
        log("interval elapsed!");
      }) as Box<dyn FnMut()>);
    
      setInterval(&cb, (ANIM_DELAY*1000.0) as u32);
      */

      cows_list = cows_list_moving;


      /*
      let mut i = 0;
      while i < 10000 {
          i = i + 1;

      }
      */

      /*
      let cb = Closure::wrap(Box::new(move || {
        log("interval elapsed!");
      }) as Box<dyn FnMut()>);
    

      window.set_interval_with_callback_and_timeout_and_arguments_0(&cb,(ANIM_DELAY*1000.0) as i32);
      */
      
      //thread::sleep(Duration::from_millis(100));
      //============================================
      index+=1;
    }
    

    //while true {}
    

    Ok(())
}
