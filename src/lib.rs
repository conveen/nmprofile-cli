// Copyright 2021 Conveen

//! # Overview
//!
//! Command line framework for creating and applying executable network profiles via the
//! NetworkManager CLI on systems running pop_os!.
//!
//! [pop_os!](https://pop.system76.com/) is an (awesome) operating system created and maintained by System76 (which is also awesome).
//! [NetworkManager](https://ubuntu.com/core/docs/networkmanager) is a common network management daemon for Linux systems that can
//! manage Wi-Fi, ethernet, VPN, and other connection types. The NetworkManager daemon has an
//! accompanying CLI, `nmcli`, for controlling devices and connections from the command line.
//! However, NetworkManager and the CLI only support certain VPN types and don't allow specifying
//! running arbitrary commands for setting up connections.  This command line framework offers a
//! slightly higher level of abstraction on top of `nmcli` and allows you to create executable
//! network profiles.
//!
//! # Use Cases
//!
//! Why would you want to do this? One example is if you use a VPN provider like NordVPN that has a separate
//! command line application for connecting to the service. In that case, you may want to create a
//! profile that does the following things:
//!
//! 1. Check to see if Wi-Fi is enabled and, if not, enable it.
//! 2. Connect to a random NordVPN server in the US.
//! 3. Make sure /etc/resolv.conf is configured to use the NordVPN DNS servers.
//!
//! Alternatively, you may have multiple VLANs that you connect to via ethernet with different static
//! configurations. Whatever the situation, simple or complex, you can write a profile to handle
//! it.
//!
//! # Creating a profile
//!
//! There are 5 steps required to create a new profile and expose it on the command line:
//!
//! 1. Create a new [profiles submodule](profiles/index.html) with a unique name for the
//!    profile (i.e., `profiles/nord_vpn.rs`).
//! 2. Within that submodule, create a struct that implements the [Profile
//!    trait](profile/trait.Profile.html)'s `up` and `down` methods.  These methods are used to
//!    apply and remove the network profile via the CLI.
//! 3. Also create a struct that derives `StructOpt` and that will serve as the CLI for the
//!    profile. For example, the profile might take a command line argument for which VPN server to
//!    connect to. Then, implement the [ProfileCliDirective trait](cli/trait.ProfileCliDirective.html)'s `run` method.
//!    This struct must be `pub`.
//! 4. Add the new submodule to the [profiles module](profiles/index.html) and (optionally) `pub use`
//!    the CLI directive struct.
//! 5. Integrate the new profile into the main CLI by augmenting the [up](cli/enum.UpDirective.html) and [down](cli/enum.DownDirective.html) CLI directive enums.
//!
//! # Example: NordVPN Profile
//!
//! ## Create new `profiles::nord_vpn` submodule.
//!
//! ```
//! // src/profiles/nord_vpn.rs
//! use structopt::StructOpt;
//!
//! use libnmprofile::cli::{ProfileAction, ProfileCliDirective};
//! use libnmprofile::profile::{self, Profile};
//! use libnmprofile::profiles::wifi::{down_wifi, up_wifi};
//!
//! /// NordVPN is connected.
//! pub fn nordvpn_is_connected() -> libnmprofile::error::Result<bool> {
//!     let (nordvpn_status, _) = profile::run_command(
//!         &["nordvpn status | grep Status | awk -F':' '{ print $2 }' | tr -d ' '"],
//!         None,
//!     )?;
//!     Ok(nordvpn_status.to_lowercase() == "connected".to_string())
//! }
//!
//! pub struct NordVpnProfile {
//!     /// VPN server location to connect to.
//!     location: String,
//! }
//!
//! impl NordVpnProfile {
//!     pub fn new(location: String) -> Self {
//!         Self { location, }
//!     }
//! }
//!
//! impl Profile for NordVpnProfile {
//!     fn up(&self) {
//!         // Enable Wi-Fi if it's not already enabled
//!         if !profile::ethernet_is_connected().unwrap() {
//!             up_wifi();
//!         }
//!
//!         if !nordvpn_is_connected().unwrap() {
//!             // Connect to NordVPN via CLI, passing in location
//!             let connect_command = format!("nordvpn connect {}", &self.location);
//!             log::info!("Connecting to NordVPN");
//!             profile::run_command(&[&connect_command], None).unwrap();
//!             log::info!("Connected to NordVPN");
//!         } else {
//!             log::info!("Already connected to NordVPN");
//!         }
//!     }
//!
//!     fn down(&self) {
//!         if nordvpn_is_connected().unwrap() {
//!             log::info!("Disconnecting from NordVPN");
//!             profile::run_command(&["nordvpn disconnect"], None).unwrap();
//!             log::info!("Disconnected from NordVPN");
//!         }
//!     }
//! }
//!
//! #[derive(StructOpt)]
//! pub struct NordVpnProfileDirective {
//!     /// Location of VPN servers to connect to
//!     #[structopt(short, long, default_value = "")]
//!     location: String,
//! }
//!
//! impl ProfileCliDirective for NordVpnProfileDirective {
//!     fn run (self, action: ProfileAction) {
//!         let profile = NordVpnProfile::new(self.location);
//!         match action {
//!             ProfileAction::Up => profile.up(),
//!             ProfileAction::Down => profile.down(),
//!         }
//!     }
//! }
//! ```
//!
//! ## Add submodule to `profiles` module
//!
//! ```ignore
//! // src/profiles.rs
//!
//! // snip...
//!
//! pub mod nord_vpn;
//!
//! // snip...
//!
//! pub use nord_vpn::NordVpnProfileDirective;
//! ```
//!
//! ## Integrate profile into the main CLI
//!
//! ```ignore
//! // src/cli.rs
//! use structopt::StructOpt
//!
//! // snip...
//!
//! #[derive(StructOpt)]
//! pub enum UpDirective {
//!     // over variants...
//!     #[structopt(alias = "n", name = "nordvpn")]
//!     NordVpn(crate::profiles::NordVpnProfileDirective),
//! }
//!
//! impl UpDirective {
//!     pub fn run(self) {
//!         match self {
//!             // other variants...
//!             Self::NordVpn(d) => d.run(ProfileAction::Up),
//!         }
//!     }
//! }
//!
//! // same for DownDirective
//! ```
//!
//! After completing these steps and recompiling the program, the `nordvpn` profile will be
//! available with `nmprofile up nordvpn -l <location>` and `nmprofile down nordvpn`.

pub mod cli;
pub mod error;
#[allow(dead_code)]
pub mod profile;
pub mod profiles;
