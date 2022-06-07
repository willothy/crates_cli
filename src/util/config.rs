use cargo_toml::{self, Dependency, Manifest};
use std::error::Error;
use std::path::PathBuf;

pub fn load_current_crate_config() -> Result<cargo_toml::Manifest, Box<dyn std::error::Error>> {
    Ok(Manifest::from_path(
        get_current_crate_root()?.join("Cargo.toml"),
    )?)
}

pub fn get_current_crate_root() -> Result<PathBuf, Box<dyn Error>> {
    find_crate_root(std::env::current_dir()?)
}

pub(self) fn find_crate_root(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let mut has_target_dir = false;
    let mut has_cargo_toml = false;

    for entry in std::fs::read_dir(&path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.ends_with("Cargo.toml") {
            has_cargo_toml = true;
        }
        if entry_path.ends_with("target") && entry_path.is_dir() {
            has_target_dir = true;
        }
        if has_cargo_toml && has_target_dir {
            return Ok(path);
        }
    }

    if let Some(parent) = path.parent() {
        find_crate_root(parent.to_path_buf())
    } else {
        Err("Could not find crate root".into())
    }
}

pub trait CargoToml {
    fn has_dependency(&self, name: &str) -> bool;
    fn get_dependency(&self, name: &str) -> Option<(&String, &Dependency)>;

    fn add_dependency<S: Into<String>>(
        &mut self,
        name: &str,
        dependency: S,
    ) -> Result<(), Box<dyn Error>>;

    fn write(&self, path: &PathBuf) -> Result<(), Box<dyn Error>>;
}

impl CargoToml for Manifest {
    fn has_dependency(&self, name: &str) -> bool {
        self.dependencies.iter().any(|(dep, _)| dep == &name)
    }

    fn get_dependency(&self, name: &str) -> Option<(&String, &Dependency)> {
        self.dependencies.iter().find(|(dep, _)| dep == &name)
    }

    fn add_dependency<S: Into<String>>(
        &mut self,
        name: &str,
        dependency: S,
    ) -> Result<(), Box<dyn Error>> {
        if self.has_dependency(name) {
            return Err("Dependency already exists".into());
        }
        self.dependencies
            .insert(name.to_owned(), Dependency::Simple(dependency.into()));
        Ok(())
    }

    fn write(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let s = toml::to_string_pretty(self)?;
        std::fs::write(path, s)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_current_crate_test() -> Result<(), Box<dyn Error>> {
        let s = toml::to_string(&super::load_current_crate_config()?)?;
        println!("{}", s);
        Ok(())
    }

    #[test]
    fn find_crate_root_test() -> Result<(), Box<dyn Error>> {
        let s = super::find_crate_root(std::env::current_dir()?)?;
        println!("{:?}", s);
        Ok(())
    }

    /*#[test]
    fn write_test() -> Result<(), Box<dyn Error>> {
        let root = super::find_crate_root(PathBuf::from("/* test dir here */"))?.join("Cargo.toml");
        let mut manifest = Manifest::from_path(&root)?;
        manifest.add_dependency("serde", "*".to_owned())?;
        manifest.write(&root)?;
        Ok(())
    }*/
}
