use glob::glob;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

/// Contains the structure of resulting rust-project.json file
/// and functions to build the data required to create the file
#[derive(Serialize, Deserialize)]
pub struct RustAnalyzerProject {
    sysroot_src: String,

    #[serde(skip)]
    cargo_tokio: String,
    pub crates: Vec<Crate>,
}

#[derive(Serialize, Deserialize)]
pub struct Crate {
    root_module: String,
    edition: String,
    deps: Vec<DepData>,
    cfg: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DepData {
    #[serde(rename="crate")]
    crate_index: i32,
    name: String,
}

impl RustAnalyzerProject {
    pub fn new() -> RustAnalyzerProject {
        RustAnalyzerProject {
            sysroot_src: String::new(),
            cargo_tokio: String::new(),
            crates: Vec::new(),
        }
    }

    /// Write rust-project.json to disk
    pub fn write_to_disk(&self) -> Result<(), std::io::Error> {
        std::fs::write(
            "./rust-project.json",
            serde_json::to_vec(&self).expect("Failed to serialize to JSON"),
        )?;
        Ok(())
    }

    /// If path contains .rs extension, add a crate to `rust-project.json`
    fn path_to_json(&mut self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                let mut c = Crate {
                    root_module: path.display().to_string(),
                    edition: "2021".to_string(),
                    deps: Vec::new(),
                    // This allows rust_analyzer to work inside #[test] blocks
                    cfg: vec!["test".to_string()],
                };
                if path.display().to_string().starts_with("exercises/async") {
                    c.deps = vec!(DepData{ crate_index: 0, name: "tokio".to_string()})
                }
                self.crates.push(c);
            }
        }

        Ok(())
    }

    fn add_tokio_to_crates(&mut self) {
        self.crates.push(Crate {
            root_module: self.cargo_tokio.to_string(),
            edition: "2021".to_string(),
            deps: Vec::new(),
            // This allows rust_analyzer to work inside #[test] blocks
            cfg: vec![
                "feature=\"fs\"".to_string(),
                "feature=\"io-util\"".to_string(),
                "feature=\"io-std\"".to_string(),
                "feature=\"macros\"".to_string(),
                "feature=\"net\"".to_string(),
                "feature=\"parking_lot\"".to_string(),
                "feature=\"process\"".to_string(),
                "feature=\"rt\"".to_string(),
                "feature=\"rt-multi-thread\"".to_string(),
                "feature=\"signal\"".to_string(),
                "feature=\"sync\"".to_string(),
                "feature=\"time\"".to_string(),
            ],
        });
    }

    /// Parse the exercises folder for .rs files, any matches will create
    /// a new `crate` in rust-project.json which allows rust-analyzer to
    /// treat it like a normal binary
    pub fn exercises_to_json(&mut self) -> Result<(), Box<dyn Error>> {
        self.add_tokio_to_crates();
        for path in glob("./exercises/**/*")? {
            self.path_to_json(path?)?;
        }
        Ok(())
    }

    /// Use `rustc` to determine the default toolchain
    pub fn get_sysroot_src(&mut self) -> Result<(), Box<dyn Error>> {
        // check if RUST_SRC_PATH is set
        if let Ok(path) = env::var("RUST_SRC_PATH") {
            self.sysroot_src = path;
            return Ok(());
        }

        let toolchain = Command::new("rustc")
            .arg("--print")
            .arg("sysroot")
            .output()?
            .stdout;

        let toolchain = String::from_utf8_lossy(&toolchain);
        let mut whitespace_iter = toolchain.split_whitespace();

        let toolchain = whitespace_iter.next().unwrap_or(&toolchain);

        println!("Determined toolchain: {}\n", &toolchain);

        self.sysroot_src = (std::path::Path::new(&*toolchain)
            .join("lib")
            .join("rustlib")
            .join("src")
            .join("rust")
            .join("library")
            .to_string_lossy())
        .to_string();
        Ok(())
    }


    pub fn get_cargo_tokio_path(&mut self) {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("~/"));
        self.cargo_tokio = (std::path::Path::new(&home)
        .join(".cargo")
        .join("registry")
        .join("src")
        .join("github.com-1ecc6299db9ec823")
        .join("tokio-1.28.1")
        .join("src")
        .join("lib.rs")
        .to_string_lossy())
        .to_string();
    }

}
