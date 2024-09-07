use super::model;

pub fn add_rule(allow: bool, deny: bool, port: u16, ip: &String, protocol: &String) {
    let action = if allow {
        "allow".to_string()
    } else if deny {
        "deny".to_string()
    } else {
        panic!("Either allow or deny must be true");
    };

    let new_rule = model::Rule {
        action,
        ip: Some(ip.to_string()),
        port: Some(port),
        protocol: Some(protocol.to_string()),
    };

    model::Rule::save(new_rule);
}
