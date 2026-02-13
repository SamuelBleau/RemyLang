use remylang::lexer::Lexer;
use remylang::parser::Parser;
use remylang::vm::Interpreter;
use remylang::llvm_backend::{LLVMCompilerContext, CodeGenerator};
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CompilerMode {
    VM,      // Traditional interpreter
    LLVM,    // LLVM compilation
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let (mode, filename) = if args.len() > 1 && args[1] == "--llvm" {
        // --llvm mode
        if args.len() > 2 {
            (CompilerMode::LLVM, Some(args[2].as_str()))
        } else {
            (CompilerMode::LLVM, None)
        }
    } else if args.len() > 1 {
        // VM mode with file
        (CompilerMode::VM, Some(args[1].as_str()))
    } else {
        // Demo mode
        (CompilerMode::VM, None)
    };

    match filename {
        Some(f) => run_file(f, mode),
        None => run_demo(mode),
    }
}

fn run_file(filename: &str, mode: CompilerMode) {
    println!("=== RemyLang v1.0.0 - Running {} ({:?} mode) ===\n", filename, mode);

    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file: {}", e);
            return;
        }
    };
    
    execute_code(&code, mode);
}

fn run_demo(mode: CompilerMode) {
    println!("=== RemyLang v1.0.0 - Demo ({:?} mode) ===\n", mode);

    // Example code to execute - simple enough to compile with LLVM
    let code = match mode {
        CompilerMode::LLVM => {
            r#"
Int x = 10;
Int y = 5;
Int sum = x + y;
"#
        }
        CompilerMode::VM => {
            // Full-featured demo for the interpreter
            r#"
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
"#
        }
    };

    println!("📝 Source Code:");
    println!("{}", "=".repeat(60));
    println!("{}", code);
    println!("{}\n", "=".repeat(60));
    
    execute_code(code, mode);
}

fn execute_code(code: &str, mode: CompilerMode) {
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
    
    match mode {
        CompilerMode::VM => execute_with_interpreter(ast),
        CompilerMode::LLVM => execute_with_llvm(ast),
    }
}

fn execute_with_interpreter(ast: Vec<remylang::ast::Stmt>) {
    println!("🚀 Execution (VM Interpreter):");
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

fn execute_with_llvm(ast: Vec<remylang::ast::Stmt>) {
    println!("🔨 Compilation (LLVM Backend):");
    println!("{}", "=".repeat(60));

    // Create LLVM context and module
    let llvm_ctx = LLVMCompilerContext::new();
    let context = llvm_ctx.get_context();
    let module = llvm_ctx.create_module("remylang_program");
    let builder = llvm_ctx.create_builder();

    let mut codegen = CodeGenerator::new(context, module, builder);

    // Create a main function context for the builder to have a valid insertion point
    codegen.create_test_function("main");

    // Compile the program
    match codegen.compile_program(&ast) {
        Ok(_) => {
            println!("✓ LLVM IR generated successfully\n");
            println!("📋 Generated LLVM IR:");
            println!("{}", "=".repeat(60));
            codegen.print_ir();
            println!("{}", "=".repeat(60));
            println!("✓ Compilation successful!");
        }
        Err(e) => {
            println!("{}", "=".repeat(60));
            eprintln!("❌ Compilation Error: {}", e);
        }
    }
}

