use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use git2::{build::CheckoutBuilder, Repository};

use crate::{errors::Errors, GF_JSON};

pub struct Git {
    path: PathBuf,
}

impl Git {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// The repositry name is the name of the current directory if not provide
    pub fn get_local_repo(
        &mut self,
        name: Option<String>,
    ) -> Result<(String, PathBuf), Box<dyn Error>> {
        // If the current directory is a .git folder, get the root path of the repository
        let _ = Repository::open(&self.path)?;
        if self.path.ends_with(".git") {
            self.path = self
                .path
                .parent()
                .expect("Can't find the repository root path.")
                .canonicalize()?;
        }
        let name =
            name.unwrap_or_else(|| self.path.file_name().unwrap().to_str().unwrap().to_string());

        Ok((name, self.path.clone()))
    }

    /// Ensure the {GF_JSON} are in the .gitignore file
    pub fn ignore_config(&self) -> Result<(), Box<dyn Error>> {
        // Add {GF_JSON} to .gitignore if not already present
        let gitignore_path = self.path.join(".gitignore");

        // Check if .gitignore exists and add {GF_JSON} if not already present
        if gitignore_path.exists() {
            let mut gitignore_content = String::new();
            let mut file = File::open(&gitignore_path)?;
            file.read_to_string(&mut gitignore_content)?;

            if !gitignore_content.lines().any(|line| line.trim() == GF_JSON) {
                let mut gitignore = OpenOptions::new().append(true).open(&gitignore_path)?;
                writeln!(gitignore, "{GF_JSON}")?;
                println!("[INFO] {GF_JSON} to .gitignore file.");
            } else {
                println!("[INFO] {GF_JSON} already exists in .gitignore file.");
            }
        } else {
            let mut gitignore = File::create(&gitignore_path)?;
            writeln!(gitignore, "{GF_JSON}")?;
            println!("[INFO] Created .gitignore file and added {GF_JSON}.");
        }

        Ok(())
    }

    pub fn restore_workdir(&self) -> Result<(), git2::Error> {
        // Open the repository
        let repo = Repository::open(&self.path)?;

        // Configure the checkout options to force the checkout
        let mut checkout_options = CheckoutBuilder::new();
        checkout_options.force();

        // Execute the checkout to restore the workdir
        repo.checkout_head(Some(&mut checkout_options))?;

        Ok(())
    }

    pub fn get_last_commit(&self) -> Result<String, git2::Error> {
        // Open the repository
        let repo = Repository::open(&self.path)?;

        // Get the last commit hash
        let branch = repo
            .find_branch("main", git2::BranchType::Local)
            .expect(Errors::MainBranchNotFound.to_string().as_str());
        let last_commit = branch.get().peel_to_commit()?.id().to_string();

        Ok(last_commit)
    }
}
