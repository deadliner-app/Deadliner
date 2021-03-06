use std::sync::{Arc, Mutex};

// System tray is supported and availabled only if `tray` feature is enabled.
// Platform: Windows, Linux and macOS.
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
// #[cfg(feature = "tray")]
pub fn bg_system_tray(exit: Arc<Mutex<bool>>) {
    use deadliner_gui::{get_current_file_ext, new_path};
    #[cfg(target_os = "linux")]
    use std::path::Path;
    use std::process::{Child, Command};
    #[cfg(target_os = "macos")]
    use tao::platform::macos::{CustomMenuItemExtMacOS, NativeImage, SystemTrayBuilderExtMacOS};
    use tao::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{ContextMenu as Menu, MenuItemAttributes, MenuType},
        system_tray::SystemTrayBuilder,
    };

    let event_loop = EventLoop::new();

    let mut tray_menu = Menu::new();

    let mut gui_handler: Option<Child> = None;

    let show_gui = tray_menu.add_item(MenuItemAttributes::new("Open/Close Window"));
    let quit_element = tray_menu.add_item(MenuItemAttributes::new("Quit Program"));

    // Windows require Vec<u8> ICO file
    #[cfg(target_os = "windows")]
    let icon = include_bytes!("../icons/icon.ico").to_vec();
    // macOS require Vec<u8> PNG file
    #[cfg(target_os = "macos")]
    let icon = include_bytes!("../icons/icon.png").to_vec();
    // Linux require Pathbuf to PNG file
    #[cfg(target_os = "linux")]
    let icon = new_path("icons/icon.png");

    // Menu is shown with left click on macOS and right click on Windows.
    #[cfg(target_os = "macos")]
    let mut system_tray = SystemTrayBuilder::new(icon.clone(), Some(tray_menu))
        .with_icon_as_template(true)
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_os = "macos"))]
    let mut system_tray = SystemTrayBuilder::new(icon.clone(), Some(tray_menu))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Acquire the lock to check if an exit signal is sent.
        let exit_lock = exit.try_lock();
        if let Ok(value) = exit_lock {
            if *value == true {
                *control_flow = ControlFlow::Exit;
            }
        }

        let open_gui = |icon_click: bool, gui_handler: &mut Option<Child>| {
            let is_gui_running = if gui_handler.is_some() {
                let gui_child = gui_handler.as_mut().unwrap();
                match gui_child.try_wait() {
                    Ok(Some(_status)) => false,
                    _ => true,
                }
            } else {
                false
            };

            if is_gui_running {
                if !icon_click {
                    gui_handler.as_mut().unwrap().kill();
                    *gui_handler = None;
                }
            } else {
                let gui_exec = format!("deadliner{}", &get_current_file_ext());

                let child_process = Command::new(new_path(&gui_exec)).spawn().unwrap();

                *gui_handler = Some(child_process);
            }
        };

        match event {
            Event::WindowEvent {
                event, window_id, ..
            } => {
                if event == WindowEvent::CloseRequested {
                    // Change tray icon
                    system_tray.set_icon(icon.clone());
                }
            }
            // on Windows, habitually, we show the window with left click
            #[cfg(target_os = "windows")]
            Event::TrayEvent {
                event: tao::event::TrayEvent::LeftClick,
                ..
            } => open_gui(true, &mut gui_handler),
            // left click on menu item
            Event::MenuEvent {
                menu_id,
                // specify only context menu's
                origin: MenuType::ContextMenu,
                ..
            } => {
                // click on `quit` item
                if menu_id == quit_element.clone().id() {
                    if gui_handler.is_some() {
                        gui_handler.as_mut().unwrap().kill();
                    }
                    // tell our app to close at the end of the loop.
                    *control_flow = ControlFlow::Exit;
                }

                if menu_id == show_gui.clone().id() {
                    // Pass an argument to tell it to only style
                    open_gui(false, &mut gui_handler)
                }
            }
            _ => (),
        }
    });
}
