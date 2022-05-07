use reqwest;

// query a web endpoint for bus data
// URL: https://realtime.portauthority.org/bustime/api/v3/getvehicles?key={}&format=json&rt=61A
// parse as json

#[derive(Debug, Copy, Clone)]
pub enum Routes {
  BUS61A,
  BUS61B,
  BUS61C,
  BUS61D
}


impl From<&str> for Routes {
  fn from(s: &str) -> Self {
    match s {
      "61A" => Routes::BUS61A,
      "61B" => Routes::BUS61B,
      "61C" => Routes::BUS61C,
      "61D" => Routes::BUS61D,
      _ => panic!("Unknown route: {}", s)
    }
  }
}

impl std::fmt::Display for Routes {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Routes::BUS61A => write!(f, "61A"),
      Routes::BUS61B => write!(f, "61B"),
      Routes::BUS61C => write!(f, "61C"),
      Routes::BUS61D => write!(f, "61D"),
    }
  }
}

// Read string from ../../key.txt
fn read_key() -> String {
  use std::{fs::File, io::Read};
  let mut file = File::open("../key.txt").expect("Unable to open key.txt");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Unable to read key.txt");
  contents
}

// pub fn request(route : Routes) -> Result<HashMap<String, String>, reqwest::Error> {
pub fn request(route : Routes) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
  let key = read_key();
  let base = "https://realtime.portauthority.org/bustime/api/v3/getvehicles";
  let url = format!(
    "{}?key={}&format=json&rt={}", 
    base,
    key, 
    route.to_string()
  );
  println!("{}", url);
  let response: String = reqwest::blocking::get(&url)?.text()?;
  let response_json: serde_json::Value = serde_json::from_str(&response)?;
  Ok(response_json)
}