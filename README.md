# Homework 4

### Fizz buzz program
1. Create a project called bootcamp using
Cargo
2. The main function should print a
welcome message.
3. Write a 'fizz buzz' function that will be
called from your main function.
1. The function should have a loop counting
up to 301
2. If the count is divisible by 3, print "fizz"
3. If the count is divisible by 5 print "buzz"
4. If the count is divisible by 3 and 5 print
"fizz buzz"
5. At the end print the number of times
"fizz buzz" occurred.


### Another mathods

I think the following way is more elegant:

``` rust
 for i in 1..=301 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => {},
        }
  }
```