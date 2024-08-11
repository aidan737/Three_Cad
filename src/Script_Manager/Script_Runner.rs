use Three_D_Libary::*;
use two_d_functions::*;
use hlua::{Lua, LuaFunction, function0};
use std::sync::{Arc, Mutex};
#[path = "../Render_3d/Render.rs"] mod render_3d;
use window_functions::*;
pub struct script_evironment
{
    pub script: String,
    pub is_2d: bool,
}

pub struct script_Information
{
    
    pub is_required: bool, 
    pub displayed_on_window: bool, 
    pub name:  String, 
    pub type_of_value:  String,
    pub value_number: f64, 

}


pub static SCRIPTS_UPLOADED: Mutex<Vec<script_evironment>> = Mutex::new(vec![]);

pub static SCRIPTS_UPLOADED_INFORMATION: Mutex<Vec<Vec<script_Information>>> = Mutex::new(vec![]);



//vars for the script loading
pub static SCRIPT_LOADING_INFORMATION: Mutex<Vec<script_Information>> = Mutex::new(vec![]);
static script_loading: Mutex<bool> = Mutex::new(false);
static script_2d: Mutex<bool> = Mutex::new(false);
static script_loading_value: Mutex<String> = Mutex::new(String::new());


// load in a script
pub fn load_script(script_value: String, is_2d_value: bool)
{
    // Clear SCRIPT_LOADING_INFORMATION
    SCRIPT_LOADING_INFORMATION.lock().unwrap().clear();


    //get the information feilds from the script (all the get functions)
    let info_funcs = extract_get_info_functions(&script_value);


    for func in info_funcs {
        println!("Function: {}", func);
        //add function to SCRIPT_LOADING_INFORMATION
        if(func == "get_information_number()")
        {
          SCRIPT_LOADING_INFORMATION.lock().unwrap().push(script_Information{is_required: true, displayed_on_window: true, name: "".to_string(), type_of_value: "number".to_string(), value_number: 0.0});

        }

    }

    
    let mut script_loading_bool = script_loading.lock().unwrap();
    *script_loading_bool = true;

    let mut script_loading_2d = script_2d.lock().unwrap();
    *script_loading_2d = is_2d_value;

    let mut script_loading_value_string = script_loading_value.lock().unwrap();
    *script_loading_value_string = script_value;
 

    //set up window
    let mut window_feilds = window_feild.lock().unwrap();
    window_feilds.clear();
    for info in SCRIPT_LOADING_INFORMATION.lock().unwrap().iter()
    {

      // cheaking if info should be displayed
        if(info.displayed_on_window){

          //cheking type int
          if(info.type_of_value == "number".to_string()){
            window_feilds.push(Window_feild{ input_type_string1_int2: 2,title: "CANCEL".to_string(), value: "".to_string()});
          }
        }

    }
    window_feilds.push(Window_feild{ input_type_string1_int2: 3,title: "CANCEL".to_string(), value: "NOTHING".to_string()});
    window_feilds.push(Window_feild{ input_type_string1_int2: 3,title: "APPROVE".to_string(), value: "nothing".to_string()});
}


fn extract_get_info_functions(script: &String) -> Vec<String> {
  let get_info_functions = vec![
      "get_information_number",
      "get_information_string",
      // Add other get information functions as needed
  ];

  let mut extracted = Vec::new();
  for &func in &get_info_functions {
      let mut start = 0;
      while let Some(pos) = script[start..].find(func) {
          let func_start = start + pos;
          if let Some(end) = script[func_start..].find(')') {
              let func_end = func_start + end + 1;
              extracted.push(script[func_start..func_end].to_string());
          }
          start = func_start + func.len();
      }
  }
  extracted
}









fn Add_Script_3d(script_value: String, script_info:Vec<script_Information> ,is_2d_value: bool)
{




    let  new_object =script_evironment {
        script:script_value,   
        is_2d:is_2d_value,
    };
    let mut global_vec = SCRIPTS_UPLOADED.lock().unwrap();
    let mut global_vec_info = SCRIPTS_UPLOADED_INFORMATION.lock().unwrap();
    // Push the value to the vector
    global_vec.push(new_object);
    global_vec_info.push(script_info);


}




static script_running_index: Mutex<usize> = Mutex::new(0);






static window_pos: Mutex<Window_Struct> = Mutex::new(Window_Struct {
    point_x: 0.0,
    point_y: 0.0,

   });
   static  window_feild:Mutex<Vec<Window_feild>> = Mutex::new(vec![]);
pub fn run_scripts(c: &Context, g: &mut G2d, mouse_position:[f64; 2], width: f64, hight: f64, mouse:bool,keypressed:Key) -> Result<(), hlua::LuaError>
{
   
    //run the window to initilize scripts
    let mut script_loading_bool = script_loading.lock().unwrap();
    if (*script_loading_bool == true)
    {
        let mut window_position = window_pos.lock().unwrap();

        let mut window_feilds = window_feild.lock().unwrap();

       

       



        let return_value = window_functions::draw_window(&c,g,&mut *window_feilds,&mut *window_position,mouse_position, mouse, 100.0, 20.0,"TITLE".to_string(),keypressed);


      
        let count = SCRIPT_LOADING_INFORMATION.lock().unwrap().len();

        if(return_value[count].value == "true")
        {
            *script_loading_bool = false;
        }
        if(return_value[count+1].value == "true")
        {
            *script_loading_bool = false;
            let mut script_loading_2d = script_2d.lock().unwrap();
            let mut script_loading_value_string = script_loading_value.lock().unwrap();

            let mut script_info: Vec<script_Information> = vec![];

            for (index, info) in SCRIPT_LOADING_INFORMATION.lock().unwrap().iter().enumerate()
            {
             
              if(return_value[index].input_type_string1_int2 == 2)
              {
              
                let  new_object =script_Information {
                   is_required: info.is_required, 
                   displayed_on_window: info.displayed_on_window, 
                   name:  "info.name".to_string(), 
                   type_of_value:  "number".to_string(),
                   value_number: return_value[index].value.parse().unwrap(), 
              };
                script_info.push(new_object);

               
              }
            }
     
            //if the script is 3d
            if(*script_loading_2d == false){
              
               Add_Script_3d(script_loading_value_string.to_string(),script_info,  *script_loading_2d);
            }
        }

        *window_feilds = return_value;
    
    }




    let mut lua = Lua::new();
    //setting custom functions
    lua.set("add_shape", function0(add_shape));
    lua.set("get_information_number", function0(get_information_number));






    
    let global_vec = Arc::new(SCRIPTS_UPLOADED.lock().unwrap());

    // Iterate over the global vector
    for (index, script_object) in global_vec.iter().enumerate() {
      let mut script_running = script_running_index.lock().unwrap();
      *script_running = index;
      drop(script_running);

      let mut script_info = script_info_index.lock().unwrap();
      *script_info = 0;
      drop(script_info);
      

        lua.execute::<()>(&script_object.script)?;
    }


    render_3d::Render_Objects(&c,g,width,hight,mouse_position,mouse);
    render_3d::clear_objects();

    Ok(())
}





// added functions


//get information funcs
//record number of functions called
static script_info_index: Mutex<usize> = Mutex::new(0);

fn get_information_number() -> Result<f64, hlua::LuaError> {

  
  let mut script_info = script_info_index.lock().unwrap();

  let mut script_running = script_running_index.lock().unwrap();

  let global_vec = Arc::new(SCRIPTS_UPLOADED_INFORMATION.lock().unwrap());

 
   let value:f64 = global_vec[*script_running][*script_info].value_number;


  
   *script_info += 1;
    Ok(value)
  }

  fn get_information_Shape() -> Result<i32, hlua::LuaError> {

    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok(0)
  }
  fn get_information_Sketch() -> Result<i32, hlua::LuaError> {
    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok(0)
  }
  fn get_information_position() -> Result<i32, hlua::LuaError> {

    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok(0)
  }
  fn get_information_position_mouse() -> Result<i32, hlua::LuaError> {


    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok(0)
  }
  fn get_information_String() -> Result<String, hlua::LuaError> {

    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok("0".to_string())
  }
  fn get_information_bool() -> Result<bool, hlua::LuaError> {

    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok(false)
  }

  fn get_information_Option_List() -> Result<i32, hlua::LuaError> {

    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok(0)
  }
  fn get_information_2D_Position() -> Result<i32, hlua::LuaError> {


    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok(0)
  }
  fn get_information_3D_Position() -> Result<i32, hlua::LuaError> {

    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok(0)
  }
  fn get_information_edge() -> Result<i32, hlua::LuaError> {


    
  let mut script_info = script_info_index.lock().unwrap();
  *script_info += 1;
    Ok(0)
  }






//action funcs
fn add_shape() {
    render_3d::add_object();
}

fn add_sketch() {
    
}
fn edit_shape() {
    render_3d::add_object();
}

//actions funcs for 2d
fn edit_sketch() {
    //only edits the sketch selected
}



