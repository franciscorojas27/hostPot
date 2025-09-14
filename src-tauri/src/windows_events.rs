use tauri::{AppHandle, Result, WebviewWindow, WindowEvent};
use tauri_plugin_dialog::{
    DialogExt, MessageDialogButtons, MessageDialogKind, MessageDialogResult,
};

pub fn windows_events(app: &AppHandle, window: &WebviewWindow) -> Result<()> {
    let app_handle = app.clone();
    let win = window.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            let dialog = app_handle
                .dialog()
                .message("Are you sure?")
                .kind(MessageDialogKind::Info)
                .title(app_handle.package_info().name.clone())
                .buttons(MessageDialogButtons::YesNoCancelCustom(
                    "Hide".to_string(),
                    "Close".to_string(),
                    "Cancel".to_string(),
                ));

            match dialog.blocking_show_with_result() {
                MessageDialogResult::Custom(ref s) if s == "Close" => {
                    let _ = win.destroy();
                }
                MessageDialogResult::Custom(ref s) if s == "Hide" => {
                    api.prevent_close();
                    let _ = win.hide();
                }
                MessageDialogResult::Custom(ref s) if s == "Cancel" => {
                    api.prevent_close();
                }
                _ => {}
            }
        }
    });
    Ok(())
}
