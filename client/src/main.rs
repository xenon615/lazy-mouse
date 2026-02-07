use dioxus::{prelude::*};
use tungstenite::connect;
use smol::stream::StreamExt;

use std::{net::UdpSocket, time::Duration};

use dioxus_free_icons:: {
    Icon,
    icons:: {
        md_content_icons:: {MdAdd, MdRemove},
        md_hardware_icons::MdMouse,
        md_action_icons::MdSyncAlt
    }
};

const MAIN_CSS:Asset =  asset!("/assets/main.css");
const NORMALIZE_CSS:Asset =  asset!("/assets/normalize.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

static CONNECTED: GlobalSignal<bool> = Signal::global(|| false);
static MOVE_START: GlobalSignal<(f64, f64)> = Signal::global(|| (0., 0.));
static DEBUG_DATA: GlobalSignal<String> = Signal::global(|| "---".to_string());

const D_PORT: u32 = 1717;
const W_PORT: u32 = 7878;
const D_REQUEST: &str = "LMDISCOVER";


fn main() {
    dioxus::launch(App);
}


struct Cmd(String); 

// ---

#[component]
fn App() -> Element {
    use_coroutine(move | mut rx : UnboundedReceiver<Cmd> | async move {
        let Ok(addr) = discover() else {
            return ;    
        };
        let addr = addr.split_once(':').unwrap().0.to_string();
        *CONNECTED.write() = false;
        let  Ok((mut socket, _ )) = connect(format!("ws://{}:{W_PORT}", addr)) else {            
            return ;
        };

        *CONNECTED.write() = true;
        while let Some(command) = rx.next().await {
            // *DEBUG_DATA.write() = command.0.clone();
            if socket.send(command.0.into()).is_err() {
                *CONNECTED.write() = false;
                return ;
            }    
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet{href : MAIN_CSS}
        document::Stylesheet{href : NORMALIZE_CSS}
        Wrap {},
    }
}

// ---

#[component]
fn Wrap() -> Element {
    rsx! {
        Coordinates{},
        Value{}        
        // Debug{}
    }
}

// ---

#[component]
fn Coordinates() -> Element {
    let mut ch = use_coroutine_handle::<Cmd>();
    let icon_class = if *CONNECTED.read() {"icon"} else {"icon disconnected"};
    rsx!{
        div {
            class: "coordinates",
            div {
                class: "pad",
                ontouchstart: move |ev|  {
                    let cc = ev.data.touches()[0].client_coordinates();
                    *MOVE_START.write() = (cc.x , cc.y); 
                },
                ontouchmove: move |ev|  {
                    let cc = ev.data.touches()[0].client_coordinates();
                    let start = *MOVE_START.read();
                    let delta = (cc.x - start.0, cc.y - start.1);
                    *MOVE_START.write() = (cc.x, cc.y);
                    ch.send(Cmd(format!("xy:{},{}", delta.0.round(), delta.1.round())));
                },
                onclick: move | _ |  {
                    if *CONNECTED.read() {
                        ch.send(Cmd("c".to_string()))    
                    } else {
                        ch.restart();
                    }
                    ch.send(Cmd("c".to_string()))
                },
                ondoubleclick: move | _ |  ch.send(Cmd("d".to_string())),

                Icon {class: icon_class, icon: MdMouse}
            }
        }
    }
}

// ---

#[component]
fn Value() -> Element {
    let ch = use_coroutine_handle::<Cmd>();
    let icon_class = if *CONNECTED.read() {"icon"} else {"icon disconnected"};
    rsx! {
        div {
            class: "value",
            div {
                class: "pad",
                ontouchstart: move |ev|  {
                    let cc = ev.data.touches()[0].client_coordinates();
                    *MOVE_START.write() = (cc.x , cc.y); 
                    // *DEBUG_DATA.write() = "touch".to_string();
                },
                ontouchmove: move |ev|  {
                    let cc = ev.data.touches()[0].client_coordinates();
                    let start = *MOVE_START.read();
                    let delta = (cc.x - start.0, cc.y - start.1);
                    *MOVE_START.write() = (cc.x, cc.y);
                    ch.send(Cmd(format!("v:{},{}", delta.0.round(), delta.1.round())));
                },
                Icon{class: icon_class, icon: MdRemove}
                Icon{class: icon_class, icon: MdSyncAlt}
                Icon{class: icon_class, icon: MdAdd}
            }
        },
    }
}

// ---

#[component]
fn Debug() -> Element {
    rsx!{
        div {
            class: "debug",
            "{*DEBUG_DATA.read()}"
        }
    }
}

// ---

#[allow(dead_code)]
fn touch_debug(t: Event<TouchData>) {
    *DEBUG_DATA.write() = format!(" {:?}", t.data());
}

// ---

fn discover() -> Result<String, std::io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.set_read_timeout(Some(Duration::from_millis(500)))?;
    socket.send_to(D_REQUEST.as_bytes(), format!("255.255.255.255:{D_PORT}"))?;
    let mut buf = [0; 20];
    match socket.recv_from(&mut buf) {
        Ok((size, addr)) =>  {
            let message = String::from_utf8_lossy(&buf[..size]);
            if message == D_REQUEST.chars().rev().collect::<String>() {
                return Ok(format!("{addr}"));
            } else {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "")) ;
            }
        }
        Err(e) => return Err(e)
    }        
}
