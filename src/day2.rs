pub fn run(input: String) {
    let mut total_sum_a: u64 = 0;
    let mut total_sum_b: u64 = 0;
    for product_id_range in input.split(",") {
        let product_id_parts: Vec<u64> = product_id_range.split("-").map(|x| x.parse::<u64>().unwrap()).collect();
        let start_id = product_id_parts[0];
        let end_id = product_id_parts[1];
        for product_id in start_id..end_id+1{
            // A
            let product_id_str = product_id.to_string();
            let mid_point = product_id_str.len() / 2;
            if product_id_str[0..mid_point] == product_id_str[mid_point..] {
                total_sum_a = total_sum_a + product_id;
            }
            // B
            for i in 1..product_id_str.len()/2 + 1{
                let chunk = &product_id_str[..i];
                if chunk.repeat(product_id_str.len()/i) == product_id_str {
                    total_sum_b += product_id;
                    break
                }
            }
        }
    }
    println!("Part 2a: {total_sum_a}");
    println!("Part 2b: {total_sum_b}");
}
