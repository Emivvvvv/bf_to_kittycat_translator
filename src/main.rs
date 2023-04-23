use std::fs;
use std::io::Write;

fn main() {
    let instructions = parse_input(read_file_string("src/input.txt").unwrap());

    let mut output = String::new();

    for instruction in instructions.chars(){
        match instruction {
            '>' => output += "prrr",
            '<' => output += "mrrr",
            '+' => output += "MEOW",
            '-' => output += "meow",
            ',' => output += "mEOw",
            '.' => output += "MeoW",
            '[' => output += "Meow",
            ']' => output += "meoW",
            _ => panic!("TISSSSS MEOW MEOW MEOW (I NO KNOW THIS HUMAN WURDS!!)"),
        }

        output += " ";
    }

    print_file_string("src/output.txt", output);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;

    Ok(data)
}

fn print_file_string(filepath: &str, output_string: String) {
    let mut file = std::fs::File::create("output.txt").expect("create failed");
    file.write_all(output_string.as_bytes()).expect("write failed");
}

fn parse_input(input: String) -> String {
    let trimmed_vec = input.trim().split(&[' ', '\n', '\t', '\r']).collect::<Vec<&str>>();
    let mut trimmed = String::new();
    for command in trimmed_vec {
        trimmed += command;
    }
    trimmed
}