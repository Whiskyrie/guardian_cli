mod api;
mod cli;
mod config;
mod error;
mod output;

use clap::Parser;
use cli::Commands;
use colored::Colorize;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("{} {}", "error:".red(), e);
        std::process::exit(1);
    }
}

async fn run(cli: cli::Cli) -> error::Result<()> {
    match cli.command {
        Commands::Login { email, password } => {
            cli::login::run(&email, password.as_deref(), &cli.output).await?;
        }
        Commands::Logout => {
            cli::logout::run(&cli.output).await?;
        }
        Commands::Whoami => {
            cli::whoami::run(&cli.output).await?;
        }
        Commands::Token => {
            cli::token::run(&cli.output).await?;
        }
        Commands::User { command } => match command {
            cli::UserCommands::List { role, search } => {
                cli::user::list(role.as_deref(), search.as_deref(), &cli.output).await?;
            }
            cli::UserCommands::Create { email, password, first, last } => {
                cli::user::create(&email, &password, &first, &last, &cli.output).await?;
            }
            cli::UserCommands::Delete { id } => {
                cli::user::delete(&id, &cli.output).await?;
            }
        },
        Commands::Role { command } => match command {
            cli::RoleCommands::Assign { user_id, roles } => {
                cli::role::assign(&user_id, &roles, &cli.output).await?;
            }
        },
        Commands::Audit { command } => match command {
            cli::AuditCommands::List { action, user_id, recent_hours } => {
                cli::audit::list(
                    action.as_deref(),
                    user_id.as_deref(),
                    recent_hours,
                    &cli.output,
                )
                .await?;
            }
        },
    }
    Ok(())
}
