use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rusty_audio::Audio;

pub struct Sound {
    audio: Audio,
    files: HashMap<String, PathBuf>,
}

impl Sound {
    pub fn new() -> Self {
        Self {
            audio: Audio::new(),
            files: HashMap::new(),
        }
    }

    pub fn add<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let filename = path
            .as_ref()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        self.audio.add(&filename, path.as_ref());
        self.files.insert(filename, path.as_ref().to_path_buf());

        Ok(())
    }

    pub fn play(&mut self, name: &str) {
        match self.play_safe(name) {
            Ok(_) => (),
            Err(_) => {}
        }
    }

    pub fn play_safe(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // check if we exist in the hashmap
        if !self.files.contains_key(name) {
            return Err("Sound not found".into());
        }

        self.audio.play(name);
        Ok(())
    }
}
