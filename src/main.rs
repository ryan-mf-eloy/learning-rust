fn main() {
    let _input_string = "A 2 B 4 C 6 D 8 E 10";
    let input_number = "1 2 3 4 5 6 7 8 9 10";

    let result: Vec<i32> = input_number
        .split(' ')
        .map(|s| {
            let converted_number = s.parse::<i32>().unwrap();
            let doubled_number = converted_number * 2;

            doubled_number
        })
        .collect();

    println!("{:?}", result);
}