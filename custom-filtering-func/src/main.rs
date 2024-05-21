// Define a trait for the filtering condition
trait FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool;
}

// Implement the filtering function with lifetime annotations
fn custom_filter<'a, T, F>(items: &'a [T], condition: &F) -> Vec<&'a T>
    where
        F: FilterCondition<T>,
{
    let mut result = Vec::new();
    for item in items {
        if condition.is_match(item) {
            result.push(item);
        }
    }
    result
}

// Example usage:

// Define a struct for a specific condition
struct GreaterThanFive;

// Implement the FilterCondition trait for GreaterThanFive
impl FilterCondition<i32> for GreaterThanFive {
    fn is_match(&self, item: &i32) -> bool {
        *item > 5
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 6, 7, 8];

    // Create an instance of the condition
    let condition = GreaterThanFive;

    // Use the custom filter function
    let filtered_numbers = custom_filter(&numbers, &condition);

    // Print the filtered numbers
    println!("Filtered numbers: {:?}", filtered_numbers);
}
