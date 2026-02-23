use alpm::{Alpm, PackageReason, SigLevel};
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    //Name of database to include in the output
    #[arg(short, long)]
    db: Vec<String>,
}


fn main() {

        let handle = Alpm::new("/var/lib/pacman", "/var/lib/pacman").unwrap();
    let mut cli = Cli::parse();

    let db_count = cli.db.len();

    for database in &mut cli.db {
      match database.as_str(){
      "core-testing"=> {

        handle
        .register_syncdb("core-testing", SigLevel::USE_DEFAULT)
        .unwrap();

      }
      "extra-testing" => {

        handle
        .register_syncdb("extra-testing", SigLevel::USE_DEFAULT)
        .unwrap();

      }
      "multilib-testing" => {
        handle
        .register_syncdb("multilib-testing", SigLevel::USE_DEFAULT)
        .unwrap();

      }
      "all" => {
        if db_count == 1 {
            handle
            .register_syncdb("core-testing", SigLevel::USE_DEFAULT)
            .unwrap();
            handle
            .register_syncdb("extra-testing", SigLevel::USE_DEFAULT)
            .unwrap();
            handle
            .register_syncdb("multilib-testing", SigLevel::USE_DEFAULT)
            .unwrap();
        }
      }
      _=> {

        println!("No such database as {}.", database.as_str());
        std::process::exit(1);
      }
     }


    }

    // iterate through each database
    for db in handle.syncdbs() {
        // iterate through every package in the databse
        println!("------ {} ------", db.name());
        for pkg in db.pkgs() {
            // print only explititly intalled packages
            if pkg.reason() == PackageReason::Explicit {
                for localpkg in handle.localdb().pkgs() {
                    if localpkg.name() == pkg.name() {
                        println!("{} {} => {} {}", localpkg.name(), localpkg.version(), pkg.name(), pkg.version());
                    }
                }
            }
        }
        println!("------------------------------ \n");
    }

}
