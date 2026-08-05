//! Platform utilities

pub fn open_url(url: &str) {
    let _ = open::that(url);
}