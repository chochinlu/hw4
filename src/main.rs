fn fizzbuzz() {
    let mut count = 0;

    for n in 1..=301 {
        match n {
            n if n % 3 == 0 && n % 5 == 0 => {
                println!("fizz buzz");
                count = count + 1;
            }
            n if n % 3 == 0 => println!("fizz"),
            n if n % 5 == 0 => println!("buzz"),
            _ => {}
        }
    }

    println!("Number of 'fizz buzz' occurred: {count}")
}

fn main() {
    println!("Welcome!! It's a bootcamp homework #4");

    fizzbuzz();
}
