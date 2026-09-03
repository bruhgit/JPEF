use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

use jpef::core::config::{AppMetadata, BuildConfig, JvmOptions, TargetPlatform};
use jpef::core::converter::convert;
use jpef::core::manifest::inspect_jar;

const BANNER: &str = r#"
     _ ____  _____ _____ 
    | |  _ \| ____|  ___|
 _  | | |_) |  _| | |_   
| |_| |  __/| |___|  _|  
 \___/|_|   |_____|_|    
"#;

#[derive(Parser)]
#[command(name = "jpef")]
#[command(author = "JPEF Team")]
#[command(version = "1.0.0")]
#[command(about = "Java Portable Executable Format - Convert JAR to .exe, .elf, and .app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a JAR into native executables (.exe, .elf, .app)
    Convert {
        /// Path to input .jar file
        jar: PathBuf,

        /// Target platforms (comma-separated: exe,elf,app)
        #[arg(short, long, default_value = "exe,elf,app")]
        targets: String,

        /// Output directory
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,

        /// Custom application/executable name
        #[arg(short, long)]
        name: Option<String>,

        /// Path to icon image (PNG, JPG, ICO)
        #[arg(short, long)]
        icon: Option<PathBuf>,

        /// Build as console application instead of GUI
        #[arg(long)]
        console: bool,

        /// Initial JVM heap memory (e.g. 256m, 1g)
        #[arg(long)]
        min_heap: Option<String>,

        /// Maximum JVM heap memory (e.g. 1024m, 4g)
        #[arg(long)]
        max_heap: Option<String>,

        /// Additional JVM argument (can be repeated)
        #[arg(long = "jvm-arg")]
        jvm_args: Vec<String>,

        /// Application version string
        #[arg(long, default_value = "1.0.0.0")]
        version: String,

        /// Company name metadata
        #[arg(long, default_value = "JPEF")]
        company: String,
    },

    /// Inspect a JAR archive's manifest and Java version requirements
    Inspect {
        /// Path to input .jar file
        jar: PathBuf,
    },

    /// Show version information
    Version,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Version => {
            println!("{}", BANNER.cyan().bold());
            println!("Java Portable Executable Format v1.0.0 (Rust Core)");
        }
        Commands::Inspect { jar } => {
            println!("{}", BANNER.cyan().bold());
            match inspect_jar(&jar) {
                Ok(info) => {
                    println!("{}", format!("JAR Archive: {}", jar.display()).green().bold());
                    println!("  File Size:    {:.2} MB ({} bytes)", info.file_size_bytes as f64 / (1024.0 * 1024.0), info.file_size_bytes);
                    println!("  Entries:      {} files", info.entry_count);
                    println!("  Main-Class:   {}", info.main_class.as_deref().unwrap_or("(None)").yellow().bold());
                    println!("  Runnable:     {}", if info.is_valid_runnable { "Yes".green() } else { "No".red() });
                    if let Some(v) = info.min_java_version {
                        println!("  Java Version: Java {}+ (bytecode {})", v, info.bytecode_major_version.unwrap_or(0));
                    }
                    if let Some(ref title) = info.implementation_title {
                        println!("  Title:        {}", title);
                    }
                    if let Some(ref ver) = info.implementation_version {
                        println!("  Version:      {}", ver);
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Convert {
            jar,
            targets,
            output,
            name,
            icon,
            console,
            min_heap,
            max_heap,
            jvm_args,
            version,
            company,
        } => {
            println!("{}", BANNER.cyan().bold());

            let target_platforms: Vec<TargetPlatform> = targets
                .split(',')
                .filter_map(|t| TargetPlatform::from_str(t.trim()))
                .collect();

            if target_platforms.is_empty() {
                eprintln!("{}: No valid targets specified. Use 'exe', 'elf', or 'app'.", "Error".red().bold());
                std::process::exit(1);
            }

            let app_name = name.unwrap_or_else(|| {
                jar.file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "JavaApp".to_string())
            });

            let mut config = BuildConfig::new(&jar, &output);
            config.targets = target_platforms;
            config.is_gui = !console;
            config.icon_path = icon;
            config.metadata = AppMetadata {
                app_name: app_name.clone(),
                version,
                company_name: company,
                file_description: format!("{} Application", app_name),
                copyright: "Copyright (C) 2026".to_string(),
                bundle_id: format!("com.jpef.{}", app_name.to_lowercase()),
                main_class: None,
            };
            config.jvm = JvmOptions {
                min_heap,
                max_heap,
                custom_args: jvm_args,
                bundled_jre_path: Some("jre".to_string()),
                min_java_version: 8,
            };

            println!("Profile:");
            println!("  Input:   {}", jar.display());
            println!("  App:     {}", app_name.yellow().bold());
            println!("  Mode:    {}", if console { "Console" } else { "GUI (No Console)" });
            println!("  Output:  {}", output.display());
            println!();

            let result = convert(&config, Some(&|msg| println!("  {} {}", ">".blue(), msg)));

            println!();
            if result.success {
                println!("{}", "[OK] Build completed successfully!".green().bold());
                println!("Generated Artifacts:");
                for art in &result.artifacts {
                    println!(
                        "  - [{}] {} ({:.2} MB)",
                        art.platform.cyan(),
                        art.path.display().to_string().green(),
                        art.size_bytes as f64 / (1024.0 * 1024.0)
                    );
                }
            } else {
                eprintln!("{}", "[FAIL] Conversion encountered errors:".red().bold());
                for err in &result.errors {
                    eprintln!("  - {}", err.red());
                }
                std::process::exit(1);
            }
        }
    }
}
