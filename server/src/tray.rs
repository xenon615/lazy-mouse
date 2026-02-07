use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

use trayicon::{Icon, MenuBuilder, TrayIcon, TrayIconBuilder};


use crate::UserEvents;
// ---

pub fn create_window(event_loop: EventLoop<UserEvents>) {
    let proxy = event_loop.create_proxy();
    let icon_idle = Icon::from_buffer(include_bytes!("../assets/icon-idle.ico"), None, None).unwrap()  ;
    let icon_connected = Icon::from_buffer(include_bytes!("../assets/icon-connected.ico"), None, None).unwrap()  ;

    let tray_icon = TrayIconBuilder::new()
        .sender(move |e: &UserEvents| {
            let _ = proxy.send_event(e.clone());
        })
        .icon(icon_idle.clone())
        .tooltip(
            format!("Local Ip:{}", local_ip_address::local_ip().unwrap()).as_str()
        )
        .on_right_click(UserEvents::RightClickTrayIcon)
        .menu(MenuBuilder::new().item("Exit", UserEvents::Exit))
        .build()
        .unwrap();

    let mut app = MyApplication {
        window: None,
        active_icon: tray_icon,
        icon_idle,
        icon_connected
    };
    event_loop.run_app(&mut app).unwrap();
}

struct MyApplication {
    window: Option<Window>,
    active_icon: TrayIcon<UserEvents>,
    icon_idle: Icon,
    icon_connected: Icon
}

impl ApplicationHandler<UserEvents> for MyApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    // ---

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
    
    // ---

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvents) {
        match event {
            UserEvents::Exit => event_loop.exit(),
            UserEvents::RightClickTrayIcon => self.active_icon.show_menu().unwrap(),
            UserEvents::SetConnected => self.active_icon.set_icon(&self.icon_connected).unwrap(),
            UserEvents::SetDisconnected => self.active_icon.set_icon(&self.icon_idle).unwrap()
        }
    }
}