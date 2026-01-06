use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use swiftsc_frontend::{parse, tokenize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tokenize a source file
    Lex { path: PathBuf },
    /// Parse a source file into AST
    Parse { path: PathBuf },
    /// Parse and check semantics
    Check {
        path: PathBuf,
        #[arg(short, long)]
        root: Option<PathBuf>,
    },
    /// Compile to WASM
    Build {
        path: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        root: Option<PathBuf>,
    },
    /// Initialize a new SwiftSC-Lang project
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Run tests
    Test {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Deploy contract to blockchain
    Deploy {
        path: PathBuf,
        #[arg(short, long)]
        network: String,
        #[arg(short, long)]
        root: Option<PathBuf>,
    },
    /// Run security analysis
    Analyze {
        path: PathBuf,
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Lex { path } => {
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("could not read file `{}`", path.display()))?;

            println!("--- Lexing: {} ---", path.display());
            let tokens = tokenize(&content);

            for (token, span) in tokens {
                println!("{:?} => {:?}", span, token);
            }
        }
        Commands::Parse { path } => {
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("could not read file `{}`", path.display()))?;

            println!("--- Parsing: {} ---", path.display());
            match parse(&content) {
                Ok(ast) => println!("{:#?}", ast),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Check { path, root } => {
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("could not read file `{}`", path.display()))?;

            match parse(&content) {
                Ok(ast) => match swiftsc_frontend::analyze(&ast, root.clone()) {
                    Ok(_) => println!("Semantic Check Passed"),
                    Err(e) => eprintln!("Semantic Error: {}", e),
                },
                Err(e) => eprintln!("Parse Error: {}", e),
            }
        }
        Commands::Build { path, output, root } => {
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("could not read file `{}`", path.display()))?;

            println!("--- Compiling: {} ---", path.display());
            match parse(&content) {
                Ok(ast) => match swiftsc_frontend::analyze(&ast, root.clone()) {
                    Ok(_) => match swiftsc_backend::compile(&ast) {
                        Ok(wasm_bytes) => {
                            let output_path = output
                                .clone()
                                .unwrap_or_else(|| path.with_extension("wasm"));
                            std::fs::write(&output_path, wasm_bytes).with_context(|| {
                                format!("could not write output file `{}`", output_path.display())
                            })?;
                            println!("Build Successful: {}", output_path.display());
                        }
                        Err(e) => eprintln!("Codegen Error: {}", e),
                    },
                    Err(e) => eprintln!("Semantic Error: {}", e),
                },
                Err(e) => eprintln!("Parse Error: {}", e),
            }
        }
        Commands::Init { path } => {
            println!(
                "--- Initializing SwiftSC-Lang project in: {} ---",
                path.display()
            );

            // Create project structure
            std::fs::create_dir_all(path.join("src"))?;
            std::fs::create_dir_all(path.join("tests"))?;

            // Create SwiftSC-Lang.toml
            let config = r#"[package]
name = "my-contract"
version = "1.0.3"

[dependencies]
# stdlib is included by default

[build]
target = "wasm32-unknown-unknown"
gas_metering = true
"#;
            std::fs::write(path.join("SwiftSC-Lang.toml"), config)?;

            // Create example contract
            let example = r#"use std::math::sub;
use std::collections::HashMap;

contract MyContract {
    storage balances: HashMap<Address, u64>;

    fn transfer(to: Address, amount: u64) -> Result<()> {
        let sender = caller();
        let bal = self.balances.get(sender).unwrap_or(0);
        
        // V1.0.3 Safe Math
        let new_bal = sub(bal, amount)?;
        
        self.balances.insert(sender, new_bal);
        self.balances.insert(to, self.balances.get(to).unwrap_or(0) + amount);
        
        Ok(())
    }
}
"#;
            std::fs::write(path.join("src/contract.stc"), example)?;

            println!("✓ Project initialized successfully!");
            println!("  - SwiftSC-Lang.toml");
            println!("  - src/contract.stc");
            println!("  - tests/");
        }
        Commands::Test { path } => {
            println!("--- Running tests in: {} ---", path.display());

            // Find all test files
            let test_dir = path.join("tests");
            if test_dir.exists() {
                println!("✓ Test directory found");
                println!("  (Test execution not yet implemented)");
            } else {
                eprintln!("✗ No tests directory found");
            }
        }
        Commands::Analyze { path, verbose } => {
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("could not read file `{}`", path.display()))?;

            match parse(&content) {
                Ok(ast) => {
                    if *verbose {
                        println!("--- Analyzing AST: {} ---", path.display());
                        println!("Pass 1: Reentrancy Detection");
                        println!("Pass 2: Integer Overflow Check");
                        println!("Pass 3: Uninitialized Storage Check");
                    }
                    let warnings = swiftsc_analyzer::SecurityAnalyzer::analyze(&ast);
                    if warnings.is_empty() {
                        println!("✓ No security issues found.");
                    } else {
                        println!("⚠️ Found {} security warnings:", warnings.len());
                        for warning in warnings {
                            println!("  - {:?}", warning);
                        }
                    }
                }
                Err(e) => eprintln!("✗ Parse error: {}", e),
            }
        }
    }

    Ok(())
}
