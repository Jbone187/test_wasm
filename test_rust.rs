use wasm_bindgen::prelude::*;

#[wasm_bindgen]

pub fn test_rust() -> String {

  let a = "Hello".to_string();

  return a
}
  //String::from("Hello from Wasm");


/*
pub fn test(s: String) -> String {
    
    return String::from(s + "Hello from Wasm");
  }

  */
