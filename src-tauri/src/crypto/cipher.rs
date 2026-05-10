// Client-side encryption is OPTIONAL in MVP.
// Default: B2 server-side encryption is sufficient.
// This module provides a future hook for E2E encryption.

pub struct Cipher;

impl Cipher {
    pub fn new() -> Self {
        Cipher
    }

    pub fn is_enabled(&self) -> bool {
        false // Disabled for MVP
    }
}
