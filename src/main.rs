fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(text: String) -> String {
    let first_word = text.split_whitespace().next().unwrap_or("").to_owned();
    let mut message = text;
    let mut output = String::new();

    output.push_str(&first_word);
    output.push('\n');

    {
        let mutable_message = &mut message;
        mutable_message.push_str(" rust");
        output.push_str(mutable_message);
        output.push('\n');
    }

    {
        let shared_message = &message;
        output.push_str(shared_message);
        output.push('\n');
    }

    output
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
