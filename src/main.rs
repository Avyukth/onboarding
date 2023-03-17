use std::env;
use std::env::temp_dir;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

// ... (Package enum definition) ...

enum Platform {
    Darwin,
    Linux,
}

// create a enum for different  packages
enum Package {
    Git,
    Zsh,
    ZshCompletions,
    BashCompletion,
}
impl AsRef<OsStr> for Package {
    fn as_ref(&self) -> &OsStr {
        OsStr::from_bytes(self.name().as_bytes())
    }
}

impl Package {
    fn name(&self) -> &str {
        match self {
            Package::Git => "git",
            Package::Zsh => "zsh",
            Package::ZshCompletions => "zsh-completions",
            Package::BashCompletion => "bash-completion",
        }
    }
}

fn main() {
    let platform = detect_platform();
    install_software(platform);
}

fn detect_platform() -> Platform {
    let os = std::env::consts::OS;
    match os {
        "macos" => Platform::Darwin,
        "linux" => Platform::Linux,
        _ => panic!("Unsupported platform"),
    }
}

// fn install_software(platform: Platform) {
//     match platform {
//         Platform::Darwin => {
//             // Install Homebrew if it's not installed
//             if !Command::new("brew").status().is_ok() {
//                 let _ = Command::new("bash")
//                     .arg("-c")
//                     .arg("\"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")
//                     .status()
//                     .expect("Failed to install Homebrew");
//             }

//             // Install the required software
//                 let packages = vec![
//         Package::Git,
//         Package::Zsh,
//         Package::ZshCompletions,
//         Package::BashCompletion,
//     ];
//             let _ = Command::new("brew")
//                 .args(&["install"])
//                 .args(&packages)
//                 .status()
//                 .expect("Failed to install required software");
//         }
//         Platform::Linux => {
//             // Install required packages
//             let _ = Command::new("sudo")
//                 .args(&["apt", "update"])
//                 .status()
//                 .expect("Failed to update package list");

//                 let packages = vec![
//         Package::Git,
//         Package::Zsh,
//         Package::ZshCompletions,
//         Package::BashCompletion,
//     ];
//             let _ = Command::new("sudo")
//                 .args(&["apt", "install", "-y"])
//                 .args(&packages)
//                 .status()
//                 .expect("Failed to install required software");
//         }
//         _ => {
//             eprintln!("Unsupported platform");
//             std::process::exit(1);
//         }
//     }

//     println!("All required software has been installed successfully.");
// }

fn install_software(platform: Platform) {
    match platform {
        Platform::Darwin => {
            // Install Homebrew if it's not installed
            if !Command::new("brew").status().is_ok() {
                let _ = Command::new("bash")
                    .arg("-c")
                    .arg("\"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")
                    .status()
                    .expect("Failed to install Homebrew");
            }

            // Install Zsh
            let _ = Command::new("brew")
                .args(&["install", "zsh"])
                .status()
                .expect("Failed to install Zsh");

            // Install Oh My Zsh
            let _ = Command::new("bash")
                .arg("-c")
                .arg("-y")
                .arg("\"$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)\"")
                .status()
                .expect("Failed to install Oh My Zsh");
        }
        Platform::Linux => {
            // Install required packages
            let _ = Command::new("sudo")
                .args(&["apt", "update"])
                .status()
                .expect("Failed to update package list");

            let _ = Command::new("sudo")
                .args(&["apt", "install", "-y", "zsh"])
                .status()
                .expect("Failed to install Zsh");

            // Install Oh My Zsh
            install_oh_my_zsh();

            install_omz_plugins();
        }
        _ => {
            eprintln!("Unsupported platform");
            std::process::exit(1);
        }
    }

    println!("Homebrew, Zsh, and Oh My Zsh have been installed successfully.");
}

fn install_oh_my_zsh() {
    // Download Oh My Zsh install script
    let output = Command::new("curl")
        .arg("-fsSL")
        .arg("https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh")
        .output()
        .expect("Failed to download Oh My Zsh install script");

    if !output.status.success() {
        eprintln!("Failed to download Oh My Zsh install script");
        println!("-------------------------------------------------------------------------------------------------");
        std::process::exit(1);
    }

    // Save script to a temporary file
    let mut temp_path = temp_dir();
    temp_path.push("ohmyzsh_install.sh");

    let mut temp_file = File::create(&temp_path).expect("Failed to create temporary file");
    temp_file
        .write_all(&output.stdout)
        .expect("Failed to write to temporary file");

    // Run the script with bash
    let status = Command::new("sh")
        .arg(temp_path)
        .status()
        .expect("Failed to run Oh My Zsh install script");

    if status.success() {
        println!("Oh My Zsh has been installed successfully.");
        println!("-------------------------------------------------------------------------------------------------");
    } else {
        eprintln!("Failed to install Oh My Zsh.");
        println!("-------------------------------------------------------------------------------------------------");
    }
}

fn install_omz_plugins() {
    // Install plugins
    let zsh_custom = env::var("ZSH_CUSTOM")
        .unwrap_or_else(|_| format!("{}/.oh-my-zsh/custom", env::var("HOME").unwrap()));

    let plugins = [
        ("zsh-z", "https://github.com/agkozak/zsh-z"),
        (
            "zsh-autosuggestions",
            "https://github.com/zsh-users/zsh-autosuggestions",
        ),
        (
            "zsh-completions",
            "https://github.com/zsh-users/zsh-completions",
        ),
        (
            "zsh-syntax-highlighting",
            "https://github.com/zsh-users/zsh-syntax-highlighting",
        ),
        ("powerlevel10k", "https://github.com/romkatv/powerlevel10k"),
    ];

    for (plugin_name, repo_url) in plugins.iter() {
        // Check if the plugin directory exists
        let plugin_dir = format!("{}/plugins/{}", zsh_custom, plugin_name);

        if Path::new(&plugin_dir).exists() {
            println!("{} plugin already exists at: {}", plugin_name, plugin_dir);
            println!("-------------------------------------------------------------------------------------------------");
        } else {
            // Clone the plugin repository
            println!("Installing {} plugin...", plugin_name);
            let status = Command::new("git")
                .args(&["clone", repo_url, &plugin_dir])
                .status()
                .expect("Failed to execute git command");

            if status.success() {
                println!("{} plugin has been installed successfully.", plugin_name);
                println!("-------------------------------------------------------------------------------------------------");
            } else {
                eprintln!("Failed to install {} plugin.", plugin_name);
                println!("-------------------------------------------------------------------------------------------------");
            }
        }
    }
}
