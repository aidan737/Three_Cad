use Three_D_Libary::*;
use render_window::*;
use three_d_functions::*;
use std::sync::{Arc, Mutex};



//list of objects to render
pub static OBJECTS_3D: Mutex<Vec<cad_object_3d>> = Mutex::new(vec![]);




pub fn Render_Objects(c: &Context, g: &mut G2d, window_width: f64, window_hight: f64)
{
    let mut camera_rotation_x:f64 = 0.0;
    let mut camera_rotation_y:f64 = 1.0;
    //INITIALIZE 3D
    Begin_3d();

    let global_vec = Arc::new(OBJECTS_3D.lock().unwrap());

    
    render_3d(&c,g,&global_vec,camera_rotation_x,camera_rotation_y,window_hight,window_width);
}

pub fn add_object()
{
    let  new_object = cad_object_3d {
        name: "My Object".to_string(),
        points_3d: vec![
            point3d { x: 110.0, y: 133.0, z: 0.0 },  // Front bottom left
        point3d { x: 133.0, y: 133.0, z: 0.0 },  // Front bottom right
        point3d { x: 133.0, y: 103.0, z: 0.0 },    // Front top right
        point3d { x: 110.0, y: 103.0, z: 0.0 },   // Front top left
        point3d { x: 110.0, y: 133.0, z: 20.0 }, // Back bottom left
        point3d { x: 133.0, y: 133.0, z: 20.0 },  // Back bottom right
        point3d { x: 133.0, y: 103.0, z: 20.0 },    // Back top right
        point3d { x: 110.0, y: 103.0, z: 20.0 },   // Back top left
        ],
        triangles: vec![
            // Front face
            triangle_point_index { t1: 3, t2: 1, t3: 0 }, // Bottom left, bottom right, top left
        triangle_point_index { t1: 1, t2: 2, t3: 3 }, // Bottom right, top right, top left
      
        // Back face
        triangle_point_index { t1: 7, t2: 5, t3: 4 }, // Bottom left, bottom right, top left
        triangle_point_index { t1: 5, t2: 6, t3: 7 }, // Bottom right, top right, top left
      
        // Left face
        triangle_point_index { t1: 7, t2: 4, t3: 0 }, // Bottom left, back bottom left, back top left
        triangle_point_index { t1: 0, t2: 3, t3: 7 }, // Bottom left, front top left, back top left 
      
        // Right face
        triangle_point_index { t1: 1, t2: 5, t3: 6 }, // Bottom right, back bottom right, back top right
        triangle_point_index { t1: 1, t2: 2, t3: 6 }, // Bottom right, front top right, back top right
      
        // Top face
        triangle_point_index { t1: 3, t2: 2, t3: 6 }, // Front top left, front top right, back top right
        triangle_point_index { t1: 3, t2: 7, t3: 6 }, // Front top left, back top left, back top right
      
        // Bottom face
        triangle_point_index { t1: 0, t2: 1, t3: 5 }, // Front bottom left, front bottom right, back bottom right
        triangle_point_index { t1: 0, t2: 4, t3: 5 }, // Front bottom left, back bottom left, back bottom right
        ],
        surfaces: vec![],
        lighting_on: true,
        color:[0.0, 1.0, 0.0, 1.0], 
    };
  // Lock the Mutex to get a mutable reference to the vector
  let mut global_vec = OBJECTS_3D.lock().unwrap();

  // Push the value to the vector
  global_vec.push(new_object);
  
}


pub fn clear_objects() {
    // Lock the Mutex to get a mutable reference to the vector
    let mut global_vec = OBJECTS_3D.lock().unwrap();

    // Clear the vector
    global_vec.clear();
}