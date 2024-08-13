use Three_D_Libary::*;
use render_window::*;
use two_d_functions::*;
#[path = "Toolbar/ToolbarRender.rs"] mod toolbar_render;







fn main() {
    let mut window: PistonWindow = start_window("Three Cad",2048.0, 1089.0);
    let mut cursor = [0.0, 0.0];
    let mut mouse_button_left = false;
    let mut key_pressed = Key::Unknown;




    while let Some(e) = window.next() {
       let window_hight = window.size().height;
       let window_width = window.size().width;

       if let Some(Button::Keyboard(key)) = e.press_args() {
       
        key_pressed = key;
     
       }
       if let Some(Button::Keyboard(key)) = e.release_args() {
       
        key_pressed =  Key::Unknown;
       }

       if let Some(Button::Mouse(button)) = e.press_args() {
      
        if(button == MouseButton::Left)
        {
            mouse_button_left = true;

        }
       }
       if let Some(Button::Mouse(button)) = e.release_args() {
       
        if(button == MouseButton::Left)
        {
            mouse_button_left = false;

        }
    }
       e.mouse_cursor(|pos| {
           cursor = pos;
           
       }); 
       window.draw_2d(&e, |c, g, _device| {
            clear_screen(&c,g);
            
            toolbar_render::render_toolbar(&c,g,cursor,window_width,window_hight,mouse_button_left,key_pressed);
           
   

            
     

        });
       
    }
}


