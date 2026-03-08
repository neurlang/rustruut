# Custom Implementations for Dependency Injection

## Summary

Successfully implemented custom implementations with string constructors for Version and Api in the dependency injection system.

## Available Custom Implementations

### 1. CustomVersion
A version provider that accepts a string in the constructor.

```rust
use rustruut::{DependencyInjection, di};

// Using CustomVersion with a specific version
let di = DependencyInjection {
    policy: di::default_impls::DummyPolicy,
    ipa: di::default_impls::DummyIpaFlavor,
    dict_getter: di::default_impls::DummyDict,
    api: di::default_impls::DummyApi,
    folder: di::default_impls::DummyFolder,
    version: di::custom_impls::CustomVersion::new("v0.7.0"),
};
```

**Implementation:**
```rust
pub struct CustomVersion {
    version: String,
}

impl CustomVersion {
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
        }
    }
}

impl Version for CustomVersion {
    fn get_version(&self) -> Option<&str> {
        Some(&self.version)
    }
}
```

### 2. CustomApi
An API path provider that accepts a string in the constructor.

```rust
use rustruut::{DependencyInjection, di};

// Using CustomApi with a specific API endpoint
let di = DependencyInjection {
    policy: di::default_impls::DummyPolicy,
    ipa: di::default_impls::DummyIpaFlavor,
    dict_getter: di::default_impls::DummyDict,
    api: di::custom_impls::CustomApi::new("http://localhost:8080"),
    folder: di::default_impls::DummyFolder,
    version: di::default_impls::DummyVersion,
};
```

**Implementation:**
```rust
pub struct CustomApi {
    api_path: String,
}

impl CustomApi {
    pub fn new(api_path: &str) -> Self {
        Self {
            api_path: api_path.to_string(),
        }
    }
}

impl Api for CustomApi {
    fn get_api_path(&self) -> &str {
        &self.api_path
    }
}
```

## Combined Usage

You can use both custom implementations together:

```rust
use rustruut::{DependencyInjection, di};

let di = DependencyInjection::with_parts(
    di::default_impls::DummyPolicy,
    di::default_impls::DummyIpaFlavor,
    di::default_impls::DummyDict,
    di::custom_impls::CustomApi::new("http://api.example.com"),
    di::default_impls::DummyFolder,
    di::custom_impls::CustomVersion::new("v0.7.0"),
);

let phonemizer = Phonemizer::new(di);
```

## Default Implementations

For comparison, here are the default implementations:

### DummyVersion
```rust
pub struct DummyVersion;

impl Version for DummyVersion {
    fn get_version(&self) -> Option<&str> {
        None  // Returns no version
    }
}
```

### DummyApi
```rust
pub struct DummyApi;

impl Api for DummyApi {
    fn get_api_path(&self) -> &str {
        ""  // Returns empty string
    }
}
```

## Build Status

✅ All code compiles successfully
✅ Tests compile and run
✅ Examples compile and run
✅ CustomVersion::new("version") implemented
✅ CustomApi::new("api_path") implemented
