use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Result,
};
fn setup_menu(app: &AppHandle) -> Result<tauri::menu::Menu<tauri::Wry>> {
    let quit_item = PredefinedMenuItem::quit(app, Some("Quit"))?;
    let menu = MenuBuilder::new(app)
        .item(&MenuItemBuilder::new("show").id("show").build(app)?)
        .separator()
        .item(&quit_item)
        .build()?;
    Ok(menu)
}

pub fn setup_tray(app: &AppHandle) -> Result<()> {
    let menu = setup_menu(app).unwrap();
    TrayIconBuilder::new()
        .show_menu_on_left_click(true)
        .icon(app.default_window_icon().unwrap().clone()) // agrega el icono desde el icono de main de la app
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "show" => {
                app.get_webview_window("main").unwrap().show().unwrap();
            }
            _ => (),
        })
        .on_tray_icon_event(|_app, event| match event {
            TrayIconEvent::Click {
                button, position, ..
            } => {
                println!("Click en tray con botón: {:?} en {:?}", button, position);
            }
            TrayIconEvent::DoubleClick { position, .. } => {
                println!("Doble click en tray en {:?}", position);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
