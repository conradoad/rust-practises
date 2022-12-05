fn main() {
    conditional_expression();
    loop_repetition();
    nested_loops();
    while_loop();
    for_loop();
    for_loop_with_range();
}

fn conditional_expression() {
    println!("\nConditional expression example:");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loop_repetition() {
    println!("\nLoop repetition example:");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn nested_loops() {
    println!("\nNested loops example:");
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    println!("\nWhile loop example:");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    println!("\nFor loop example:");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_loop_with_range() {
    println!("\nFor loop with range example:");
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}