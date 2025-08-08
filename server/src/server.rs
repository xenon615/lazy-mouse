use std::{
    net::TcpListener,
    thread::sleep,
    time::Duration
};
use tungstenite::{accept, Message};
use mouce::{Mouse, MouseActions, 
    common::{
        ScrollDirection,
        ScrollUnit,
        MouseButton
    }
};

pub struct Server {
    mouse: Mouse
}

// ---

impl Server {
    pub fn new() -> Self{
        Self {mouse: Mouse::new()}
    }

    // ---

    pub fn start(&self, port: u16) {
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let r = websocket.read();
                if let Ok(msg) = r {
                    self.command(msg);
                } else {
                    break;
                }
            }
        }
    } 

    // ---

    fn command(&self, message: Message) {
        let command = message.to_string();
        if let Some((axis, value_str)) =  command.split_once(':') {
            let xy = value_str.split(',').map(|v| v.parse::<f32>().unwrap_or(0.0) * 2.).collect::<Vec<f32>>();
            let _ = match axis {
                "xy" => self.mouse.move_relative(xy[0] as _, xy[1] as _),
                "v" =>  self.mouse.scroll_wheel(if xy[0] > 0. {ScrollDirection::Up} else {ScrollDirection::Down},ScrollUnit::Pixel, xy[0].abs() as _),
                _ => Ok(())
            };
        } else if ["c", "d"].contains(&command.as_str()) {
            let _ = self.mouse.click_button(MouseButton::Left);
            if command == "d" {
                 sleep(Duration::from_millis(250));
                 let _ = self.mouse.click_button(MouseButton::Left);   
            }
        }            
    }
}