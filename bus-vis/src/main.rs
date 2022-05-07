use std::{io, thread, time::Duration};
use tui::{
  backend::CrosstermBackend,
  widgets::{Widget, GraphType, Dataset, Chart, Block, Borders, canvas::{Canvas}},
  layout::{Layout, Constraint, Direction},
  Terminal,
  symbols,
  style::Style,
};
use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde_json::Value;

mod query;

#[derive(Debug, Clone)]
struct BusInfo {
  route: query::Routes,
  lat: f64,
  long: f64,
  vehicle_id: String,
}

fn unpack_vehicle(vehicle: &Value) -> BusInfo {
  match vehicle {
    Value::Object(obj) => {
      BusInfo {
        route: obj.get("rt").unwrap().as_str().unwrap().into(),
        lat: obj.get("lat").unwrap().as_str().unwrap().parse::<f64>().unwrap(),
        long: obj.get("lon").unwrap().as_str().unwrap().parse::<f64>().unwrap(),
        vehicle_id: obj.get("vid").unwrap().as_str().unwrap().into(),
      }
    }
    _ => panic!("Expected an Object"),
  }
}

fn main() -> Result<(), io::Error> {
  let buses = query::request(query::Routes::BUS61A).unwrap();
  // extract all the long,lat from buses
  let buses = match buses {
    Value::Object(x) => x,
    _ => panic!("Expected an object"),
  };
  let buses = buses.get("bustime-response").unwrap();
  let buses = match buses {
    Value::Object(x) => x,
    _ => panic!("Expected an object"),
  };
  let buses = buses.get("vehicle").unwrap();
  let bus_arr = buses.as_array().unwrap().iter().map(|x| unpack_vehicle(x)).collect::<Vec<BusInfo>>();
  println!("{:#?}", bus_arr);


  // Plot lat/long onto a map in tui

  let stdout = io::stdout();
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // let canvas = 
  //   Canvas::default()
  //     .block(Block::default().borders(Borders::ALL).title("World"))
  //     .paint(|ctx| {
  //       ctx.draw(&Map {
  //           color: Color::White,
  //           resolution: MapResolution::High,
  //       });
  //       ctx.print(
  //           app.x,
  //           -app.y,
  //           Span::styled("You are here", Style::default().fg(Color::Yellow)),
  //       );
  //   })
  //   .x_bounds([-180.0, 180.0])
  //   .y_bounds([-90.0, 90.0]);
  
  terminal.draw(|f| {
    let size = f.size();
      let block = Block::default()
          .title("Block")
          .borders(Borders::ALL);
      f.render_widget(block, size);
  })?;

  thread::sleep(Duration::from_millis(5000));

  // restore terminal
  disable_raw_mode()?;
  execute!(
      terminal.backend_mut(),
      LeaveAlternateScreen,
      DisableMouseCapture
  )?;
  terminal.show_cursor()?;
  
  Ok(())

}