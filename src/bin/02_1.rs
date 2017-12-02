fn main() {

    let matrix = [[5, 1, 9, 5], [7, 5, 3, -1], [2, 4, 6, 8]];

    let mut numbers = Vec::new();

    let mut row_index = 1;
    for row in matrix.iter() {
        let mut largest = row.first().unwrap();
        let mut smallest = row.first().unwrap();

        println!("row: {}", row_index);
        for number in row.iter() {
            if number > largest {
                largest = number;
            }
            if number < smallest && /*hax*/ *number != -1 {
                smallest = number;
            }
        } // row
        row_index += 1;
        let diff = largest - smallest;
        println!("biggest: {}, smallest: {}, diff: {}", largest, smallest, diff);
        numbers.push(diff);
    }

    let mut result = 0;
    for i in numbers {
        result += i;
    }
    println!("result: {}", result);
}
