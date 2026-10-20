// Each declaration loads a child module from the matching file in this directory.
mod problem0;
mod problem1;
mod problem2;

// `use` brings these types into this module's scope for the factory below.
use self::problem0::Problem0;
use self::problem1::Problem1;
use self::problem2::Problem2;

const SEPARATOR: &str = "******************************************************************************************************************";

pub trait Problem {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn solution(&self) -> String;

    fn execute(&self) {
        println!("{SEPARATOR}");
        println!();
        println!("{}", self.name());
        println!("{}", self.description());
        println!("{}", self.solution());
        println!();
    }
}

pub fn create_problem_adapter(number: u32) -> Box<dyn Problem> {
    match number {
        1 => Box::new(Problem1),
        2 => Box::new(Problem2),
        _ => Box::new(Problem0),
    }
}

#[cfg(test)]
mod tests {
    use super::create_problem_adapter;

    #[test]
    fn solves_multiples_of_three_or_five() {
        let problem = create_problem_adapter(1);
        assert_eq!(problem.name(), "Problem 1");
        assert_eq!(problem.solution(), "233168");
    }

    #[test]
    fn solves_even_fibonacci_numbers() {
        let problem = create_problem_adapter(2);
        assert_eq!(problem.name(), "Problem 2");
        assert_eq!(problem.solution(), "4613732");
    }

    #[test]
    fn unknown_numbers_use_the_fallback() {
        for number in [0, 3, u32::MAX] {
            let problem = create_problem_adapter(number);
            assert_eq!(problem.name(), "Problem 0");
            assert_eq!(problem.description(), "No problem.");
            assert_eq!(problem.solution(), "No answer.");
        }
    }
}
