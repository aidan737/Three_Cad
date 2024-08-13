
use crate::toolbar_render::Script_Button;
use Three_D_Libary::two_d_functions::Color;
use std::convert::TryInto;
use std::fs;
use std::io;
fn read_file_contents(file_path: &str) -> Result<String, std::io::Error> {
  // Attempt to read the file contents
  match fs::read_to_string(file_path) {
    Ok(contents) => Ok(contents),
    Err(error) => Err(error),
  }
}

pub fn get_toolbar_Functions() -> Vec<Script_Button>
{

  let mut buttons_Found: Vec<Script_Button> = Vec::new();
  let dir_3d = "C:\\Users\\aidan\\Desktop\\VsRust\\3Cad\\Three_Cad\\3Cad_Scripts"; // replace with your desired directory
  let dir_2d = "C:\\Users\\aidan\\Desktop\\VsRust\\3Cad\\Three_Cad\\3Cad_Scripts_2d"; 
  for entry in fs::read_dir(dir_3d).expect("Failed to read directory") {
      match entry {
          Ok(entry) => {
              let path = entry.path();
              if path.is_dir() {


                let mut result:String=String::from("empty"); 


                match read_file_contents(&(path.to_str().unwrap().to_owned() +"\\Avitar.txt")) {
                   Ok(contents) =>result =contents,
                   Err(error) => println!("Error reading file: {}", error),
                 }
                 let colors: Vec<&str> = result.split(',').collect();

                 let mut texture: [Color; 9] = [Color{ red: 0.0, green: 0.0, blue: 0.0, transperency: 0.0 }; 9];
                 for (index) in 0..9 {
                  texture[index] =(Color { red: colors[0+index].parse().unwrap_or(0.0), green: colors[1+index].parse().unwrap_or(0.0), blue:colors[2+index].parse().unwrap_or(0.0), transperency: colors[3+index].parse().unwrap_or(0.0) });
                 }
       
                buttons_Found.push(Script_Button
                  {
                      tasklayer:1,
                      texture: texture,
                      is_2d: false,
                      script_path:  (&(path.to_str().unwrap().to_owned() +"\\Script.txt").clone()).into(),
          
                  });
                  println!("{}", path.display());
              }
          },
          Err(_) => continue, // skip errors
      }
  }




  for entry in fs::read_dir(dir_2d).expect("Failed to read directory") {
    match entry {
        Ok(entry) => {
            let path = entry.path();
            if path.is_dir() {


              let mut result:String=String::from("empty"); 


              match read_file_contents(&(path.to_str().unwrap().to_owned() +"\\Avitar.txt")) {
                 Ok(contents) =>result =contents,
                 Err(error) => println!("Error reading file: {}", error),
               }
               let colors: Vec<&str> = result.split(',').collect();

               let mut texture: [Color; 9] = [Color{ red: 0.0, green: 0.0, blue: 0.0, transperency: 0.0 }; 9];
               for (index) in 0..9 {
                texture[index] =(Color { red: colors[0+index].parse().unwrap_or(0.0), green: colors[1+index].parse().unwrap_or(0.0), blue:colors[2+index].parse().unwrap_or(0.0), transperency: colors[3+index].parse().unwrap_or(0.0) });
               }
     
              buttons_Found.push(Script_Button
                {
                    tasklayer:1,
                    texture: texture,
                    is_2d: true,
                    script_path:  (&(path.to_str().unwrap().to_owned() +"\\Script.txt").clone()).into(),
        
                });
                println!("{}", path.display());
            }
        },
        Err(_) => continue, // skip errors
    }
}







   
    //TESTING
    

        return buttons_Found;
}

