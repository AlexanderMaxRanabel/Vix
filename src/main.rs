use std::env;
use std::process::Command;

fn is_nixos() -> bool {
    let output = Command::new("nixos-version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok());

    match output {
        Some(stdout) => stdout.trim() == "NixOS",
        None => false,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let argument = &args[1].to_string();
        let installion_argument = &args[2].to_string();

        if is_nixos() {

        } else {
        match argument.as_str() {
            "install" => {
                let install_command = "nixpkgs.".to_string() + installion_argument;

                let output = Command::new("nix-env")
                .arg("-iA")
                .arg(install_command)
                .output()
                .expect("Failed to run command");

            if output.status.success() {
                println!("İnstalled wanted packages");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                 eprintln!("Failed to run command. Error: {}", stderr);
                }
            },

            "uc" => {
                let output = Command::new("nix-channel")
                 .arg("--update")
                 .output()
                 .expect("Failed to run command");

                if output.status.success() {
                    println!("Updated Channels");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to run command. Error: {}", stderr);
                }
            },

            "list" => {
               let output = Command::new("nix-env")
               .arg("-q")
               .output()
               .expect("Failed to run command");

             if output.status.success() {
                println!("Succesfully runned command");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to run command. error: {}", stderr);
                }
            },

            "clear" => {
                let output = Command::new("nix-collect-garbage")
                 .output()
                 .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
            },
            _ => {
                println!("Invalid Argument");
            }
        }
      }
    } else {
        println!("No argument provided.");
    }
}