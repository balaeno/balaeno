use std::{collections::HashMap, ops::Deref, path::{Path, PathBuf}, sync::{Arc, RwLock}};
use anyhow::{anyhow, Error};
use serde::{de::DeserializeOwned, Serialize};
use crate::error::{ErrorType, RuntimeError};

// store, load(json_dir: &Path), 
use std::fs;

struct Store<V: Serialize + DeserializeOwned> {
    map: HashMap<String, Arc<RwLock<V>>>, 
    json_dir: PathBuf,
}

impl<V> Store<V> where V: Serialize + DeserializeOwned {
    pub fn new(json_dir: &Path) -> Self {
        Self {
            map: HashMap::new(),
            json_dir: json_dir.to_path_buf(),
        }
    }

    pub fn load(&mut self) -> Result<(), Error> {
        let json_files = fs::read_dir(self.json_dir)?;
        for json_file in json_files {
            let json_file = match json_file {
                Ok(json_file) => json_file,
                Err(err) => {
                    return Err(anyhow!("{}", RuntimeError {
                        message: err.to_string(),
                        error_type: ErrorType::Runtime
                    }));
                }
            };
            let json_file_path = json_file.path();
            // TODO: raise error instead of using unwrap
            let container_id = match json_file_path.file_name().and_then(|name| name.to_str()) {
                Some(id) => id,
                None => {
                    return Err(anyhow!("{}", RuntimeError {
                        message: "Failed to extract container ID".to_string(),
                        error_type: ErrorType::Runtime
                    }));
                }
            };
            let json = match fs::read_to_string(json_file_path) {
                Ok(json) => json,
                Err(err) => {
                    return Err(anyhow!("{}", RuntimeError {
                        message: err.to_string(),
                        error_type: ErrorType::Runtime
                    }));
                }
            
            };
            let serde_json = match serde_json::from_str(&json) {
                Ok(serde_json) => serde_json,
                Err(err) => {
                    return Err(anyhow!("{}", RuntimeError {
                        message: err.to_string(),
                        error_type: ErrorType::Runtime
                    }));
                }
            };
            let container: Arc<RwLock<V>> = Arc::new(RwLock::new(serde_json));
            self.map.insert(container_id.to_string(), container);
        }
        Ok(())
    }

    pub fn persist(&mut self, name: String) -> Result<(), Error> {
        let entry = self.map.get(&name).ok_or_else(|| anyhow!("{}", RuntimeError {
            message: "Entry not found".to_string(),
            error_type: ErrorType::Runtime
        }))?;
        entry.clear_poison();
        let unlock_entry = entry.read().map_err(|err| anyhow!("{}", RuntimeError {
            message: err.to_string(),
            error_type: ErrorType::Runtime
        }))?;
        fs::write(self.json_dir.join(name), serde_json::to_string(unlock_entry.deref())?)
            .map_err(|err| anyhow!("{}", RuntimeError {
                message: err.to_string(),
                error_type: ErrorType::Runtime
            }))?;
        Ok(())
    }

    pub fn set(&mut self, name: String, data: V) -> Result<(), Error> {
        self.map.insert(name, Arc::new(RwLock::new(data)));
        self.persist(name);
        Ok(())
    }
}
