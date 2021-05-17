// Copyright 2021 Conveen

/// Network profile.
///
/// All profiles must implement the [up](trait.Profile.html#tymethod.up) and [down](trait.Profile.html#tymethod.down)
/// methods, which determine the commands that get run when applying or removing the profile.
/// The [run_command](trait.Profile.html#method.run_command) method should be used for running
/// arbitary commands in a profile, such as connecting to a VPN or resetting IPv4 settings.
/// The rest of the methods are convenience methods that should across most System76 devices with
/// single wired and wireless NICs.
pub trait Profile {
    /// Run an arbitrary command using the provided shell, or Bash by default, and return stdout
    /// and stderr.
    fn run_command<I, S>(command: I, shell: Option<&str>) -> crate::error::Result<(String, String)>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let shell = shell.unwrap_or("/usr/bin/bash");
        let output = std::process::Command::new(shell)
            .arg("-c")
            .args(command)
            .output()
            .map_err(crate::error::Error::from)?;

        let stdout = std::str::from_utf8(&output.stdout)
            .map_err(crate::error::Error::from)?
            .trim()
            .to_string();
        let stderr = std::str::from_utf8(&output.stderr)
            .map_err(crate::error::Error::from)?
            .trim()
            .to_string();
        Ok((stdout, stderr))
    }

    /// Status of the first `ethernet` device.
    fn gen_ethernet_status() -> crate::error::Result<String> {
        let (ethernet_status, _) = <Self as Profile>::run_command(
            &["nmcli device status | grep ethernet | head -n 1 | awk '{ print $3 }'"],
            None,
        )?;
        Ok(ethernet_status)
    }

    /// An ethernet cable is connected to the computer.
    ///
    /// The `connecting` status means that an ethernet cable is plugged in but NetworkManager waiting for configuration,
    /// either statically or via DHCP.  This function determines whether a cable is connected, and
    /// thus return `true` for `connecting`.
    fn ethernet_is_connected() -> crate::error::Result<bool> {
        let ethernet_status = <Self as Profile>::gen_ethernet_status()?;
        Ok(ethernet_status == "connected".to_string()
            || ethernet_status == "connecting".to_string())
    }

    /// Status of the first `wifi` device.
    fn gen_wifi_status() -> crate::error::Result<String> {
        let (ethernet_status, _) = <Self as Profile>::run_command(
            &["nmcli device status | grep wifi | head -n 1 | awk '{ print $3 }'"],
            None,
        )?;
        Ok(ethernet_status)
    }

    /// The wireless card is enabled (status is not `unavailable).
    ///
    /// Useful if waiting for the wireless card to be enabled before connecting to a specific
    /// network.
    fn wifi_is_available() -> crate::error::Result<bool> {
        let wifi_status = <Self as Profile>::gen_wifi_status()?;
        Ok(wifi_status != "".to_string() && wifi_status != "unavailable".to_string())
    }

    /// The wireless card is connected to a Wi-Fi network.
    ///
    /// Useful if waiting for Wi-Fi to connect before performing another step (i.e., connecting to
    /// a VPN).
    fn wifi_is_connected() -> crate::error::Result<bool> {
        Ok(<Self as Profile>::gen_wifi_status()? == "connected".to_string())
    }

    /// Apply network profile.
    fn up(&self);

    /// Remove network profile.
    fn down(&self);
}

/// Utility function to run a function continuously until it returns `true`.
///
/// Can be used with functions like [wifi_is_connected](trait.Profile.html#method.wifi_is_connected)
/// to wait for a Wi-Fi connection before performing other steps in a profile.
pub fn wait_for<F>(predicate: F, sleep_for: Option<u64>) -> crate::error::Result<()>
where
    F: Fn() -> crate::error::Result<bool>,
{
    let sleep_for = sleep_for.unwrap_or(1);
    while !predicate()? {
        std::thread::sleep(std::time::Duration::from_secs(sleep_for));
    }
    Ok(())
}
