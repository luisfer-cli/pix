use crate::diagnostics::{Diagnostic, JsonDiagnostics};
use crate::{fmt, parser, render, source, spec, validate};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "pix",
    version,
    about = "Deterministic pixel art renderer for the .pix DSL"
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate a .pix file
    Check {
        input: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// Render a .pix file or one named frame to PNG
    Render {
        input: Option<PathBuf>,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long)]
        frame: Option<String>,
        #[arg(long, default_value_t = 1)]
        scale: u32,
    },
    /// Format a .pix file
    Fmt {
        input: Option<PathBuf>,
        #[arg(long)]
        write: bool,
    },
    /// Print parsed AST
    Ast {
        input: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// Print compact DSL spec
    Spec {
        #[arg(long)]
        json: bool,
    },
    /// Export each frame of an animation as individual PNG files
    Anim {
        input: Option<PathBuf>,
        #[arg(short, long)]
        animation: String,
        #[arg(long)]
        out_dir: PathBuf,
        #[arg(long, default_value_t = 1)]
        scale: u32,
    },
    /// Export an animation as a spritesheet PNG
    Sheet {
        input: Option<PathBuf>,
        #[arg(short, long)]
        animation: String,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long)]
        json: Option<PathBuf>,
        #[arg(long, default_value_t = 1)]
        scale: u32,
        #[arg(long, default_value_t = 0)]
        columns: u32,
    },
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Command::Check { input, json } => {
            let (src, name) = source::read_input(input.as_deref())?;
            match parse_and_validate(&src) {
                Ok(_) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&JsonDiagnostics {
                                valid: true,
                                errors: &[]
                            })?
                        );
                    }
                    Ok(())
                }
                Err(errors) => {
                    emit_errors(&errors, &name, &src, json)?;
                    std::process::exit(1);
                }
            }
        }
        Command::Render {
            input,
            output,
            frame,
            scale,
        } => {
            let (src, name) = source::read_input(input.as_deref())?;
            match parse_and_validate(&src) {
                Ok(doc) => {
                    let canvas = if let Some(frame) = frame {
                        render::render_frame(&doc, &frame)?
                    } else {
                        render::render(&doc)?
                    };
                    canvas.save_png(&output, scale)?;
                    Ok(())
                }
                Err(errors) => {
                    emit_errors(&errors, &name, &src, false)?;
                    std::process::exit(1);
                }
            }
        }
        Command::Fmt { input, write } => {
            let (src, name) = source::read_input(input.as_deref())?;
            match parser::parse(&src) {
                Ok(doc) => {
                    let formatted = fmt::format_document(&doc);
                    if write {
                        let Some(path) = input else {
                            return Err("fmt --write requires an input file".into());
                        };
                        fs::write(path, formatted)?;
                    } else {
                        print!("{formatted}");
                    }
                    Ok(())
                }
                Err(e) => {
                    emit_errors(&[e], &name, &src, false)?;
                    std::process::exit(1);
                }
            }
        }
        Command::Ast { input, json } => {
            let (src, name) = source::read_input(input.as_deref())?;
            match parser::parse(&src) {
                Ok(doc) => {
                    if json {
                        println!("{}", serde_json::to_string_pretty(&doc)?);
                    } else {
                        println!("{doc:#?}");
                    }
                    Ok(())
                }
                Err(e) => {
                    emit_errors(&[e], &name, &src, false)?;
                    std::process::exit(1);
                }
            }
        }
        Command::Spec { json } => {
            if json {
                println!("{}", serde_json::to_string_pretty(&spec::json_spec())?);
            } else {
                print!("{}", spec::SPEC);
            }
            Ok(())
        }
        Command::Anim {
            input,
            animation,
            out_dir,
            scale,
        } => {
            let (src, name) = source::read_input(input.as_deref())?;
            match parse_and_validate(&src) {
                Ok(doc) => {
                    fs::create_dir_all(&out_dir)?;
                    let (_fps, frames) = render::render_animation(&doc, &animation)?;
                    for (i, (frame_name, canvas)) in frames.iter().enumerate() {
                        let path = out_dir.join(format!("{animation}-{i:03}-{frame_name}.png"));
                        canvas.save_png(&path, scale)?;
                    }
                    Ok(())
                }
                Err(errors) => {
                    emit_errors(&errors, &name, &src, false)?;
                    std::process::exit(1);
                }
            }
        }
        Command::Sheet {
            input,
            animation,
            output,
            json,
            scale,
            columns,
        } => {
            let (src, name) = source::read_input(input.as_deref())?;
            match parse_and_validate(&src) {
                Ok(doc) => {
                    let (fps, frames) = render::render_animation(&doc, &animation)?;
                    let columns = if columns == 0 {
                        frames.len() as u32
                    } else {
                        columns
                    };
                    let metadata =
                        render::save_spritesheet(&frames, &output, scale, columns, Some(fps))?;
                    if let Some(path) = json {
                        fs::write(path, serde_json::to_string_pretty(&metadata)?)?;
                    }
                    Ok(())
                }
                Err(errors) => {
                    emit_errors(&errors, &name, &src, false)?;
                    std::process::exit(1);
                }
            }
        }
    }
}

fn parse_and_validate(src: &str) -> Result<validate::ValidDocument, Vec<Diagnostic>> {
    let doc = parser::parse(src).map_err(|e| vec![e])?;
    validate::validate(doc)
}

fn emit_errors(
    errors: &[Diagnostic],
    filename: &str,
    src: &str,
    json: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&JsonDiagnostics {
                valid: false,
                errors
            })?
        );
    } else {
        for e in errors {
            eprintln!("{}", e.render_human(filename, src));
        }
    }
    Ok(())
}
