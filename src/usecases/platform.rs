use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    Amd64,
    Arm,
    Arm64,
    I386,
    Riscv64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OS {
    Android,
    Darwin,
    Linux,
    Windows,
    Freebsd,
}

impl FromStr for Architecture {
    type Err = PlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "amd64" | "x86_64" => Ok(Architecture::Amd64),
            "arm" => Ok(Architecture::Arm),
            "arm64" | "aarch64" => Ok(Architecture::Arm64),
            "386" | "i386" | "i686" => Ok(Architecture::I386),
            "riscv64" => Ok(Architecture::Riscv64),
            _ => Err(PlatformError::UnsupportedArchitecture(s.to_string())),
        }
    }
}

impl FromStr for OS {
    type Err = PlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "android" => Ok(OS::Android),
            "darwin" => Ok(OS::Darwin),
            "linux" => Ok(OS::Linux),
            "windows" => Ok(OS::Windows),
            "freebsd" => Ok(OS::Freebsd),
            _ => Err(PlatformError::UnsupportedOs(s.to_string())),
        }
    }
}

impl ToString for Architecture {
    fn to_string(&self) -> String {
        match self {
            Architecture::Amd64 => "amd64".to_string(),
            Architecture::Arm => "arm".to_string(),
            Architecture::Arm64 => "arm64".to_string(),
            Architecture::I386 => "386".to_string(),
            Architecture::Riscv64 => "riscv64".to_string(),
        }
    }
}

impl ToString for OS {
    fn to_string(&self) -> String {
        match self {
            OS::Android => "android".to_string(),
            OS::Darwin => "darwin".to_string(),
            OS::Linux => "linux".to_string(),
            OS::Windows => "windows".to_string(),
            OS::Freebsd => "freebsd".to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("Unsupported architecture: {0}")]
    UnsupportedArchitecture(String),
    #[error("Unsupported OS: {0}")]
    UnsupportedOs(String),
}

pub struct Platform {
    pub architecture: Architecture,
    pub os: OS,
}

impl Platform {
    pub fn new() -> Result<Self, PlatformError> {
        let arch = std::env::consts::ARCH;
        let os = std::env::consts::OS;

        let architecture = Architecture::from_str(arch)?;
        let os = if os == "linux" && cfg!(target_os = "android") {
            OS::Android
        } else {
            OS::from_str(os)?
        };

        Ok(Self { architecture, os })
    }

    pub fn from_parts(arch: &str, os: &str) -> Result<Self, PlatformError> {
        let architecture = Architecture::from_str(arch)?;
        let os = OS::from_str(os)?;
        Ok(Self { architecture, os })
    }
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        format!("Platform(OS: {}, Architecture: {})", self.os.to_string(), self.architecture.to_string())
    }
}
