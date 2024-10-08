use std::sync::{atomic::AtomicBool, Arc};

use clap::{Arg, ArgGroup, Command};

use crate::core::rule::Rule;
use crate::infrastructure::monitor::Monitor;

pub fn handle_input(running: Arc<AtomicBool>) {
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

    let monitor = Monitor::new(running);

    match matches.subcommand() {
        Some(("start", _)) => monitor.start(),
        Some(("stop", _)) => monitor.stop(),
        Some(("add-rule", add_rule_matches)) => {
            let mut allow = add_rule_matches.get_flag("allow");
            let deny = add_rule_matches.get_flag("deny");

            if allow && deny {
                println!("Cannot allow and deny at the same time");
                return;
            }

            if deny {
                allow = false;
            }

            let port: u16 = *add_rule_matches.get_one("port").expect("required");
            let ip = add_rule_matches.get_one::<String>("ip").expect("required");
            let protocol = add_rule_matches
                .get_one::<String>("protocol")
                .expect("required");

            Rule::new(allow, port, ip, protocol);
        }
        _ => println!("No valid option was provided"),
    }
}
