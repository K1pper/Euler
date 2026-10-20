// `super` refers to the parent `problems` module.
use super::Problem;

pub(super) struct Problem0;

impl Problem for Problem0 {
    fn name(&self) -> &str {
        "Problem 0"
    }

    fn description(&self) -> &str {
        "No problem."
    }

    fn solution(&self) -> String {
        "No answer.".to_owned()
    }
}
