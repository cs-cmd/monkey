use monkeylang_rs::lexer::Lexer;
use std::io::Write;

const PROMPT: &str = ">> ";

pub fn start() -> () {
    // TODO: Change this to support other types of input/output (why?)
    let term_in = std::io::stdin();
    let mut term_out = std::io::stdout();

    loop {
        let _ = term_out.write(PROMPT.as_bytes());
        let _ = term_out.flush();

        let mut input = String::new();
        let _ = term_in.read_line(&mut input);

        let mut line_lexer = Lexer::new(&input);

        while line_lexer.has_next() {
            let tok = line_lexer.next_token();

            println!("{:?}", tok);
        }
    }
}
