fn multiple_any(x: i64, ys: &[i64]) -> bool {
    ys.iter().any(|&y| x % y == 0)
}

fn main() {
    let multiples_of = [3, 5, 7];
    
    // Create an iterator that generates an infinite sequence of numbers
    let sum: i64 = (1..)
        .map(|n| n * n) // Square the number
        .take_while(|&square| square < 5000) // Take squares less than 5000
        .filter(|&square| multiple_any(square, &multiples_of)) // Filter by multiples of 3, 5, or 7
        .sum(); // Sum the resulting squares
    
    println!("Sum of squares that are multiples of 3, 5, or 7 and smaller than 5000: {}", sum);
}
