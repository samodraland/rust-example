mod calculate_vector;
mod operations;

fn main() {
    let numbers = vec![10, 2, 4, 5, 3];
    println!("\n----------------------------------");
    println!("Numbers: {:?}", numbers);
    println!("----------------------------------");
    /*
     * since numbers variable is a vector it doesn't apply Copy
     * and it's used multiple times, the ownership of variable is moving
     * therefore have to from borrow origin numbers variable by writing &numbers
     */
    use_with_option(&numbers);
    println!("----------------------------------");
    use_no_option(&numbers);
}

fn operators_declaration() -> [[&'static str; 2]; 4] {
    [
        ["+", "Summation"],
        ["-", "Subtraction"],
        ["/", "Division"],
        ["*", "Multiplication"],
    ] // no semicolon means it will return the value
}

fn use_with_option(numbers: &Vec<i32>) {
    let operators = operators_declaration();
    for operator in operators {
        let result = calculate_vector::calc_with_option(operator[0], &numbers);
        match result {
            Some(val) => println!("{} result is {}", operator[1], val),
            None => println!("Error"),
        };
    }
}

fn use_no_option(numbers: &Vec<i32>) {
    let operators = operators_declaration();
    for operator in operators {
        let result = calculate_vector::calc_no_option(operator[0], &numbers);
        println!("{} result is {}", operator[1], result);
    }
}
