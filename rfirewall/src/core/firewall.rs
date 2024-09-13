use crate::infrastructure::monitor::Monitor;

use super::rule::Rule;

pub struct Firewall {
    monitor: Monitor,
    rules: Vec<Rule>,
}

impl Firewall {
    pub fn new() -> Self {
        Firewall {
            monitor: Monitor::default(),
            rules: Vec::new(),
        }
    }

    pub fn start(&self) {
        println!("Starting firewall");
        self.monitor.start();
    }

    pub fn stop(&self) {
        println!("Stopping firewall");
        self.monitor.stop();
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }
}
