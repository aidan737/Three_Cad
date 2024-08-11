use Three_D_Libary::*;
use render_window::*;
use two_d_functions::*;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::fs;
#[path = "Get_Functions.rs"] mod Get_Functions;
#[path = "../Script_Manager/Script_Runner.rs"] mod Script_Runner;
pub struct Button
{
    pub tasklayer: i32,
    pub texture: [Color; 9],
    pub is_2d: bool,
    pub script_path: PathBuf,
}


const bar_hight: f64 = 45.0;
const button_gap: f64 = 5.0;
static mut BUTTONS: Option<Arc<Mutex<Vec<Button>>>> = None;

fn get_buttons_mut() -> Option<Arc<Mutex<Vec<Button>>>> {
    unsafe { BUTTONS.as_mut().map(|b| b).cloned() } // Dereference the Arc to get the Mutex
}

pub fn render_toolbar(c: &Context, g: &mut G2d, mouse_position:[f64; 2], width: f64, hight: f64, mouse:bool)
{

    

    
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

    if let Some(buttons) = get_buttons_mut() {
        // Lock the mutex to ensure thread safety while iterating
        let buttons = buttons.lock().unwrap();
        for (index, button) in buttons.iter().enumerate() {
            // Access the individual Button struct here
            if(mouse_button_pressed(&mouse)){
                if(is_point_in_rectangle(
                    &Point2d {
                     point_x: mouse_position[0] ,
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
            BUTTONS = Some(Arc::new(Mutex::new(Get_Functions::get_toolbar_Functions())));
        }
    }



    //running scripts that are uploaded by the user pushing buttons
    Script_Runner::run_scripts(&c,g,mouse_position,width,hight,mouse);
}


fn is_point_in_rectangle(point: &Point2d, rect_point: &Point2d,size: &Point2d) -> bool {
    let x = point.point_x;
    let y = point.point_y;
    let rect_x = rect_point.point_x;
    let rect_y = rect_point.point_y;
    let rect_width = size.point_x;
    let rect_height = size.point_y;

    if x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height {
        return true;
    }

    false
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