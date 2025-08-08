use dioxus::{prelude::*};
use tungstenite::connect;
use async_std::stream::StreamExt;

// use dioxus_free_icons::{
//     icons::fa_solid_icons::{
//         FaLink, FaComputerMouse,FaPlus, FaMinus, FaArrows
//     },
//     Icon
// };

use dioxus_free_icons:: {
    Icon,
    icons:: {
        md_content_icons:: {MdLink, MdLinkOff, MdAdd, MdRemove},
        md_hardware_icons::MdMouse,
        md_action_icons::MdSyncAlt
    }
};






const MAIN_CSS:Asset =  asset!("/assets/main.css");
const NORMALIZE_CSS:Asset =  asset!("/assets/normalize.css");
const FAVICON: Asset = asset!("/assets/favicon.ico");

static CONNECTED: GlobalSignal<bool> = Signal::global(|| false);
static IP_ADDR: GlobalSignal<String> = Signal::global(|| "192.168.1.2".to_string());
static MOVE_START: GlobalSignal<(f64, f64)> = Signal::global(|| (0., 0.));
static DEBUG_DATA: GlobalSignal<String> = Signal::global(|| "adasdasd".to_string());

fn main() {
    dioxus::launch(App);
}


struct Cmd(String); 

// ---

#[component]
fn App() -> Element {

    use_coroutine(move | mut rx : UnboundedReceiver<Cmd> | async move {
        *CONNECTED.write() = false;
        let  Ok((mut socket, _ )) = connect(format!("ws://{}:7878", *IP_ADDR.read())) else {
            return ;
        };
        *CONNECTED.write() = true;
        while let Some(command) = rx.next().await {
            *DEBUG_DATA.write() = command.0.clone();
            socket.send(command.0.into()).unwrap();    
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
        Ip{},
        Coordinates{},
        Value{}        
        // Debug{}
    }
}


#[component]
fn Ip() -> Element {

    let mut ch = use_coroutine_handle::<Cmd>();
    rsx! {

        div {
            class: "ip",  
            input {
                value: "{*IP_ADDR.read()}",
                oninput: move | ev |  *IP_ADDR.write() =  ev.value()
            }
            button { 
                onclick: move |_|  ch.restart() ,
                if*CONNECTED.read() {
                    Icon {class: "icon", icon: MdLink}
                } else {
                    Icon {class: "icon", icon: MdLinkOff}
                }
            }
        }
    }
}

// ---

#[component]
fn Coordinates() -> Element {
    let ch = use_coroutine_handle::<Cmd>();

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
                onclick: move | _ |  ch.send(Cmd("c".to_string())),
                ondoubleclick: move | _ |  ch.send(Cmd("d".to_string())),
                Icon {class: "icon", icon: MdMouse}
            }
        }
    }
}

// ---

#[component]
fn Value() -> Element {
    let ch = use_coroutine_handle::<Cmd>();
    rsx! {
        div {
            class: "value",
            div {
                class: "pad",
                ontouchstart: move |ev|  {
                    let cc = ev.data.touches()[0].client_coordinates();
                    *MOVE_START.write() = (cc.x , cc.y); 
                    *DEBUG_DATA.write() = "touch".to_string();
                },
                ontouchmove: move |ev|  {
                    let cc = ev.data.touches()[0].client_coordinates();
                    let start = *MOVE_START.read();
                    let delta = (cc.x - start.0, cc.y - start.1);
                    *MOVE_START.write() = (cc.x, cc.y);
                    ch.send(Cmd(format!("v:{},{}", delta.0.round(), delta.1.round())));
                },

                Icon{class: "icon", icon: MdRemove}
                Icon{class: "icon", icon: MdSyncAlt}
                Icon{class: "icon", icon: MdAdd}
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
