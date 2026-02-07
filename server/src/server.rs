use std::{
    net::{TcpListener, UdpSocket}, thread::{sleep, spawn}, time::Duration
};
use enigo::Button;
use tungstenite::{accept, Message};
use enigo::{Enigo, Mouse, Settings, Direction::Click};
use winit::event_loop::EventLoopProxy;
use crate::UserEvents;

pub struct Server {
    controller: Enigo,
    port: u16,
    proxy: EventLoopProxy<UserEvents>
}

const PORT: u32 = 1717;
const D_REQUEST: &str = "LMDISCOVER";


// ---

impl Server {
    pub fn new(port: u16, proxy: EventLoopProxy<UserEvents>) -> Self{
        Self {
            controller: Enigo::new(&Settings::default()).unwrap(),
            port, 
            proxy
        } 
    }

    // ---
    
    fn discover() -> std::io::Result<()>{
        let socket = UdpSocket::bind(format!("0.0.0.0:{PORT}"))?;
        let mut buf = [0; 20];
        loop {
            match socket.recv_from(&mut buf)  {
                Ok((size, addr)) =>  {
                    let message = String::from_utf8_lossy(&buf[..size]);
                    if message == D_REQUEST.to_string() {
                        let resp = D_REQUEST.chars().rev().collect::<String>();
                        socket.send_to(resp.as_bytes(), addr)?;
                        // return Ok(());
                    }
                }
                Err(e) => return Err(e) 
            }
        }
    }

    // ---

    pub fn start(&mut self) {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(addr).unwrap();
        spawn(move || Self::discover().unwrap());
        
        for stream in listener.incoming() {
            let mut websocket = accept(stream.unwrap()).unwrap();
            self.proxy.send_event(UserEvents::SetConnected).unwrap();
            loop {
                let r = websocket.read();
                if let Ok(msg) = r {
                    self.command(msg);
                } else {
                    self.proxy.send_event(UserEvents::SetDisconnected).unwrap();
                    break;
                }
            }
        }
    } 
    
    // ---

    fn command(&mut self, message: Message) {
        let command = message.to_string();
        if let Some((axis, value_str)) =  command.split_once(':') {
            let xy = value_str.split(',').map(|v| v.parse::<i32>().unwrap_or(0) * 2 ).collect::<Vec<i32>>();
            let _ = match axis {
                "xy" => self.controller.move_mouse(xy[0], xy[1], enigo::Coordinate::Rel),
                "v" =>  self.controller.scroll(-xy[0] / 30, enigo::Axis::Vertical),
                _ => Ok(())
            };
        } else if ["c", "d"].contains(&command.as_str()) {
            let _ = self.controller.button(Button::Left, Click);
            if command == "d" {
                sleep(Duration::from_millis(250));
                let _ = self.controller.button(Button::Left, Click);   
            }
        }            
    }    
}
