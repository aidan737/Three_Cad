use Three_D_Libary::*;
use render_window::*;
use three_d_functions::*;
use std::sync::{Arc, Mutex};

pub struct cad_sketch
{
    pub name:String,
    pub points_3d:Vec<point3d>,
    pub triangles:Vec<triangle_point_index>,
    pub surfaces:Vec<surface>,
    pub points_2d:Vec<point2d>,


}

pub static SKETCHS: Mutex<Vec<cad_object_3d>> = Mutex::new(vec![]);
