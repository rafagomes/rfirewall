use super::model;

pub fn get_rules() -> Vec<model::Rule> {
    return model::Rule::load();
}
