// Copyright 2021 Conveen

use structopt::StructOpt;

use crate::cli::{ProfileAction, ProfileCliDirective};
use crate::profile::{self, Profile};

/// Wi-Fi radio.
///
/// Toggles the Wi-Fi radio via `nmcli radio wifi {on,off}`.
pub struct WiFiProfile {}

impl WiFiProfile {
    pub fn new() -> Self {
        Self {}
    }
}

impl profile::Profile for WiFiProfile {
    fn up(&self) {
        if !profile::wifi_is_available().unwrap() {
            log::info!("Enabling Wi-Fi");
            profile::run_command(&["nmcli radio wifi on"], None).unwrap();
        } else {
            log::info!("Wi-Fi is already enabled");
        }
    }

    fn down(&self) {
        log::info!("Disabling Wi-Fi");
        profile::run_command(&["nmcli radio wifi off"], None).unwrap();
    }
}

/// Create a new [WiFiProfile](struct.WiFiProfile.html) instance and call `down`.
pub fn down_wifi() {
    let profile = WiFiProfile::new();
    profile.down();
}

/// Create a new [WiFiProfile](struct.WiFiProfile.html) instance and call `up`.
pub fn up_wifi() {
    let profile = WiFiProfile::new();
    profile.up();
}

/// Wi-Fi radio CLI directive.
///
/// The `wifi` profile doesn't accept CL arguments, so
/// [run](../../cli/trait.ProfileCliDirective.html#tymethod.run) simply creates an instance of
/// [WiFiProfile](struct.WiFiProfile.html) and calls `up` or `down`.
#[derive(StructOpt)]
pub struct WiFiProfileDirective {}

impl ProfileCliDirective for WiFiProfileDirective {
    fn run(self, action: ProfileAction) {
        match action {
            ProfileAction::Up => up_wifi(),
            ProfileAction::Down => down_wifi(),
        }
    }
}
