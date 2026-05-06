use alpm::{Alpm, PackageReason, SigLevel};
use clap::Parser;
use pacmanconf::Config;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    //Name of database to include in the output
    #[arg(short, long)]
    db: Vec<String>,
}

fn is_included(db: &str) -> bool {
    let config = Config::new().expect("Couldn't read pacman.conf.");
    config.repos.iter().any(|r| r.name == db)
}

fn register_db(handle: &Alpm) {
    let mut cli = Cli::parse();

    let db_count = cli.db.len();

    for database in &mut cli.db {
        match database.as_str() {
            "core-testing" => {
                if !is_included("core-testing") {
                    println!("core-testing is not included in config file");
                } else {
                    handle
                        .register_syncdb("core-testing", SigLevel::USE_DEFAULT)
                        .unwrap();
                }
            }
            "extra-testing" => {
                if !is_included("extra-testing") {
                    println!("extra-testing is not included in config file");
                } else {
                    handle
                        .register_syncdb("extra-testing", SigLevel::USE_DEFAULT)
                        .unwrap();
                }
            }
            "multilib-testing" => {
                if !is_included("multilib-testing") {
                    println!("multilib-testing is not included in config file");
                } else {
                    handle
                        .register_syncdb("multilib-testing", SigLevel::USE_DEFAULT)
                        .unwrap();
                }
            }
            "all" => {
                if db_count == 1 {
                    if !is_included("core-testing") {
                        println!("core-testing is not included in config file");
                    } else {
                        handle
                            .register_syncdb("core-testing", SigLevel::USE_DEFAULT)
                            .unwrap();
                    }
                    if !is_included("extra-testing") {
                        println!("extra-testing is not included in config file");
                    } else {
                        handle
                            .register_syncdb("extra-testing", SigLevel::USE_DEFAULT)
                            .unwrap();
                    }
                    if !is_included("multilib-testing") {
                        println!("multilib-testing is not included in config file");
                    } else {
                        handle
                            .register_syncdb("multilib-testing", SigLevel::USE_DEFAULT)
                            .unwrap();
                    }
                }
            }
            _ => {
                println!("No such database as {}.", database.as_str());
                std::process::exit(1);
            }
        }
    }
}

fn packages_output(handle: &Alpm) {
    // iterate through each database
    for db in handle.syncdbs() {
        // iterate through every package in the databse
        println!("------ {} ------", db.name());
        for pkg in db.pkgs() {
            // print only explititly intalled packages
            if pkg.reason() == PackageReason::Explicit {
                for localpkg in handle.localdb().pkgs() {
                    if localpkg.name() == pkg.name() {
                        if localpkg.version() != pkg.version() {
                            println!(
                                "{} {} => {} {}",
                                localpkg.name(),
                                localpkg.version(),
                                pkg.name(),
                                pkg.version()
                            );
                        } else {
                            println!(
                                "{} {} already up to date on local system.",
                                pkg.name(),
                                pkg.version()
                            );
                        }
                    }
                }
            }
        }
        println!("------------------------------ \n");
    }
}

fn main() {
    let handle = Alpm::new("/var/lib/pacman", "/var/lib/pacman").unwrap();
    register_db(&handle);
    packages_output(&handle);
}
