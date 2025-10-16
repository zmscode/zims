mod password;

use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, Runtime, Window};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use password::vault::{VaultState, VaultStateMutex};

#[tauri::command]
async fn force_focus<R: Runtime>(window: Window<R>) -> Result<(), String> {
    window.set_focus().map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSApp, NSApplication};
        unsafe {
            let app = NSApp();
            app.activateIgnoringOtherApps_(cocoa::base::YES);
        }
    }

    Ok(())
}

#[tauri::command]
async fn toggle_window_visibility<R: Runtime>(window: Window<R>) -> Result<(), String> {
    if window.is_visible().map_err(|e| e.to_string())? {
        window.hide().map_err(|e| e.to_string())?;
    } else {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;

        #[cfg(target_os = "macos")]
        {
            use cocoa::appkit::{NSApp, NSApplication};
            unsafe {
                let app = NSApp();
                app.activateIgnoringOtherApps_(cocoa::base::YES);
            }
        }
    }
    Ok(())
}

#[tauri::command]
async fn enable_clickthrough<R: Runtime>(window: Window<R>) -> Result<(), String> {
    window
        .set_ignore_cursor_events(true)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn disable_clickthrough<R: Runtime>(window: Window<R>) -> Result<(), String> {
    window
        .set_ignore_cursor_events(false)
        .map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSApp, NSApplication};
        unsafe {
            let app = NSApp();
            app.activateIgnoringOtherApps_(cocoa::base::YES);
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(VaultState::new()))
        .invoke_handler(tauri::generate_handler![
            force_focus,
            enable_clickthrough,
            disable_clickthrough,
            toggle_window_visibility,
            password::vault::initialize_vault,
            password::vault::unlock_vault,
            password::vault::lock_vault,
            password::vault::vault_exists,
            password::vault::is_vault_unlocked,
            password::vault::get_all_passwords,
            password::vault::get_password,
            password::vault::create_password,
            password::vault::update_password,
            password::vault::delete_password,
            password::vault::generate_password,
            password::vault::check_password_strength,
        ])
        .setup(|app| {
            setup_global_shortcuts(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_global_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    app.global_shortcut()
        .on_shortcut("CommandOrControl+Shift+Space", {
            let app_handle = app.clone();
            move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.emit("toggle-command-palette", ());

                        if let Ok(is_visible) = window.is_visible() {
                            if !is_visible {
                                let _ = window.show();
                                let _ = window.set_focus();

                                #[cfg(target_os = "macos")]
                                {
                                    use cocoa::appkit::{NSApp, NSApplication};
                                    unsafe {
                                        let app = NSApp();
                                        app.activateIgnoringOtherApps_(cocoa::base::YES);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })?;

    Ok(())
}
