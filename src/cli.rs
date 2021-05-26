// Copyright 2021 Conveen

//! CLI entrypoint and glue code.

use structopt::StructOpt;

/// Apply or remove profile.
///
/// Used with [ProfileCliDirective::run](trait.ProfileCliDirective.html#tymethod.run)
/// to show which action was requested on the CLI, `up` or `down`.
pub enum ProfileAction {
    Down,
    Up,
}

/// Entrypoint for profile CLI directives.
///
/// Structs that define the CLI for a given profile are separate from the structs that implement the actual
/// profile (see: `WiFiProfileDirective` and `WiFiProfile` in the
/// [wifi](../profiles/wifi/index.html) module). All CLI directives must implement this trait
/// before integration into the main CLI.
///
/// # Examples
///
/// For profiles that don't accept CL arguments (see: [wifi](../profiles/wifi/index.html))
/// run can simply be implemented as:
///
/// ```
/// // some_profile.rs
/// use structopt::StructOpt;
///
/// use libnmprofile::cli::{ProfileAction, ProfileCliDirective};
/// use libnmprofile::profile::{self, Profile};
///
/// struct SomeProfile {}
///
/// impl Profile for SomeProfile {
///     fn up(&self) {
///         // some code...
///     }
///
///     fn down(&self) {
///         // some code...
///     }
/// }
///
/// #[derive(StructOpt)]
/// pub struct SomeProfileDirective {}
///
/// impl ProfileCliDirective for SomeProfileDirective {
///     fn run(self, action: ProfileAction) {
///         let profile = SomeProfile {};
///         match action {
///             ProfileAction::Up => profile.up(),
///             ProfileAction::Down => profile.down(),
///         }
///     }
/// }
/// ```
///
/// If a profile does accept CL arguments, they can be passed to the profile in the call to `run`:
///
/// ```ignore
/// // some_profile.rs
/// use structopt::StructOpt;
///
/// use libnmprofile::cli::{ProfileAction, ProfileCliDirective};
/// // snip...
///
/// struct SomeProfile {
///     setting: String,
/// }
///
/// // snip...
///
///
/// #[derive(StructOpt)]
/// struct SomeProfileDirective {
///     #[structopt(short, long)]
///     setting: String,
/// }
///
/// impl ProfileCliDirective for SomeProfileDirective {
///     fn run(self, action: ProfileAction) {
///         let profile = SomeProfile {setting: self.setting};
///         match action {
///             ProfileAction::Up => profile.up(),
///             ProfileAction::Down => profile.down(),
///         }
///     }
/// }
/// ```
pub trait ProfileCliDirective {
    /// Either apply or remove the profile.
    fn run(self, action: ProfileAction);
}

/// `up` directive - apply a network profile.
///
/// All network profiles must be added as a variant to this enum,
/// and [run](enum.UpDirective.html#method.run) must be updated,
/// for the profile to be integrated into the CLI. See the [WiFi variant](enum.UpDirective.html#variant.WiFi)
/// as an example.
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

/// `down` directive - remove a network profile.
///
/// All network profiles must be added as a variant to this enum,
/// and [run](enum.DownDirective.html#method.run) must be updated,
/// for the profile to be integrated into the CLI. See the [WiFi variant](enum.DownDirective.html#variant.WiFi)
/// as an example.
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

/// Create and apply executable network profiles via the NetworkManager CLI.
#[doc(hidden)]
#[derive(StructOpt)]
#[structopt(
    name = "network",
    author = "Conveen",
    version = env!("CARGO_PKG_VERSION"),
    setting=structopt::clap::AppSettings::ArgRequiredElseHelp,
)]
pub enum Cli {
    #[structopt(
        alias = "u",
        name = "up",
        about = "Apply a network profile",
        long_about = "",
    )]
    Up(UpDirective),
    #[structopt(
        alias = "d",
        name = "down",
        about = "Remove a network profile",
        long_about = "",
    )]
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
