use clap::{Arg, ArgGroup, Command};

use crate::{monitor, rules};

pub fn handle_input() {
    let matches = Command::new("rust-firewall")
        .version("0.1.0")
        .author("rafagomes")
        .subcommand(Command::new("start").about("Start the firewall"))
        .subcommand(Command::new("stop").about("Stop the firewall"))
        .subcommand(
            Command::new("add-rule")
                .arg(
                    Arg::new("allow")
                        .long("allow")
                        .help("Allow traffic")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("deny")
                        .long("deny")
                        .help("Deny traffic")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("port")
                        .long("port")
                        .help("Port number")
                        .value_parser(clap::value_parser!(u16))
                        .require_equals(true)
                        .action(clap::ArgAction::Set),
                )
                .arg(
                    Arg::new("ip")
                        .long("ip")
                        .help("IP address")
                        .action(clap::ArgAction::Set),
                )
                .arg(
                    Arg::new("protocol")
                        .long("protocol")
                        .help("Protocol")
                        .value_parser(["tcp", "udp", "both"])
                        .action(clap::ArgAction::Set),
                )
                .group(
                    ArgGroup::new("action")
                        .args(&["allow", "deny"])
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", _)) => monitor::package_monitor::start_monitor(),
        Some(("stop", _)) => {
            println!("Stopping the firewall");
        }
        Some(("add-rule", add_rule_matches)) => {
            let allow = add_rule_matches.get_flag("allow");
            let deny = add_rule_matches.get_flag("deny");

            let port: u16 = *add_rule_matches.get_one("port").expect("required");
            let ip = add_rule_matches.get_one::<String>("ip").expect("required");
            let protocol = add_rule_matches
                .get_one::<String>("protocol")
                .expect("required");

            rules::add_rules::add_rule(allow, deny, port, ip, protocol)
        }
        _ => println!("No valid option was provided"),
    }
}
