// Copyright 2021 Conveen

use structopt::StructOpt;

use crate::cli::{ProfileAction, ProfileCliDirective};
use crate::profile::Profile;

pub struct WiFiProfile {}

impl WiFiProfile {
    pub fn new() -> Self {
        Self {}
    }
}

impl Profile for WiFiProfile {
    fn up(&self) {
        if !<Self as Profile>::wifi_is_available().unwrap() {
            log::info!("Enabling Wi-Fi");
            <Self as Profile>::run_command(&["nmcli radio wifi on"], None).unwrap();
        } else {
            log::info!("Wi-Fi is already enabled");
        }
    }

    fn down(&self) {
        log::info!("Disabling Wi-Fi");
        <Self as Profile>::run_command(&["nmcli radio wifi off"], None).unwrap();
    }
}

/// Wi-Fi radio.
#[derive(StructOpt)]
pub struct WiFiProfileDirective {}

impl ProfileCliDirective for WiFiProfileDirective {
    fn run(self, action: ProfileAction) {
        let profile = WiFiProfile::new();
        match action {
            ProfileAction::Up => profile.up(),
            ProfileAction::Down => profile.down(),
        }
    }
}
