use comfy_table::{Cell, Color, Table};
use colored::Colorize;

#[derive(Debug, Clone, clap::ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Pretty,
    Json,
}

pub fn print_message(msg: &str, success: bool, format: &OutputFormat) {
    match format {
        OutputFormat::Json => {
            let obj = serde_json::json!({ "message": msg, "success": success });
            println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        }
        OutputFormat::Pretty => {
            if success {
                println!("{} {}", "✓".green(), msg);
            } else {
                println!("{} {}", "✗".red(), msg);
            }
        }
    }
}

pub fn print_errors(errors: &[crate::api::models::UserError], format: &OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&errors).unwrap());
        }
        OutputFormat::Pretty => {
            for err in errors {
                let field = err.field.as_deref().unwrap_or("");
                let code = err.code.as_deref().unwrap_or("");
                if !field.is_empty() {
                    eprintln!("  {} {}: {}", "✗".red(), field.yellow(), err.message);
                } else {
                    eprintln!("  {} {}", "✗".red(), err.message);
                }
                if !code.is_empty() {
                    eprintln!("    {} {}", "code:".dimmed(), code.dimmed());
                }
            }
        }
    }
}

pub fn print_user(user: &crate::api::models::User, format: &OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&user).unwrap());
        }
        OutputFormat::Pretty => {
            let mut table = Table::new();
            table.load_preset(comfy_table::presets::NOTHING);
            table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
            table.add_rows(vec![
                vec![Cell::new("ID"), Cell::new(&user.id)],
                vec![Cell::new("Email"), Cell::new(&user.email)],
                vec![Cell::new("Name"), Cell::new(user.full_name.as_deref().unwrap_or("-"))],
                vec![Cell::new("Display"), Cell::new(&user.display_name)],
                vec![Cell::new("Role"), Cell::new(user.role.as_deref().unwrap_or("-"))],
                vec![Cell::new("Roles"), Cell::new(user.roles.as_ref().map(|r| r.join(", ")).as_deref().unwrap_or("-"))],
                vec![Cell::new("Last Login"), Cell::new(user.last_login_at.as_deref().unwrap_or("never"))],
                vec![Cell::new("Created"), Cell::new(&user.created_at)],
            ]);
            println!("{table}");
        }
    }
}

pub fn print_users_list(users: &[crate::api::models::User], format: &OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&users).unwrap());
        }
        OutputFormat::Pretty => {
            let mut table = Table::new();
            table.load_preset(comfy_table::presets::UTF8_FULL);
            table.set_header(vec![
                Cell::new("ID").fg(Color::Cyan),
                Cell::new("Email").fg(Color::Cyan),
                Cell::new("Name").fg(Color::Cyan),
                Cell::new("Role").fg(Color::Cyan),
                Cell::new("Last Login").fg(Color::Cyan),
            ]);
            for user in users {
                table.add_row(vec![
                    Cell::new(&user.id),
                    Cell::new(&user.email),
                    Cell::new(user.full_name.as_deref().unwrap_or("-")),
                    Cell::new(user.role.as_deref().unwrap_or("-")),
                    Cell::new(user.last_login_at.as_deref().unwrap_or("never")),
                ]);
            }
            println!("{table}");
        }
    }
}

pub fn print_audit_logs(logs: &[crate::api::models::AuditLog], format: &OutputFormat) {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&logs).unwrap());
        }
        OutputFormat::Pretty => {
            let mut table = Table::new();
            table.load_preset(comfy_table::presets::UTF8_FULL);
            table.set_header(vec![
                Cell::new("ID").fg(Color::Cyan),
                Cell::new("Action").fg(Color::Cyan),
                Cell::new("Resource").fg(Color::Cyan),
                Cell::new("Result").fg(Color::Cyan),
                Cell::new("User").fg(Color::Cyan),
                Cell::new("IP").fg(Color::Cyan),
                Cell::new("Date").fg(Color::Cyan),
            ]);
            for log in logs {
                let user_display = log
                    .user
                    .as_ref()
                    .map(|u| u.display_name.clone())
                    .unwrap_or_else(|| "-".to_string());
                let result_color = match log.result.as_str() {
                    "success" => Color::Green,
                    "failure" => Color::Red,
                    "blocked" => Color::Yellow,
                    _ => Color::White,
                };
                table.add_row(vec![
                    Cell::new(&log.id),
                    Cell::new(&log.action),
                    Cell::new(&log.resource),
                    Cell::new(&log.result).fg(result_color),
                    Cell::new(&user_display),
                    Cell::new(log.ip_address.as_deref().unwrap_or("-")),
                    Cell::new(&log.created_at),
                ]);
            }
            println!("{table}");
        }
    }
}
