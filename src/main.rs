use std::{env, process::Command};

use features::{
    icons::get_icon,
    session_commands::{get_custom_commands, get_session_commands},
};
use sniffer_rs::sniffer::Sniffer;
use tigris_rs::features::{
    actions::{ResultAction, RunExtensionAction},
    api::{
        get_extension_request, send_search_results,
        RequestType::{GetResults, RunAction},
    },
    search_results::SearchResult,
    settings::get_extension_setting,
};

pub mod features;

const EXTENSION_ID: &str = "session-manager";

fn main() {
    let extension_request = get_extension_request();

    match extension_request.request_type {
        GetResults => {
            let request = extension_request.get_results_request.unwrap();
            let search_text = request.search_text;
            let sniffer = Sniffer::new();
            let mut results = Vec::<SearchResult>::new();

            if sniffer.matches("lock", &search_text) {
                results.push(get_search_result("Lock", "lock", "lock", false));
            }

            if sniffer.matches("log out/sign out", &search_text) {
                results.push(get_search_result("Log Out", "logout", "logout", true));
            }

            if sniffer.matches("shutdown/power off", &search_text) {
                results.push(get_search_result("Shutdown", "shutdown", "power", true));
            }

            if sniffer.matches("restart/reboot", &search_text) {
                results.push(get_search_result("Reboot", "reboot", "reboot", true));
            }

            if sniffer.matches("hibernate", &search_text) {
                results.push(get_search_result("Hibernate", "hibernate", "sleep", false));
            }

            if sniffer.matches("suspend", &search_text) {
                results.push(get_search_result("Suspend", "suspend", "sleep", false));
            }

            send_search_results(&results);
        }
        RunAction => {
            let request = extension_request.run_action_request.unwrap();
            let action = request.action;
            let preset = get_extension_setting(EXTENSION_ID, "preset").unwrap();

            let session_commands = match preset == "auto" {
                true => {
                    let env =
                        env::var("XDG_CURRENT_DESKTOP").expect("Error getting session environment");

                    get_session_commands(&env)
                }
                false => {
                    if preset == "custom" {
                        get_custom_commands()
                    } else {
                        get_session_commands(&preset)
                    }
                }
            };

            let command = match action.as_str() {
                "shutdown" => session_commands.shutdown,
                "reboot" => session_commands.reboot,
                "suspend" => session_commands.suspend,
                "hibernate" => session_commands.hibernate,
                "logout" => session_commands.logout,
                _ => session_commands.lock,
            };

            Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("Error running command");
        }
        _ => {}
    }
}

fn get_search_result(title: &str, action: &str, icon: &str, dangerous: bool) -> SearchResult {
    let icon = get_icon(icon);

    let result_action =
        ResultAction::new_run_extension_action(&RunExtensionAction::new(EXTENSION_ID, action))
            .set_require_confirmation(dangerous);

    SearchResult::new(title)
        .set_icon_path(&icon)
        .set_icon_color("accent")
        .set_action(&result_action)
}
