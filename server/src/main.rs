use std::thread;
 use winit::event_loop::EventLoop;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum UserEvents {
    RightClickTrayIcon,
    SetConnected,
    SetDisconnected,
    Exit
}


mod server;
mod tray;



fn main() {
    let event_loop = EventLoop::<UserEvents>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();
    thread::spawn(|| server::Server::new(7878, proxy).start());
    tray::create_window(event_loop);
}

