use remylang::lexer::Lexer;
use remylang::parser::Parser;
use remylang::vm::Interpreter;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // File mode
        let filename = &args[1];
        run_file(filename);
    } else {
        // Demo mode
        run_demo();
    }
}

fn run_file(filename: &str) {
    println!("=== RemyLang v1.0.0 - Running {} ===\n", filename);
    
    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file: {}", e);
            return;
        }
    };
    
    execute_code(&code);
}

fn run_demo() {
    println!("=== RemyLang v0.1.0 - Interpreter Demo ===\n");
    
    // Example code to execute
    let code = r#"
Int x = 10;
Int y = 5;

print("x =", x);
print("y =", y);

Int sum = x + y;
print("x + y =", sum);

Int product = x * y;
print("x * y =", product);

if (sum > 10) {
    print("Sum is greater than 10");
} else {
    print("Sum is 10 or less");
}

Array<Int> numbers = [1, 2, 3, 4, 5];
print("Array:", numbers);
print("First element:", numbers[0]);
print("Third element:", numbers[2]);
"#;

    println!("📝 Source Code:");
    println!("{}", "=".repeat(60));
    println!("{}", code);
    println!("{}\n", "=".repeat(60));
    
    execute_code(code);
}

fn execute_code(code: &str) {
    // Step 1: Tokenization
    println!("🔤 Tokenization...");
    let mut lexer = Lexer::new(code.to_string());
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token();
        if token == remylang::lexer::Token::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    println!("✓ {} tokens generated\n", tokens.len() - 1);
    
    // Step 2: Parsing
    println!("🌳 Parsing...");
    let mut parser = Parser::new(tokens);
    
    let ast = match parser.parse() {
        Ok(ast) => {
            println!("✓ {} statements parsed\n", ast.len());
            ast
        }
        Err(e) => {
            eprintln!("❌ Parse Error: {}", e);
            return;
        }
    };
    
    // Step 3: Execution
    println!("🚀 Execution:");
    println!("{}", "=".repeat(60));
    let mut interpreter = Interpreter::new();
    
    match interpreter.execute(ast) {
        Ok(_) => {
            println!("{}", "=".repeat(60));
            println!("✓ Program executed successfully");
        }
        Err(e) => {
            println!("{}", "=".repeat(60));
            eprintln!("❌ Runtime Error: {}", e);
        }
    }
}
