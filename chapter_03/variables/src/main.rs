fn main() {
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("3h = {THREE_HOURS_IN_SECONDS}s");

    let y = 6;
    let y = y + 1;

    {
        let y = y * 2;

        println!("The value of y in the inner scope is: {y}")
    }

    println!("The value of y is: {y}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("Contains {spaces} spaces");

    let sum = 5 + 10;
    println!("Addition, {sum}");

    let difference = 95.5 - 4.3;
    println!("Difference, {difference}");

    let product = 4 * 30;
    println!("Multiplication, {product}");

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("Division quotient, {quotient}");
    println!("Division truncated, {truncated}");

    let remainder = 43 % 5;
    println!("Remainder, {remainder}");

    // char literals are specified with single quotes
    let heart_eyed_cat: char = '😻';
    println!("{heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of x is: {x}, {five_hundred}");
    println!("The value of y is: {y}, {six_point_four}");
    println!("The value of z is: {z}, {one}");

    let a = [3; 5];
    let months: [&str; 11] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first_number = a[0];
    let second_number = a[1];
    println!("First number, {first_number}");
    println!("Second number, {second_number}");

    let last_month = months[months.len() - 1];
    let second_last_month = months[months.len() - 2];
    println!("Last month, {last_month}");
    println!("Second last month, {second_last_month}");
}
