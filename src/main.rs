use std::vec::Vec;

fn main() {
    const COUNT: usize = 100;

    let mut vector = Vec::with_capacity(COUNT);

    for index in 1..COUNT {
        let result = match (index % 3, index % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => format!("{}", index),
        };

        vector.push(result);
    }

    print_result(&vector);
}

fn print_result(collection: &[String]) {
    for index in collection {
        println!("{}", index)
    }
}
