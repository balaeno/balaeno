use std::{collections::HashMap, path::{Path, PathBuf}};
use anyhow::Error;

// store, load(json_dir: &Path), 
use std::fs;
use super::entry::{Entry};

struct Store {
    map: HashMap<String, dyn Entry>, 
    json_dir: PathBuf,
}

impl Store {
    pub fn new(json_dir: &Path) -> Self {
        Self {
            map: HashMap::new(),
            json_dir: json_dir.to_path_buf(),
        }
    }

    pub fn load(&mut self) -> Result<(), Error> {
        let json_files = fs::read_dir(self.json_dir)?;
        for json_file in json_files {
            let json_file = json_file?;
            let json_file_path = json_file.path();
            // TODO: raise error instead of using unwrap
            let container_id = json_file_path.file_name().unwrap().to_str().unwrap();
            let json = fs::read_to_string(json_file_path)?;
            let container: Entry = Entry::new(json); // Fix: Call Entry::new() instead of Entry::get()
            self.map.insert(container_id.to_string(), container);
        }
        Ok(())
    }

    
    pub fn persist(&mut self, name: String) -> Result<(), Error> {
        let entry = self.map.get(&name).ok_or_else(|| Error::msg("entry not found"))?;
        fs::write(self.json_dir.join(name), entry.get())?;
        Ok(())
    }

    pub fn set(&mut self, name: String, data: HashMap<String, >) -> Result<(), Error> {
        let entry = self.map.get_mut(&name).ok_or_else(|| Error::msg("entry not found"))?;
        entry.set(data);
        Ok(())
    }
}