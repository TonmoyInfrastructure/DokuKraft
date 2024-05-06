use std::path::PathBuf; 
/* 
* Documentation : https://doc.rust-lang.org/std/path/struct.PathBuf.html
* An owned, mutable path (akin to String).
* This type provides methods like push and set_extension that mutate the path in place.
* It also implements Deref to Path, meaning that all methods on Path slices are available 
* on PathBuf values as well.
*/
use crate::config::Config;
/* 
* Documentation : https://docs.rs/config/0.14.0/config/
* Config organizes hierarchical or layered configurations for Rust applications.
* Config lets you set a set of default parameters and then extend them via merging in 
* configuration from a variety of sources:
* - Environment variables
* - String literals in well-known formats
* - Another Config instance
* - Files: TOML, JSON, YAML, INI, RON, JSON5 and custom ones defined with Format trait
* - Manual, programmatic override (via a .set method on the Config instance)
*/
use super::DokuKraft;
/* 
* Documentation : https://doc.rust-lang.org/std/keyword.super.html
* The super and self keywords can be used in the path to remove ambiguity when accessing 
* items and to prevent unnecessary hardcoding of paths.
* More : https://doc.rust-lang.org/rust-by-example/mod/super.html 
*/
use crate::anyhow;
/* 
* Documentation : https://docs.rs/anyhow/1.0.83/anyhow/
* This library provides anyhow::Error, a trait object based error type for 
* easy idiomatic error handling in Rust applications.
*/
use std::fs::{self, File};
/* 
* Documentation : https://doc.rust-lang.org/std/fs/struct.File.html
* An object providing access to an open file on the filesystem.
*/


pub struct DocGenerator{
    root: PathBuf, // Provided by the Rust standard library for handling file system paths.
    generate_gitign: bool, // A boolean value indicating whether to generate a .gitignore file.
    config: Config, // Configuration data related to the generation process.
    duplicate_theme: bool, // Indicates whether to duplicate a theme or not.
}

// Defines the implementation block (impl) for the DocGenerator struct. 
impl DocGenerator {
    // It serves as a constructor method to create a new instance of DocGenerator.
    pub fn new<P: Into<PathBuf>>(root: P) -> DocGenerator {
        // Initializes a new DocGenerator instance.
        DocGenerator {
            root: root.into(), // initializes the root field with the value passed to the new function.
            generate_gitign: false, // Won't create .gitignore file.
            config: Config::default(), // Initializes the config field with the default value of Config.
            duplicate_theme: false, // Theme will not be duplicated.
        }
    }
    // Generate .gitignore File.
    pub fn generate_gitign(&mut self, generate: bool) -> &mut DocGenerator {
        self.generate_gitign = generate; // Assign parameters , allowing mod struct!
        self // Returns a mutable reference to self.
    }

    // Takes mutable reference to self, allowing it to modify the DocGenerator instance it's called on.
    pub fn wconfig(&mut self, cfg: Config) -> &mut DocGenerator {
        self.config = cfg; // Assigns the cfg parameter to the config field of the DocGenerator instance.
        self // Returns a mutable reference to self.
    }

    // &self : The method takes an immutable reference to self, meaning it doesn't modify the DocGenerator instance.
    // &Config : Return type of the method as a reference to Config.
    pub fn config(&self) -> &Config {
        &self.config // Returns a reference to the config field of the DocGenerator instance.
    }

    // Defines method duplicate_theme for the DocGenerator struct.
    pub fn duplicate_theme(&mut self, duplicate: bool) -> &mut DocGenerator {
        self.duplicate_theme = duplicate;
        self
    }

    pub fn build(&self) -> Result<DokuKraft> {
        info!(">> Initiating A New DokuKraft With Stub...");
        self.make_dir_struct().context("Failed To Create Directory Strcture!")?;
        // It calls context on the result of make_dir_struct and chains it with the ? operator to propagate errors if any.
        self.gen_stub_files().context("Failed To Create Stub Files!")?;
        if self.generate_gitign {
            self.gen_gitign().context("Failed To Create .gitignore!")
        }
    }

    fn gen_gitign(&self) -> Result<()> {
        debug!("Generating .gitignore");
        let mut f = File::create(self.root.join(".gitignore"))?;
        writeln!(f, "{}", self.config.build.build_dir.display())?;
        OK(())
    }

    fn dup_theme(&self) -> Result<()> {
        debug!("Duplicating Theme");
        let html_config = self.config.html_config().unwrap_or_default();
        let themedir = html_config.theme_dir(&self.root);
        if !themedir.exists(){
            println!("{} does not exist, creating directory", themedir.display());
            fs::create_dir(&themedir)?;
        }
        let mut index = File::create(themedir.join("index.hbs"))?;
        index.write_all(theme::index)?;
        let stydir = themedir.join("css");
        if !stydir.exists() {
            fs::create_dir(&stydir)?;
        }
        let mut common_css = File::create(stydir.join("common.css"))?;
        common_css.write_all(theme::COMMON_CSS)?;
        let mut chrome_css = File::create(stydir.join("chrome.css"))?;
        chrome_css.write_all(theme::CHROME_CSS)?;
        if html_config.print.enable {
            let mut print_css = File::create(stydir.join("print.css"))?;
            print_css.write_all(theme::PRINT_CSS)?;
        }
        let mut variables_css = File::create(stydir.join("variables.css"))?;
        variables_css.write_all(theme::VARIABLES_CSS)?;
        let mut favicon = File::create(themedir.join("favicon.png"))?;
        favicon.write_all(theme::FAVICON_PNG)?;
        let mut favicon = File::create(themedir.join("favicon.svg"))?;
        favicon.write_all(theme::FAVICON_SVG)?;
    }

}