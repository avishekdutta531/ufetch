use std::io;
use std::path::Path;
use std::process::Command;

pub fn get_pkg() -> io::Result<String> {
    let mut result = String::new();

    if Path::new("/bin/dpkg").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("dpkg --get-selections | wc -l")
            .output()?;

        if String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<i32>()
            .unwrap()
            != 0
        {
            result += &format!("{} (dpkg) ", String::from_utf8_lossy(&output.stdout).trim());
        }
    }

    if Path::new("/bin/flatpak").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("flatpak list | wc -l")
            .output()?;

        if String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<i32>()
            .unwrap()
            != 0
        {
            result += &format!(
                "{} (flatpak) ",
                String::from_utf8_lossy(&output.stdout).trim()
            );
        }
    }

    if Path::new("/bin/pacman").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("pacman -Qq | wc -l")
            .output()?;

        if String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<i32>()
            .unwrap()
            != 0
        {
            result += &format!(
                "{} (pacman) ",
                String::from_utf8_lossy(&output.stdout).trim()
            );
        }
    }

    if Path::new("/var/lib/rpm").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("rpm -qa | wc -l")
            .output()?;

        if String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<i32>()
            .unwrap()
            != 0
        {
            result += &format!("{} (rpm) ", String::from_utf8_lossy(&output.stdout).trim());
        }
    }

    if Path::new("/bin/snap").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("snap list | wc -l")
            .output()?;

        if String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<i32>()
            .unwrap()
            != 0
        {
            result += &format!("{} (snap) ", String::from_utf8_lossy(&output.stdout).trim());
        }
    }

    if Path::new("/bin/xbps-install").exists() {
        let output = Command::new("sh")
            .arg("-c")
            .arg("xbps-query -l | wc -l")
            .output()?;

        if String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<i32>()
            .unwrap()
            != 0
        {
            result += &format!("{} (xbps) ", String::from_utf8_lossy(&output.stdout).trim());
        }
    }

    Ok(result)
}
