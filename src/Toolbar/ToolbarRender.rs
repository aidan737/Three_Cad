use Three_D_Libary::*;
use render_window::*;
use two_d_functions::*;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::fs;
#[path = "Get_Functions.rs"] mod Get_Functions;
#[path = "../Script_Manager/Script_Runner.rs"] mod Script_Runner;

#[derive(Clone)]
pub struct Script_Button
{
    pub tasklayer: i32,
    pub texture: [Color; 9],
    pub is_2d: bool,
    pub script_path: PathBuf,
}

static is_2d_mode: Mutex<bool> = Mutex::new(false);
const bar_hight: f64 = 45.0;
const button_gap: f64 = 5.0;
static mut SCRIPT_BUTTONS: Option<Arc<Mutex<Vec<Script_Button>>>> = None;




#[derive(Clone)]
pub struct Function_Button
{
    pub tasklayer: i32,
    pub texture: [Color; 9],
    pub type_number: usize,

}

static mut FUNCTION_BUTTONS: Option<Arc<Mutex<Vec<Function_Button>>>> = None;

fn get_buttons_mut(is_2d:&bool) -> Option<Arc<Mutex<Vec<Script_Button>>>> {

    //deciding whether to get 2d button or 3d
    unsafe {
        SCRIPT_BUTTONS.as_mut().map(|buttons| {
            Arc::new(Mutex::new(
                buttons.lock().unwrap().iter()
                    .filter(|button| button.is_2d == *is_2d)
                    .cloned()
                    .collect()
            ))
        })
    }
}
fn get_funtion_buttons_mut() -> Option<Arc<Mutex<Vec<Function_Button>>>> {

    //deciding whether to get 2d button or 3d
    unsafe {
        FUNCTION_BUTTONS.as_mut().map(|buttons| {
            Arc::new(Mutex::new(
                buttons.lock().unwrap().iter()
                    .cloned()
                    .collect()
            ))
        })
    }
}
pub fn render_toolbar(c: &Context, g: &mut G2d, mouse_position:[f64; 2], width: f64, hight: f64, mouse:bool, keypressed:Key)
{
    //render back plate for taskbar
    draw_rectangle(
        &c,
        g,
        &Point2d {
             point_x: 0.0,
             point_y: hight- bar_hight,
        },                
        &Color {
            red: 0.0,
            green: 162.0,
            blue: 237.0,
            transperency: 1.0,
        },
        width,
        bar_hight
        
    );


    //render function buttons
    let mut is_2d = is_2d_mode.lock().unwrap();


    
    if let Some(buttons) = get_funtion_buttons_mut() {
  // Lock the mutex to ensure thread safety while iterating
  let buttons = buttons.lock().unwrap();
  for (index, button) in buttons.iter().enumerate() {
      // Access the individual Button struct here
       
          if(is_point_in_rectangle(
              &Point2d {
               point_x: mouse_position[0],
               point_y: mouse_position[1],
              },
              &Point2d {
               point_x: (width-30.0) -(index as f64* (30.0+button_gap)) ,
               point_y: hight- bar_hight,
              },
              &Point2d {
               point_x: 30.0,
               point_y: 30.0,
              } )){
                //here we must load a script to script_runner
              if(mouse_button_pressed(&mouse)){

                if(button.type_number == 1)
                {
                    //add a 2d instance in the script line
                    *is_2d = !*is_2d;
                }

                



                }
             }

          
      draw_image(&c,g,&button.texture,&Point2d {
          point_x: (width-30.0) -(index as f64* (30.0+button_gap)) ,
          point_y: hight- bar_hight,
     },3,3,10.0   );
  }
    }else {
        let mut result:String=String::from("1.0,0.0,0.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,1.0,2.0,1.0,2.0,1.0,0.0,0.0,1.0,1.0,2.0,1.0,2.0,"); 
        let colors: Vec<&str> = result.split(',').collect();
   
        let mut texture: [Color; 9] = [Color{ red: 0.0, green: 0.0, blue: 0.0, transperency: 0.0 }; 9];
        for (index) in 0..9 {
         texture[index] =(Color { red: colors[0+index].parse().unwrap_or(0.0), green: colors[1+index].parse().unwrap_or(0.0), blue:colors[2+index].parse().unwrap_or(0.0), transperency: colors[3+index].parse().unwrap_or(0.0) });
        }

        let mut buttons_Found: Vec<Function_Button> = Vec::new();
        buttons_Found.push(Function_Button{ tasklayer:1,
            texture: texture,
            type_number: 1,
         
         });
      
        unsafe {
            FUNCTION_BUTTONS = Some(Arc::new(Mutex::new( buttons_Found)));
        }
    }







    //render script buttons

    if let Some(buttons) = get_buttons_mut(&*is_2d) {
        // Lock the mutex to ensure thread safety while iterating
        let buttons = buttons.lock().unwrap();
        for (index, button) in buttons.iter().enumerate() {
            // Access the individual Button struct here
          
                if(is_point_in_rectangle(
                    &Point2d {
                     point_x: mouse_position[0],
                     point_y: mouse_position[1],
                    },
                    &Point2d {
                     point_x: (index as f64* (30.0+button_gap)) ,
                     point_y: hight- bar_hight,
                    },
                    &Point2d {
                     point_x: 30.0,
                     point_y: 30.0,
                    } )){
                      //here we must load a script to script_runner
                    if(mouse_button_pressed(&mouse)){
                        

                       
                   let script_content = fs::read_to_string(button.script_path.clone())
                   .expect("Failed to read script file");
                  
                      Script_Runner::load_script(script_content, false)
                      }
                   }

                
            draw_image(&c,g,&button.texture,&Point2d {
                point_x: (index as f64* (30.0+button_gap)) ,
                point_y: hight- bar_hight,
           },3,3,10.0   );
        }
    } else {
        unsafe {
            SCRIPT_BUTTONS = Some(Arc::new(Mutex::new(Get_Functions::get_toolbar_Functions())));
        }
    }



    //running scripts that are uploaded by the user pushing buttons
    Script_Runner::run_scripts(&c,g,mouse_position,width,hight,mouse,keypressed);
}




static mouse_pressed: Mutex<bool> = Mutex::new(false);


fn mouse_button_pressed(mouse: &bool) -> bool {
    let mut mouse_held = mouse_pressed.lock().unwrap();
    if(*mouse == true ){
        if(*mouse_held==false){
            *mouse_held = true;
         return true;
        }
    }else
    {
        *mouse_held = false;
        
    }
    return false;
}