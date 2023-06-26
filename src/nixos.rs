use std::process::Command;

pub fn vixnixos(argument: &String, installion_argument: String) {
           match argument.as_str() {
             "install" => {
                let install_command = "nixos.".to_string() + &installion_argument;

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

            "update" => {
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
                 let clear_output = String::from_utf8_lossy(&output.stdout);
                 println!("{}", clear_output);
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

            "clear-generations" => {
                let output = Command::new("nix-collect-garbage")
                .arg("-d")
                .output()
                .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully executed command");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                 }
            },

            "remove" => {
                let output = Command::new("nix-env")
                    .arg("--uninstall")
                    .arg(&installion_argument)
                    .output()
                    .expect("Failed to run command");

                if output.status.success() {
                    println!("Succesfully deleted package: {}", installion_argument.clone());
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
            },

             "generations" => {
                let output = Command::new("nix-env")
                    .arg("--list-generations")
                    .output()
                    .expect("Failed to run command");

                if output.status.success() {
                    let clear_output = String::from_utf8_lossy(&output.stdout);
                    println!("{}", clear_output);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
               }
             },

             "rebuild" => {
                println!("Please be sure you runned Vix as root or with sudo.");
                let output = Command::new("nixos-rebuild")
                    .arg("switch")
                    .output()
                    .expect("Failed to run command");

                if output.status.success() {
                    let clear_output = String::from_utf8_lossy(&output.stdout);
                    println!("{}", clear_output);
                    println!("Builded system");
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error {}", stderr);
                }
             },

             "rebuild-boot" => {
                 println!("PLease be sure you runned Vix as root or with sudo");
                let output = Command::new("nixos-rebuild")
                     .arg("boot")
                     .output()
                     .expect("Failed to run command");
                
                if output.status.success() {
                    let clear_output = String::from_utf8_lossy(&output.stdout);
                    println!("{}", clear_output);
                    println!("Nix Builded Succesfully. please reboot your system");

                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to rebuild Nix. Error: {}", stderr);
                }
             },

             "switch" =>  {
                 let output = Command::new("nix-env")
                     .arg("--switch-generation")
                     .arg(installion_argument.clone())
                     .output()
                     .expect("Failed to run command");

                if output.status.success() {
                    println!("Switched Generation to: {}", installion_argument);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to execute command. error {}", stderr);
                }
             },
             "bootstrap" => {
                 println!("Warning. Vix Assumes you have Cargo and git installed");
                 let vixclone = Command::new("git")
                     .arg("clone")
                     .arg("https://github.com/AlexanderMaxRanabel/Vix.git")
                     .output()
                     .expect("Failed to bootstrap Vix");

                if vixclone.status.success() {
                    println!("Cloned Vix");
                    let vixcd = Command::new("cd")
                        .arg("Vix")
                        .output()
                        .expect("Failed to bootstrap Vix");

                    if vixcd.status.success() {
                        println!("Changed directory to Vix");
                        let vixbuild = Command::new("cargo")
                            .arg("build")
                            .arg("--release")
                            .output()
                            .expect("Failed to bootstrap Vix");

                        if vixbuild.status.success() {
                            println!("Vix has been builded");
                        } else {
                            let stderr = String::from_utf8_lossy(&vixclone.stderr);
                            eprintln!("Failed to execute command. error: {}", stderr);
                        }
                    } else {
                        let stderr = String::from_utf8_lossy(&vixclone.stderr);
                        eprintln!("Failed to execute command. error: {}", stderr);
   
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&vixclone.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);
                }
             },

             "flake-init" => {
                let flake = Command::new("nix")
                    .arg("flake")
                    .arg("init")
                    .output()
                    .expect("Failed to Init the flake");

                if flake.status.success(){
                    println!("Initilased The Flake");
                    let add = Command::new("git")
                        .arg("add")
                        .arg("flake.nix")
                        .output()
                        .expect("");

                    if add.status.success() {
                        println!("Added flake.nix to git");
                    } else {
                        let stderr = String::from_utf8_lossy(&add.stderr);
                        eprintln!("Failed to execute command. error: {}", stderr);
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&flake.stderr);
                    eprintln!("Failed to execute command. error: {}", stderr);

                }
             },


             "help" => {
                println!("install <package name>: installs the specified package");
                println!("delete <package name>: deletes specified package");
                println!("clear: Garbage collects Nix");
                println!("list: lists installed packages");
                println!("generations: lists generations");

             },

        "version" => {
            println!("0.0.5");
        },
        _ => {
            println!("Invalid Argument");
        }
    }
}
