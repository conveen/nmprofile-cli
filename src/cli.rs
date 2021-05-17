// Copyright 2021 Conveen

use structopt::StructOpt;

/// Apply or remove profile.
pub enum ProfileAction {
    Down,
    Up,
}

/// Entrypoint for profile CLI directives (subcommands).
pub trait ProfileCliDirective {
    fn run(self, action: ProfileAction);
}

/// Apply a network profile.
#[derive(StructOpt)]
pub enum UpDirective {
    #[structopt(alias = "w", name = "wifi")]
    WiFi(crate::profiles::WiFiProfileDirective),
}

impl UpDirective {
    pub fn run(self) {
        match self {
            Self::WiFi(d) => d.run(ProfileAction::Up),
        }
    }
}

/// Remove a network profile.
#[derive(StructOpt)]
pub enum DownDirective {
    #[structopt(alias = "w", name = "wifi")]
    WiFi(crate::profiles::WiFiProfileDirective),
}

impl DownDirective {
    pub fn run(self) {
        match self {
            Self::WiFi(d) => d.run(ProfileAction::Down),
        }
    }
}

/// Create and apply executable network profiles through NetworkManager CLI.
#[derive(StructOpt)]
#[structopt(
    name = "network",
    author = "Conveen",
    version = env!("CARGO_PKG_VERSION"),
    setting=structopt::clap::AppSettings::ArgRequiredElseHelp,
)]
pub enum Cli {
    #[structopt(alias="u", name = "up")]
    Up(UpDirective),
    #[structopt(alias="d", name = "down")]
    Down(DownDirective),
}

impl Cli {
    pub fn run(self) {
        match self {
            Self::Up(d) => d.run(),
            Self::Down(d) => d.run(),
        }
    }
}
