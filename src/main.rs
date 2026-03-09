use clap::Parser;
use colored::*;
use putain::config::Config;
use putain::hook;
use putain::matcher::Matcher;
use putain::personality::Personality;
use putain::shell::{CommandContext, Shell};

#[derive(Parser)]
#[command(
    name = "putain",
    version,
    about = "Un correcteur de commandes shell. Comme thefuck, mais en français."
)]
struct Cli {
    /// Generate shell hook script (bash, zsh, fish)
    #[arg(long)]
    hook: Option<String>,

    /// Called internally by the shell hook
    #[arg(long, hide = true)]
    from_hook: bool,

    /// Skip confirmation, just run the fix
    #[arg(short = 'y', long)]
    yes: bool,
}

fn main() {
    let cli = Cli::parse();

    // Hook generation mode
    if let Some(ref shell_name) = cli.hook {
        match Shell::from_env_str(shell_name) {
            Some(shell) => {
                println!("{}", hook::generate_hook(shell));
                return;
            }
            None => {
                eprintln!(
                    "{}",
                    format!("Shell inconnu: '{}'. Utilise bash, zsh ou fish.", shell_name).red()
                );
                std::process::exit(1);
            }
        }
    }

    // Get the failed command context
    let ctx = if cli.from_hook {
        CommandContext::from_env()
    } else {
        CommandContext::from_history()
    };

    let ctx = match ctx {
        Some(ctx) => ctx,
        None => {
            eprintln!(
                "{}",
                "Putain, j'arrive pas à trouver la dernière commande. Installe le hook pour de meilleurs résultats.".yellow()
            );
            if let Some(shell) = Shell::detect() {
                eprintln!("{}", hook::install_instructions(&shell));
            }
            std::process::exit(1);
        }
    };

    // Find a correction
    let config = Config::default();
    let matcher = Matcher::new(&config);

    match matcher.find_correction(&ctx) {
        Some(correction) => {
            let mut personality = Personality::new();
            let reaction = personality.react(&correction.rule_name);

            eprintln!("{}", reaction.red().bold());
            println!("{}", format!("→ {}", correction.fixed_command).green().bold());

            if cli.yes || cli.from_hook {
                run_correction(&correction.fixed_command);
            } else {
                eprint!("{}", "Exécuter ? [O/n] ".yellow());
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_ok() {
                    let input = input.trim().to_lowercase();
                    if input.is_empty() || input == "o" || input == "oui" || input == "y" || input == "yes" {
                        run_correction(&correction.fixed_command);
                    }
                }
            }
        }
        None => {
            eprintln!(
                "{}",
                "Putain... j'ai rien trouvé pour corriger ça. T'es tout seul sur ce coup.".red()
            );
            std::process::exit(1);
        }
    }
}

fn run_correction(command: &str) {
    let status = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .status();

    match status {
        Ok(s) => std::process::exit(s.code().unwrap_or(0)),
        Err(e) => {
            eprintln!("{}", format!("Putain, même la correction a merdé: {}", e).red());
            std::process::exit(1);
        }
    }
}
