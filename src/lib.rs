// Re-export the types from type.rs
pub use crate::types::*;

// Include the types module
pub mod types {
    use serde::{Deserialize, Serialize};
    use std::default::Default;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PingWebhookParams {
        pub webhook_id: String,
    }

    impl Default for PingWebhookParams {
        fn default() -> Self {
            Self {
                webhook_id: String::new(),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PingWebhookResponse {
        pub error: Option<String>,
        pub status: String,
    }

    impl Default for PingWebhookResponse {
        fn default() -> Self {
            Self {
                error: None,
                status: String::new(),
            }
        }
    }
}

// Make sure tests can find the types
#[cfg(test)]
pub use self::types::*;
