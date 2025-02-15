fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut index = 0;

    loop {
        if index >= numbers.len() {
            break;
        }

        let num = numbers[index];

        if num % 2 == 0 {
            numbers.remove(index);
        } else {
            index += 1;
        }
    }

    println!("Result: {:?}", numbers);
}