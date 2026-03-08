# Version Type Parameter Implementation

## Summary

Successfully implemented a version type parameter `V` for the dependency injection system in Rust.

## Changes Made

### 1. Added Version Trait (`src/interfaces.rs`)
```rust
pub trait Version: Send + Sync + Clone {
    fn get_version(&self) -> Option<&str>;
}
```

### 2. Updated DependencyInjection Struct (`src/di.rs`)
- Added `V` type parameter with default `DummyVersion`
- Added `version: V` field
- Created `with_parts()` constructor for custom implementations

### 3. Implementations

#### DummyVersion (Default)
```rust
pub struct DummyVersion;

impl Version for DummyVersion {
    fn get_version(&self) -> Option<&str> {
        None
    }
}
```

#### CustomVersion (With String Constructor)
```rust
pub struct CustomVersion {
    version: String,
}

impl CustomVersion {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}

impl Default for CustomVersion {
    fn default() -> Self {
        Self {
            version: env!("RUSTRUUT_VERSION").to_string(),
        }
    }
}

impl Version for CustomVersion {
    fn get_version(&self) -> Option<&str> {
        Some(&self.version)
    }
}
```

### 4. Updated All Dependent Structs
- `Phonemizer<P, I, D, A, F, V>`
- `PhonemizeUsecaseImpl<P, I, D, A, F, V>`
- `GoruutState<P, I, D, A, F, V>`
- `Goruut<P, I, D, A, F, V>`
- `Config<P, I, D, A, F, V>`

## Usage Examples

### Using Default (DummyVersion)
```rust
use rustruut::DependencyInjection;

let di: DependencyInjection = DependencyInjection::new();
// version.get_version() returns None
```

### Using CustomVersion with Specific Version
```rust
use rustruut::{DependencyInjection, di};

let di = DependencyInjection::with_parts(
    di::default_impls::DummyPolicy,
    di::default_impls::DummyIpaFlavor,
    di::default_impls::DummyDict,
    di::default_impls::DummyApi,
    di::default_impls::DummyFolder,
    di::custom_impls::CustomVersion::new("0.7.0"),
);
// version.get_version() returns Some("0.7.0")
```

### Using CustomVersion with Package Version
```rust
use rustruut::{DependencyInjection, di};

let di = DependencyInjection {
    policy: di::default_impls::DummyPolicy,
    ipa: di::default_impls::DummyIpaFlavor,
    dict_getter: di::default_impls::DummyDict,
    api: di::default_impls::DummyApi,
    folder: di::default_impls::DummyFolder,
    version: di::custom_impls::CustomVersion::default(), // Uses RUSTRUUT_VERSION
};
```

## Build Status

✅ All code compiles successfully
✅ Tests compile and run
✅ Examples compile and run
