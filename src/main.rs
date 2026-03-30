use std::process::Command;

// TODO  add autoremove command : LINE 70
// TODO add a help command that lists commands and names their Functions

fn run(cmd: &str, args: &[&str]) {
    Command::new(cmd).args(args).status().unwrap_or_else(|_| {
        eprintln!("Failed to execute {}", cmd);
        std::process::exit(1);
    });
}

fn require_pkg(pkg: Option<&String>) -> &str {
    pkg.map(|s| s.as_str()).unwrap_or_else(|| {
        eprintln!("Missing package name");
        std::process::exit(1);
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("No command provided. Use: fusi <command>");
        std::process::exit(1);
    }

    let cmd = args[0].as_str();
    let pkg = args.get(1);

    match cmd {
        // fusi install <package>
        "install" => run("sudo", &["pacman", "-S", require_pkg(pkg)]),

        // fusi remove <package>
        "remove" => run("sudo", &["pacman", "-Rns", require_pkg(pkg)]),

        // searches for a package
        "search" => run("pacman", &["-Ss", require_pkg(pkg)]),

        // Updates the system
        "update" => run("sudo", &["pacman", "-Syu"]),

        // does a light clean up unlike the remove command
        "softremove" => run("sudo", &["pacman", "-R", require_pkg(pkg)]),

        // Gives you info abt a specific package
        "info" => run("pacman", &["-Si", require_pkg(pkg)]),

        // lists installed packages (not including deps)
        "list" => run("pacman", &["-Qe"]),

        // lists all installed packages including deps
        "listall" => run("pacman", &["-Q"]),

        // Checks if a specific pkg is installed
        "installed" => run("pacman", &["-Qs", require_pkg(pkg)]),

        // fusi details
        "details" => {
            println!("Wip");
        }

        // upgrades a pkg
        "upgrade" => run("sudo", &["pacman", "-S", require_pkg(pkg)]),

        //downgrades a pkg
        "downgrade" => run("sudo", &["pacman", "-U", require_pkg(pkg)]),

        // removes uused deps
        // "autoremove" => run("sudo", &["pacman", "-Qdtg"]),

        // shows files owned by pkg
        "files" => run("pacman", &["-Ql", require_pkg(pkg)]),

        // shows which pagake owns a file
        "owner" => run("pacman", &["-Qo", require_pkg(pkg)]),

        // shows deps of pkg
        "deps" => run("pacman", &["-Si", require_pkg(pkg)]),

        // exact same as deps but fancier name
        "dependencies" => run("pacman", &["-Si", require_pkg(pkg)]),
        // shows install history
        "log" => run("cat", &["/var/log/pacman.log"]),

        // find,rank,update mirrorlist
        "mirrors" => run("sudo", &["reflector"]),

        // removes pacan lock file
        "unlock" => run("sudo", &["rm", "/var/lib/pacman/db.lck"]),

        // shows amount of pkgs installed
        "stats" => run("pacman", &["-Qq"]),

        // anything else
        _ => {
            println!(
                "unknown command ( {} ) type fusi help to list all Commands",
                cmd
            );
        }
    }

    std::process::exit(0);
}
