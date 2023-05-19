// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use rustpython_vm::pyobject::{PyResult, PyObject};
// use rustpython_vm::virtual_machine::VirtualMachine;

// fn execute_python_code(code: &str) -> PyResult<PyObject> {
//     // Create a new virtual machine
//     let mut vm = VirtualMachine::new();

//     // Execute the Python code
//     vm.run_code(code, None)
// }

// fn main_rustpython() {
//     let code = r#"
//         print("Hello, RustPython!")
//         x = 2 + 3
//         print("2 + 3 =", x)
//     "#;

//     let result = execute_python_code(code);
//     println!("{:?}", result);
// }

use cozo::*;
use std::path::Path;
// use std::path::PathBuf;
use home::*;
use std::fs::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
    // let mut db = DbInstance::new("mem", "", Default::default()).unwrap();
    // const script = "?[a] := a in [1, 2, 3]";
    // const result = db.run_script(script, Default::default()).unwrap();
    // println!("{:?}", result);

}

fn hello() {
  let home_str=home_dir().unwrap();
  let db_path=home_str.to_str().unwrap().to_owned()+"/Sastra/db";
  DirBuilder::new().recursive(true).create(db_path.clone()).unwrap();
  let database_url = Path::new(&db_path).join("sastra.db");

  let db = DbInstance::new("sqlite", &database_url, Default::default()).unwrap();
  let script = "?[a] := a in [1, 2, 3,4,5,6,7,8]";
  let result = db.run_script(script, Default::default()).unwrap();
  println!("{:?}", result);
  
}



fn main() {

  // let db = DbInstance::new("mem", "", Default::default()).unwrap();
  // let script = "?[a] := a in [1, 2, 3,4,5,6,7,8]";
  // let result = db.run_script(script, Default::default()).unwrap();
  // println!("{:?}", result);
  hello();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    // .invoke_handler(tauri::generate_handler![hello])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}




