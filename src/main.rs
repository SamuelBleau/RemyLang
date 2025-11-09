use remylang::lexer::Lexer;
use remylang::parser::Parser;

fn main() {
    println!("=== RemyLang v0.1.0 - Parser Demo ===\n");
    
    // Example code to parse
    let code = r#"
func Add(Int a, Int b) -> Int {
    return a + b;
}

func Main() -> Int {
    Int x = 10;
    Int y = 5;
    Int result = Add(x, y);
    
    if (result > 10) {
        print("Result is greater than 10");
    } else {
        print("Result is 10 or less");
    }
    
    return 0;
}
"#;

    println!("📝 Source Code:");
    println!("{}", "=".repeat(60));
    println!("{}", code);
    println!("{}\n", "=".repeat(60));
    
    // Step 1: Tokenization
    println!("🔤 Tokenization:");
    println!("{}", "=".repeat(60));
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token();
        println!("  {:?}", token);
        if token == remylang::lexer::Token::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    println!("{}\n", "=".repeat(60));
    
    // Step 2: Parsing
    println!("🌳 Abstract Syntax Tree (AST):");
    println!("{}", "=".repeat(60));
    let mut parser = Parser::new(tokens);
    
    match parser.parse() {
        Ok(ast) => {
            for (i, stmt) in ast.iter().enumerate() {
                println!("Statement {}:", i + 1);
                println!("{:#?}\n", stmt);
            }
        }
        Err(e) => {
            eprintln!("❌ Parse Error: {}", e);
        }
    }
    println!("{}", "=".repeat(60));
}
