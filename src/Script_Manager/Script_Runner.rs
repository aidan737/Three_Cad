use Three_D_Libary::*;
use two_d_functions::*;
use hlua::{Lua, LuaFunction, function0};
use std::sync::{Arc, Mutex};
#[path = "../Render_3d/Render.rs"] mod render_3d;

pub struct script_evironment
{
    pub script: String,
    pub information: Vec<script_Information>,
    pub is_2d: bool,
}

pub struct script_Information
{
    
    pub is_required: bool, 
    pub displayed_on_window: bool, 
    pub name:  String, 
    pub type_of_value:  String,
    pub value_pos_x: f64, //this could be mouse information
    pub value_pos_y: f64, //this could be mouse information


}


pub static SCRIPTS_UPLOADED: Mutex<Vec<script_evironment>> = Mutex::new(vec![]);



//vars for the script loading
pub static SCRIPT_LOADING_INFORMATION: Mutex<Vec<script_Information>> = Mutex::new(vec![]);
static script_loading: Mutex<bool> = Mutex::new(false);
static script_2d: Mutex<bool> = Mutex::new(false);
static script_loading_value: Mutex<String> = Mutex::new(String::new());


// load in a script
pub fn load_script(script_value: String, is_2d_value: bool)
{
    let mut script_loading_bool = script_loading.lock().unwrap();
    *script_loading_bool = true;

    let mut script_loading_2d = script_2d.lock().unwrap();
    *script_loading_2d = is_2d_value;

    let mut script_loading_value_string = script_loading_value.lock().unwrap();
    *script_loading_value_string = script_value;


    //get the information feilds from the script



}


fn Add_Script(script_value: String, is_2d_value: bool)
{
    let  new_object =script_evironment {
        script:script_value,
        information: Vec::new(),
        is_2d:is_2d_value,
    };
    let mut global_vec = SCRIPTS_UPLOADED.lock().unwrap();

    // Push the value to the vector
    global_vec.push(new_object);



}




pub fn run_scripts(c: &Context, g: &mut G2d, mouse_position:[f64; 2], width: f64, hight: f64, mouse:bool) -> Result<(), hlua::LuaError>
{
   
    let mut lua = Lua::new();
    lua.set("add_shape", function0(add_shape));
    
    let global_vec = Arc::new(SCRIPTS_UPLOADED.lock().unwrap());
   
    // Iterate over the global vector

    for script_object in global_vec.iter() {
        
        lua.execute::<()>(&script_object.script)?;
    }


    render_3d::Render_Objects(&c,g,width,hight);
    render_3d::clear_objects();

    Ok(())
}





// added functions
fn set_information() {
  
  
}
fn get_information() {
  
  
}



fn add_shape() {
  
    render_3d::add_object();
}








