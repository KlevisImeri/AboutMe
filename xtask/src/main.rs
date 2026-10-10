use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const ADDR: &str = "127.0.0.1:8080";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let commands: Vec<String> = std::env::args().skip(1).collect();
    if commands.is_empty() {
        println!(
            "cargo xtask <command>...\n\n\
             commands:\n\
             \x20 build   build the static site into dist/\n\
             \x20 check   run fmt and clippy\n\
             \x20 dev     run the Dioxus dev server\n\
             \x20 serve   serve dist/ at http://{ADDR}\n\n\
             commands can be chained, e.g. `cargo xtask build serve`"
        );
        return Ok(());
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("workspace directory not found")?;

    for command in commands {
        match command.as_str() {
            "build" => build(root)?,
            "check" => check(root)?,
            "dev" => dev(root)?,
            "serve" => serve(root)?,
            other => return Err(format!("unknown task: {other}").into()),
        }
    }
    Ok(())
}

fn dioxus_cli() -> PathBuf {
    // Cargo's bin directory may not yet be on PATH after a first installation.
    let cargo_home = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cargo")));
    if let Some(home) = cargo_home {
        let dx = home
            .join("bin")
            .join(if cfg!(windows) { "dx.exe" } else { "dx" });
        if dx.is_file() {
            return dx;
        }
    }
    "dx".into()
}

fn dev(root: &Path) -> Result<()> {
    let status = Command::new(dioxus_cli())
        .current_dir(root)
        .args(["serve", "--web"])
        .stdin(Stdio::null())
        .status()?;
    if !status.success() {
        return Err("Dioxus development server failed".into());
    }
    Ok(())
}

fn build(root: &Path) -> Result<()> {
    let public = root.join("target/dx/about-me/release/web/public");
    // DX does not clean its output between builds, so stale hashed assets accumulate.
    if public.exists() {
        fs::remove_dir_all(&public)?;
    }

    // DX owns Tailwind compilation, asset processing, CNAME/public copying, and SSG.
    let status = Command::new(dioxus_cli())
        .current_dir(root)
        .args([
            "build",
            "--web",
            "--release",
            "--ssg",
            "--force-sequential=true",
            "--cargo-args=--locked",
        ])
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .status()?;
    if !status.success() {
        return Err("Dioxus build failed; the previous dist was preserved".into());
    }

    for page in ["index.html", "cooking/index.html"] {
        if !public.join(page).is_file() {
            return Err(format!("Dioxus did not generate {page}; dist was preserved").into());
        }
    }

    let dist = root.join("dist");
    if dist.exists() {
        fs::remove_dir_all(&dist)?;
    }
    dircpy::copy_dir(&public, &dist)?;

    println!("Generated dist/index.html and dist/cooking/index.html");
    Ok(())
}

fn check(root: &Path) -> Result<()> {
    run_cargo(root, &["fmt", "--all", "--check"])?;
    run_cargo(
        root,
        &["clippy", "--workspace", "--all-targets", "--locked"],
    )?;
    Ok(())
}

fn serve(root: &Path) -> Result<()> {
    let dist = root.join("dist");
    if !dist.is_dir() {
        return Err("dist/ not found; run `cargo xtask build` first".into());
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let app =
                axum::Router::new().fallback_service(tower_http::services::ServeDir::new(&dist));
            let listener = tokio::net::TcpListener::bind(ADDR).await?;
            println!(
                "Serving {} at http://{ADDR}/ (Ctrl+C to stop)",
                dist.display()
            );
            axum::serve(listener, app).await?;
            Ok::<(), std::io::Error>(())
        })?;

    Ok(())
}

fn run_cargo(root: &Path, args: &[&str]) -> Result<()> {
    // Prefer the cargo that invoked this task.
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let status = Command::new(cargo)
        .current_dir(root)
        .args(args)
        .stdin(Stdio::null())
        .status()?;
    if !status.success() {
        return Err(format!("cargo {} failed", args.join(" ")).into());
    }
    Ok(())
}
