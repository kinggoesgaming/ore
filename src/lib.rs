//! A library for interacting with [Ore], the plugin repository for [Sponge] minecraft modding
//! platform.
//!
//! [Ore]: https://ore.spongepowered.org
//! [Sponge]: https://www.spongepowered.org

/// The API for version 1 of the Ore API.
pub mod v1;

pub struct Err<'a> {
    error: &'a str
}

impl<'a> Err<'a> {
    pub fn get_error(&self) -> &'a str {
        self.error
    }
}
