mod lexer;

use lexer::Lexer;

fn main() {
    println!("RemyLang v0.1.0");
    
    // Example usage
    let code = "Int nb = 42;";
    let lexer = Lexer::new(code.to_string());
    
    println!("\nTokenizing: {}", code);
    println!("Tokens:");
    for token in lexer {
        println!("  {:?}", token);
    }
}
