# 🧪 Testing & Coverage Guide

## Running Tests

### Basic test commands

```bash
# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a specific test
cargo test test_simple_number

# Run tests in a specific module
cargo test lexer::tests

# List all tests without running
cargo test -- --list

# Run tests in parallel (default) or sequentially
cargo test -- --test-threads=1
```

### Test output

```
running 50 tests
test lexer::tests::test_empty_input ... ok
test lexer::tests::test_single_number ... ok
test lexer::tests::test_arithmetic_operators ... ok
...

test result: ok. 50 passed; 0 failed; 0 ignored; 0 measured
```

---

## Code Coverage

### Method 1: cargo-tarpaulin (Recommended for Linux)

#### Installation
```bash
cargo install cargo-tarpaulin
```

#### Usage
```bash
# Generate coverage report
cargo tarpaulin --verbose --all-features --workspace --timeout 120

# Generate HTML report
cargo tarpaulin --out Html --output-dir coverage/

# Open the HTML report
xdg-open coverage/index.html  # Linux
# or
open coverage/index.html      # macOS
```

#### Output example
```
|| Tested/Total Lines:
|| src/lexer/lexer.rs: 245/250 (98.00%)
|| src/lexer/token.rs: 45/45 (100.00%)
||
98.33% coverage, 290/295 lines covered
```

---

### Method 2: cargo-llvm-cov (Cross-platform)

#### Installation
```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
```

#### Usage
```bash
# Generate coverage report in terminal
cargo llvm-cov

# Generate HTML report
cargo llvm-cov --html
open target/llvm-cov/html/index.html

# Generate detailed report
cargo llvm-cov --open  # Opens browser automatically

# Export to various formats
cargo llvm-cov --lcov --output-path coverage.lcov  # LCOV format
cargo llvm-cov --json --output-path coverage.json  # JSON format
```

---

### Method 3: grcov (Alternative)

#### Installation
```bash
cargo install grcov
rustup component add llvm-tools-preview
```

#### Usage
```bash
# Set environment variables
export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="coverage-%p-%m.profraw"

# Clean previous builds
cargo clean

# Run tests
cargo test

# Generate report
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/

# Open report
xdg-open coverage/index.html
```

---

## Understanding Coverage Metrics

### Line Coverage
Percentage of code lines executed during tests.
- **90-100%**: Excellent
- **70-89%**: Good
- **Below 70%**: Needs improvement

### Branch Coverage
Percentage of code branches (if/else, match arms) tested.

### Function Coverage
Percentage of functions called during tests.

---

## Coverage Best Practices

### What to aim for
- ✅ **Lexer**: 95%+ coverage (critical component)
- ✅ **Parser**: 90%+ coverage
- ✅ **VM**: 85%+ coverage
- ✅ **Overall**: 85%+ coverage

### What's OK to skip
- Error messages formatting
- Debug output
- Unreachable code (clearly marked with comments)

### Example: Ignoring code from coverage
```rust
#[cfg(not(tarpaulin_include))]
fn debug_only_function() {
    // This won't be included in coverage
}
```

---

## CI/CD Integration

### GitHub Actions (already configured in .github/workflows/coverage.yml)

The workflow will:
1. Run all tests
2. Generate coverage report
3. Upload to codecov.io (if configured)

### Setting up Codecov
1. Go to https://codecov.io/
2. Connect your GitHub repository
3. Add badge to README.md:
```markdown
[![codecov](https://codecov.io/gh/SamuelBleau/RemyLang/branch/main/graph/badge.svg)](https://codecov.io/gh/SamuelBleau/RemyLang)
```

---

## Quick Start

```bash
# Install coverage tool (choose one)
cargo install cargo-tarpaulin  # Linux only
# OR
cargo install cargo-llvm-cov   # Cross-platform

# Run tests
cargo test

# Generate coverage
cargo tarpaulin --out Html --output-dir coverage/
# OR
cargo llvm-cov --html

# View report
xdg-open coverage/index.html
```

---

## Current Test Suite

### Lexer Tests (50 tests)
- ✅ Basic tokens (numbers, operators, keywords)
- ✅ Identifiers and string/char literals
- ✅ Comments (line and block)
- ✅ Complex expressions
- ✅ Variable declarations
- ✅ Function declarations
- ✅ Array syntax
- ✅ Conditionals
- ✅ Invalid tokens
- ✅ Complete programs

### Coverage Target
- Lexer: 95%+ (currently implemented)
- Parser: 90%+ (to be implemented)
- VM: 85%+ (to be implemented)

---

## Troubleshooting

### cargo-tarpaulin fails on macOS/Windows
Use `cargo-llvm-cov` instead (cross-platform).

### Coverage seems low
- Check if all code paths are tested
- Add tests for edge cases
- Test error conditions

### Tests pass but coverage tool fails
- Ensure you have `llvm-tools-preview`: `rustup component add llvm-tools-preview`
- Try cleaning: `cargo clean` then rerun

---

## Resources

- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [Rust testing guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Codecov documentation](https://docs.codecov.io/)
