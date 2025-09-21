use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

use super::platform::{Architecture, OS};

#[derive(Debug)]
pub struct Executable {
    pub size: u64,
    pub sha256: String,
    pub architecture: Architecture,
    pub os: OS,
    pub servers: Vec<String>,
}

#[derive(Error, Debug)]
pub enum ExecutableError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Verification error: {0}")]
    Verification(String),
    #[error("Download error: {0}")]
    Download(String),
}

impl Executable {
    pub fn file_name(&self) -> String {
        let arch = self.architecture.to_string();
        let os_ext = match self.os {
            OS::Windows => ".exe",
            OS::Linux => ".bin",
            OS::Darwin => ".dmg",
            _ => "",
        };
        format!(
            "goruut.{}.{}.{}{}",
            self.sha256,
            arch,
            self.os.to_string(),
            os_ext
        )
    }

    pub fn file_name_public(&self) -> String {
        let arch = self.architecture.to_string();
        format!("goruut-{}-{}", self.os.to_string(), arch)
    }

    pub fn exists(&self, temp_dir: &Path) -> Result<PathBuf, ExecutableError> {
        let temp_file_path = temp_dir.join(self.file_name());

        if !temp_file_path.exists() {
            return Err(ExecutableError::Verification(
                "File does not exist".to_string(),
            ));
        }

        // Verify file size
        let metadata = fs::metadata(&temp_file_path)?;
        if metadata.len() != self.size {
            return Err(ExecutableError::Verification(
                "File size mismatch".to_string(),
            ));
        }

        // Verify SHA256
        let mut file = File::open(&temp_file_path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 4096];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let hash = format!("{:x}", hasher.finalize());
        if hash != self.sha256 {
            return Err(ExecutableError::Verification("SHA256 mismatch".to_string()));
        }

        Ok(temp_file_path)
    }

    pub fn download(&self, temp_dir: &Path) -> Result<PathBuf, ExecutableError> {
        let temp_file_path = temp_dir.join(self.file_name());
        let mut last_error = None;

        for url_prefix in &self.servers {
            let url = format!("{}{}", url_prefix, self.file_name_public());

            let response = match reqwest::blocking::get(&url) {
                Ok(resp) => resp,
                Err(e) => {
                    last_error = Some(e.to_string());
                    continue;
                }
            };

            if !response.status().is_success() {
                last_error = Some(format!("HTTP error: {}", response.status()));
                continue;
            }

            let mut file = File::create(&temp_file_path)?;
            let content = response
                .bytes()
                .map_err(|e| ExecutableError::Download(e.to_string()))?;
            file.write_all(&content)?;

            // Verify the downloaded file
            match self.exists(temp_dir) {
                Ok(path) => {
                    // Make executable
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let mut perms = fs::metadata(&path)?.permissions();
                        perms.set_mode(0o755);
                        fs::set_permissions(&path, perms)?;
                    }
                    return Ok(path);
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    fs::remove_file(&temp_file_path).ok(); // Ignore errors removing temp file
                }
            }
        }

        Err(ExecutableError::Download(last_error.unwrap_or_else(|| {
            "All download attempts failed".to_string()
        })))
    }
}
