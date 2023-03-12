fn main() {
    let condition = true;
    let number = if condition { 6 } else { 5 };

    println!("The value of number is: {number}");

    if number != 0 {
        println!("Number was something other than zero");
    }

    if number < 5 {
        println!("Condition was true");
    }

    // else {
    //     println!("Condition was false");
    // }

    if number % 4 == 0 {
        println! {"Number is divisible by 4"};
    } else if number % 3 == 0 {
        println! {"Number is divisible by 3"};
    } else if number % 2 == 0 {
        println! {"Number is divisible by 2"};
    } else {
        println! {"Number is not divisible by 4, 3 or 2"};
    }
}
