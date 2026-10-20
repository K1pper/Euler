// `super` refers to the parent `problems` module.
use super::Problem;

pub(super) struct Problem1;

impl Problem for Problem1 {
    fn name(&self) -> &str {
        "Problem 1"
    }

    fn description(&self) -> &str {
        r"
  If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of   
  these multiples is 23. Find the sum of all the multiples of 3 or 5 below 1000.                               
"
    }

    fn solution(&self) -> String {
        (0..1000)
            .filter(|n| n % 3 == 0 || n % 5 == 0)
            .sum::<u32>()
            .to_string()
    }
}
