//! Workspace and project management
//! 
//! Handles solutions, projects, and multi-file compilations.

use std::path::PathBuf;
use crate::compilation::Compilation;
use crate::CompilerOptions;

/// Represents a C# solution
pub struct Solution {
    pub path: PathBuf,
    pub projects: Vec<Project>,
}

/// Represents a C# project
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub sources: Vec<PathBuf>,
    pub references: Vec<ProjectReference>,
    pub options: CompilerOptions,
}

/// A reference to another project or assembly
pub enum ProjectReference {
    Project(String),
    Assembly(PathBuf),
    Package(String, String), // name, version
}

impl Solution {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            projects: Vec::new(),
        }
    }
    
    pub fn load_from_file(path: PathBuf) -> Result<Self, std::io::Error> {
        // TODO: Parse .sln file
        todo!("Solution file parsing")
    }
}

impl Project {
    pub fn new(name: String, path: PathBuf, options: CompilerOptions) -> Self {
        Self {
            name,
            path,
            sources: Vec::new(),
            references: Vec::new(),
            options,
        }
    }
    
    pub fn load_from_file(path: PathBuf) -> Result<Self, std::io::Error> {
        // TODO: Parse .csproj file
        todo!("Project file parsing")
    }
    
    pub fn create_compilation(&self) -> Result<Compilation, std::io::Error> {
        let mut compilation = Compilation::new(self.options.clone());
        
        // Load all source files
        for source_path in &self.sources {
            let content = std::fs::read_to_string(source_path)?;
            compilation.add_source(
                source_path.to_string_lossy().into_owned(),
                content
            );
        }
        
        Ok(compilation)
    }
}