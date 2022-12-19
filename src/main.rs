use std::env;

fn main() {
    let key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY env variable should be set");

    let args = env::args();

    if args.len() > 2 {
        panic!("Too much args! Note that you should add double quotes around the prompt.");
    }

    let question: String = args.skip(1).collect();

    dbg!(key, question);
}
