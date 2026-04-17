pub mod audit;
pub mod login;
pub mod logout;
pub mod role;
pub mod token;
pub mod user;
pub mod whoami;

use clap::{Parser, Subcommand};
use crate::output::OutputFormat;

#[derive(Parser)]
#[command(name = "guardian", version, about = "CLI for Guardian Auth")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, value_enum, global = true, default_value = "pretty")]
    pub output: OutputFormat,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authenticate and save token
    Login {
        /// Email address
        #[arg(short, long)]
        email: String,
        /// Password (will prompt if not provided)
        #[arg(short, long)]
        password: Option<String>,
    },
    /// Logout and invalidate token
    Logout,
    /// Show current authenticated user
    Whoami,
    /// Refresh or show current token
    Token,

    /// Manage users (admin)
    User {
        #[command(subcommand)]
        command: UserCommands,
    },
    /// Manage user roles (admin)
    Role {
        #[command(subcommand)]
        command: RoleCommands,
    },
    /// View audit logs (admin)
    Audit {
        #[command(subcommand)]
        command: AuditCommands,
    },
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// List all users
    List {
        /// Filter by role (user, admin)
        #[arg(long)]
        role: Option<String>,
        /// Search by name or email
        #[arg(long)]
        search: Option<String>,
    },
    /// Create a new user
    Create {
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
        #[arg(long)]
        first: String,
        #[arg(long)]
        last: String,
    },
    /// Delete a user by ID
    Delete {
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum RoleCommands {
    /// Assign roles to a user
    Assign {
        #[arg(long)]
        user_id: String,
        /// Roles to assign (comma-separated: user,admin)
        #[arg(long, value_delimiter = ',')]
        roles: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum AuditCommands {
    /// List audit logs
    List {
        /// Filter by action
        #[arg(long)]
        action: Option<String>,
        /// Filter by user ID
        #[arg(long)]
        user_id: Option<String>,
        /// Filter logs from last N hours
        #[arg(long)]
        recent_hours: Option<i32>,
    },
}
