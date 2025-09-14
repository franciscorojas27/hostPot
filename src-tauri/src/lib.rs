mod tray;
mod windows_events;
use windows_events::windows_events;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value};
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tauri::AppHandle;
use tauri::{Manager, State, Wry};
use tauri_plugin_notification::NotificationBuilder;
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_store::{Store, StoreExt};
use tray::setup_tray;
static IS_AUTH: AtomicBool = AtomicBool::new(true);
#[derive(Debug, Serialize)]
struct HostedNetworkSettings {
    mode: Option<String>,
    ssid_name: Option<String>,
    max_clients: Option<u32>,
    authentication: Option<String>,
    cipher: Option<String>,
    status: Option<String>,
    key: Option<String>,
}
struct HostedNetworkGlobal(Arc<HostedNetworkSettings>);
struct ManagedStore(Arc<Store<Wry>>);
impl Deref for ManagedStore {
    type Target = Arc<Store<Wry>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    password: String,
}
impl User {
    fn new(name: &str, password: &str) -> Self {
        User {
            name: name.to_string(),
            password: password.to_string(),
        }
    }
}
#[tauri::command]
fn logout() {
    IS_AUTH.store(false, Ordering::SeqCst);
}
#[tauri::command]
fn is_authenticated() -> bool {
    IS_AUTH.load(Ordering::SeqCst)
}
#[tauri::command]
fn login(user: User, store: State<ManagedStore>) -> bool {
    let value = store.get("usuario").unwrap();
    let validate_user: User = from_value(value).unwrap();
    if user.name == validate_user.name && user.password == validate_user.password {
        IS_AUTH.store(true, Ordering::SeqCst);
        return true;
    }
    false
}
#[cfg(target_os = "windows")]
fn notification_event(app_handle: &AppHandle, title: String, message: String) {
    let builder: NotificationBuilder<_> = app_handle.notification().builder();
    builder.title(title).body(message).show().unwrap();
}
#[tauri::command]
fn get_hosted_network_settings_to_fronted(
    host: State<HostedNetworkGlobal>,
) -> HostedNetworkSettings {
    HostedNetworkSettings {
        mode: host.inner().0.mode.clone(),
        ssid_name: host.inner().0.ssid_name.clone(),
        max_clients: host.inner().0.max_clients,
        authentication: host.inner().0.authentication.clone(),
        cipher: host.inner().0.cipher.clone(),
        status: host.inner().0.status.clone(),
        key: host.inner().0.key.clone(),
    }
}
#[cfg(target_os = "windows")]
fn parse_hosted_network_output(output: &str, key: String) -> HostedNetworkSettings {
    let mut mode = None;
    let mut ssid_name = None;
    let mut max_clients = None;
    let mut authentication = None;
    let mut cipher = None;
    let mut status = None;

    for line in output.lines() {
        let line = line.trim();
        // English and Spanish support
        if line.starts_with("Mode") || line.starts_with("Modo") {
            mode = line.splitn(2, ':').nth(1).map(|s| s.trim().to_string());
        } else if line.starts_with("SSID name") || line.starts_with("Nombre SSID") {
            ssid_name = line
                .splitn(2, ':')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').to_string());
        } else if line.starts_with("Max number of clients")
            || line.starts_with("N máximo de clientes")
        {
            max_clients = line
                .splitn(2, ':')
                .nth(1)
                .and_then(|s| s.trim().parse::<u32>().ok());
        } else if line.starts_with("Authentication") || line.starts_with("Autenticación") {
            authentication = line.splitn(2, ':').nth(1).map(|s| s.trim().to_string());
        } else if line.starts_with("Cipher") || line.starts_with("Cifrado") {
            cipher = line.splitn(2, ':').nth(1).map(|s| s.trim().to_string());
        } else if line.starts_with("Status") || line.starts_with("Estado") {
            status = line.splitn(2, ':').nth(1).map(|s| s.trim().to_string());
        }
    }

    HostedNetworkSettings {
        mode,
        ssid_name,
        max_clients,
        authentication,
        cipher,
        status,
        key: Some(key),
    }
}
#[cfg(target_os = "windows")]
fn get_hosted_network_settings(key: String) -> Option<HostedNetworkSettings> {
    let output = Command::new("netsh")
        .args(&["wlan", "show", "hostednetwork"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    Some(parse_hosted_network_output(&output_str, key))
}
#[cfg(target_os = "windows")]
#[tauri::command]
fn config_hosted_network(
    ssid: &str,
    key: &str,
    host: State<HostedNetworkGlobal>,
    store: State<ManagedStore>,
) {
    unsafe {
        use windows::{
            core::{w, PCWSTR},
            Win32::UI::Shell::ShellExecuteW,
            Win32::UI::WindowsAndMessaging::SW_HIDE,
        };
        let value = format!(
            "wlan set hostednetwork mode=allow ssid={} key={} keyUsage=persistent",
            ssid, key
        );
        store.set("password", to_value(key).unwrap());
        if let Some(settings) = Arc::get_mut(&mut host.inner().0.clone()) {
            settings.ssid_name = Some(ssid.to_string());
            settings.key = Some(key.to_string());
        }
        // use windows::core::PCWSTR;
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        let value_wide: Vec<u16> = OsStr::new(&value)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let result = ShellExecuteW(
            None,
            w!("runas"),
            w!("netsh"),
            PCWSTR(value_wide.as_ptr()),
            PCWSTR::null(),
            SW_HIDE,
        );

        if result.0 as usize <= 32 {
            eprintln!("ShellExecuteW failed (code <= 32): {:?}", result);
        }
    }
}
#[cfg(target_os = "windows")]
#[tauri::command]
fn stop_hosted_network(app_handle: AppHandle) -> bool {
    use std::os::windows::process::CommandExt;

    let output = Command::new("netsh")
        .args(&["wlan", "stop", "hostednetwork"])
        .creation_flags(0x08000000)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            notification_event(
                &app_handle,
                app_handle.package_info().name.clone(),
                "Hosted network stopped successfully".to_string(),
            );
            true
        }
        _ => {
            notification_event(
                &app_handle,
                app_handle.package_info().name.clone(),
                "Failed to stop hosted network".to_string(),
            );
            false
        }
    }
}
#[tauri::command]
fn get_auto_start(store: State<ManagedStore>) -> bool {
    let value = store.get("autoStart").unwrap();
    let real = match from_value::<String>(value) {
        Ok(s) => s == "true",
        Err(_) => false,
    };
    real
}
#[tauri::command]
fn set_auto_start(flag: bool, store: State<ManagedStore>) {
    store.set("autoStart", to_value(flag.to_string()).unwrap());
}
#[cfg(target_os = "windows")]
#[tauri::command]
fn start_hosted_network(app_handle: AppHandle) -> bool {
    unsafe {
        use windows::{
            core::{w, PCWSTR},
            Win32::UI::Shell::ShellExecuteW,
            Win32::UI::WindowsAndMessaging::SW_HIDE,
        };

        let result = ShellExecuteW(
            None,           // hwnd as Option<HWND>
            w!("runas"),    // lpOperation (verb) - run as administrator
            w!("netsh"),    // lpFile
            w!("start"),    // lpParameters
            PCWSTR::null(), // lpDirectory
            SW_HIDE,        // nShowCmd (SW_HIDE = 0)
        );

        if result.0 as usize <= 32 {
            eprintln!("ShellExecuteW failed (code <= 32): {:?}", result);
            notification_event(
                &app_handle,
                app_handle.package_info().name.clone(),
                "Failed to start hosted network".to_string(),
            );
            return false;
        }
        notification_event(
            &app_handle,
            app_handle.package_info().name.clone(),
            "started hosted network".to_string(),
        );
        return true;
    }
}
#[cfg(target_os = "windows")]
#[tauri::command]
fn is_alive(server: State<HostedNetworkGlobal>) -> bool {
    let status = server.inner().0.status.clone().unwrap_or_default();
    match status.as_str() {
        "Started" => true,
        "Not available" => false,
        "Not started" => false,
        _ => false,
    }
}
fn parse_command_security_password() -> Option<String> {
    let output = Command::new("netsh")
        .args(&["wlan", "show", "hostednetwork", "setting=security"])
        .creation_flags(0x08000000)
        .output()
        .unwrap();
    if !output.status.success() {
        println!("Failed to execute command");
        return None;
    }
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
        let line = line.trim();
        if line.starts_with("User security key") && !line.starts_with("User security key usage")
            || line.starts_with("Clave de seguridad de usuario")
                && !line.starts_with("Clave de seguridad del sistema")
        {
            let key = line.splitn(2, ':').nth(1).map(|s| s.trim()).unwrap_or("");
            if key.is_empty() || key == "<Not specified>" || key == "<No especificada>" {
                return None;
            } else {
                return Some(key.to_string());
            }
        }
    }
    None
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            windows_events(&app.handle(), &window)?;
            setup_tray(&app.handle())?;
            let store = app.store("app_data.json").unwrap();
            if store.get("usuario").is_none() {
                store.set("usuario", to_value(User::new("svhg", "svhg54321")).unwrap());
            }
            if store.get("autoStart").is_none() {
                store.set("autoStart", to_value(false).unwrap());
            }
            let key = match store.get("password") {
                Some(val) => val,
                None => {
                    let password = parse_command_security_password();
                    let value = match password {
                        Some(pass) => to_value(pass).unwrap(),
                        None => to_value("Configure antes de iniciar").unwrap(),
                    };
                    store.set("password", value.clone());
                    value
                }
            };
            let host = HostedNetworkGlobal(Arc::new(
                get_hosted_network_settings(key.clone().to_string()).unwrap_or(
                    HostedNetworkSettings {
                        mode: None,
                        ssid_name: None,
                        max_clients: None,
                        authentication: None,
                        cipher: None,
                        status: None,
                        key: from_value(key).unwrap(),
                    },
                ),
            ));
            app.manage(host);
            app.manage(ManagedStore(store));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            login,
            logout,
            is_authenticated,
            config_hosted_network,
            start_hosted_network,
            stop_hosted_network,
            is_alive,
            get_hosted_network_settings_to_fronted,
            get_auto_start,
            set_auto_start
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
