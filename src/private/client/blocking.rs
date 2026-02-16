use std::sync::LazyLock;

pub static BLOCKING_CLIENT: LazyLock<reqwest::blocking::Client> = LazyLock::new(|| {
    reqwest::blocking::Client::builder()
        .timeout(None)
        .build()
        .expect("Failed to create HTTP client")
});

