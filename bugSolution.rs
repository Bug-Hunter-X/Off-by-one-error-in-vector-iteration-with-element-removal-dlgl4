fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Method 1: Iterate backward
    for i in (0..numbers.len()).rev() {
        if numbers[i] % 2 == 0 {
            numbers.remove(i);
        }
    }
    println!("Result (Method 1): {:?}", numbers);

    // Method 2: Create a new vector
    let mut numbers2 = vec![1, 2, 3, 4, 5];
    let mut numbers3 = Vec::new();
    for num in numbers2 {
        if num % 2 != 0 {
            numbers3.push(num);
        }
    }
    println!("Result (Method 2): {:?}", numbers3);

    // Method 3: Using retain
    let mut numbers4 = vec![1, 2, 3, 4, 5];
    numbers4.retain(|&x| x % 2 != 0);
    println!("Result (Method 3): {:?}", numbers4);
} 