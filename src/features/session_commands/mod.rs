use tigris_rs::features::settings::get_extension_setting;

use crate::EXTENSION_ID;

pub struct SessionCommands {
    pub shutdown: String,
    pub restart: String,
    pub suspend: String,
    pub hibernate: String,
    pub logout: String,
    pub lock: String,
}

impl SessionCommands {
    pub fn new(
        shutdown: &str,
        restart: &str,
        suspend: &str,
        hibernate: &str,
        logout: &str,
        lock: &str,
    ) -> Self {
        Self {
            shutdown: shutdown.to_string(),
            restart: restart.to_string(),
            suspend: suspend.to_string(),
            hibernate: hibernate.to_string(),
            logout: logout.to_string(),
            lock: lock.to_string(),
        }
    }
}

pub fn get_custom_commands() -> SessionCommands {
    SessionCommands::new(
        &get_extension_setting(EXTENSION_ID, "custom-shutdown").unwrap(),
        &get_extension_setting(EXTENSION_ID, "custom-reboot").unwrap(),
        &get_extension_setting(EXTENSION_ID, "custom-suspend").unwrap(),
        &get_extension_setting(EXTENSION_ID, "custom-hibernate").unwrap(),
        &get_extension_setting(EXTENSION_ID, "custom-logout").unwrap(),
        &get_extension_setting(EXTENSION_ID, "custom-lock").unwrap(),
    )
}

pub fn get_session_commands(environment: &str) -> SessionCommands {
    match environment.to_lowercase().as_str() {
        "kde" => SessionCommands::new(
            "systemctl poweroff",
            "systemctl reboot",
            "systemctl suspend",
            "systemctl hibernate",
            "qdbus6 org.kde.Shutdown /Shutdown  org.kde.Shutdown.logout",
            "loginctl lock-session",
        ),
        "gnome" => SessionCommands::new(
            "systemctl poweroff",
            "systemctl reboot",
            "systemctl suspend",
            "systemctl hibernate",
            "gnome-session-quit --no-prompt",
            "dbus-send --type=method_call --dest=org.gnome.ScreenSaver /org/gnome/ScreenSaver org.gnome.ScreenSaver.Lock",
        ),
        "x-cinnamon" => SessionCommands::new(
            "systemctl poweroff",
            "systemctl reboot",
            "systemctl suspend",
            "systemctl hibernate",
            "cinnamon-session-quit --logout --force",
            "cinnamon-screensaver-command --lock",
        ),
        "hyprland" => SessionCommands::new(
            "systemctl poweroff",
            "systemctl reboot",
            "systemctl suspend",
            "systemctl hibernate",
            "hyprctl dispatch exit",
            "hyprlock",
        ),
        _ => SessionCommands::new(
            "systemctl poweroff",
            "systemctl reboot",
            "systemctl suspend",
            "systemctl hibernate",
            "notify-send 'Environment not coded. Please open a issue'",
            "notify-send 'Environment not coded. Please open a issue'",
        ),
    }
}
