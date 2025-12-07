pub fn run(input: String) {
    let mut count_a = 0;
    let mut count_b = 0;
    let mut pos = 50;

    for instruction in  input.split("\n") {
        if instruction.len() == 0 {
            break;
        }
        let dir = match instruction.chars().nth(0).unwrap() {
            'R' => 1,
            'L' => -1,
            _ => 0,
        };
        let magnitude_str = &instruction[1..];
        let magnitude_int = magnitude_str.parse::<i32>().unwrap();

        let start_pos = pos;
        let new_pos = pos + (magnitude_int * dir);
        pos = new_pos.rem_euclid(100);

        if new_pos >= 100{
            count_b += new_pos/100;
        }
        if new_pos < 1 {
            count_b += (-(new_pos)).div_euclid(100) + 1;
            if start_pos == 0 {
                count_b -= 1;
            }
        }

        if pos == 0 {
            // count_b += 1;
            count_a += 1;
        }
        println!("instruction {instruction} pos {pos}  count {count_b}");
    }
    println!("Part 1a: {count_a}");
    println!("Part 1b: {count_b}");
}
