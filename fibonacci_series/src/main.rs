fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = prev + current;
        prev = current;
        current = next;
    }

    current
}

fn main() {
    println!("Fibonacci Numbers from 0 to 10:");
    for i in 0..= 100 {
        println!("Fibonacci({}) = {}", i, fibonacci(i));
    }
}
