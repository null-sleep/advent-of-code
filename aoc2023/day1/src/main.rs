use ::std::fs;

// https://adventofcode.com/2023/day/1

fn read_input() -> Vec<String> {
    let file_path = "src/input.txt";
    let input = fs::read_to_string(file_path)
        .expect(format!("Error reading input file: {}", file_path).as_str());

    let input: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    // Filter empty lines from input
    let input: Vec<String> = input.into_iter().filter(|line| !line.is_empty()).collect();

    // let mut input: Vec<i32> = input
    //     .lines()
    //     .map(|line| line.parse::<i32>().unwrap())
    //     .collect();
    input
}

fn parse_calibration_value(line: String) -> i32 {
    let mut first_number = -1;
    let mut second_number = -1;

    for char in line.chars() {
        if char.is_ascii_digit() {
            if first_number == -1 {
                first_number = char.to_digit(10).unwrap() as i32;
                second_number = first_number;
            } else {
                second_number = char.to_digit(10).unwrap() as i32;
            }
        }
    }
    first_number * 10 + second_number
}

fn to_calibration_values(input: Vec<String>) -> Vec<i32> {
    input.into_iter().map(|line| parse_calibration_value(line)).collect()
}

fn main() {
    let input = read_input();
    let calibration_values = to_calibration_values(input);
    println!("Calibration values: {:?}", calibration_values);
    let sum: i32 = calibration_values.iter().sum();
    println!("Calibration sum: {}", sum);
}
