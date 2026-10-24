//! Help → project links (Livi's website, this app's page and its source repository). The commands return the URL; the frontend opens it.

use serde_json::{Value, json};

use super::*;

/// The app's id.
pub const APP_ID: &str = "livipen";
pub const WEBSITE_URL: &str = "https://liviverse.com";
/// Livi Pen's source repository.
pub const SOURCE_URL: &str = "https://github.com/sarayunova/livi-vector";

/// This app's page on the website.
pub fn app_page_url() -> String {
    WEBSITE_URL.into()
}

/// This app's source repository.
pub fn github_url() -> String {
    SOURCE_URL.into()
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.website", "Livi Website", ["Help"], None, "{} → {url}", always, |_, _| url(WEBSITE_URL.into())),
        cmd!(query "help.appPage", "Livi Pen on liviverse.com", ["Help"], None, "{} → {url} this app's page", always, |_, _| url(app_page_url())),
        cmd!(query "help.github", "Livi Pen on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {website, appPage, github}", always, |_, _| {
            Ok(json!({ "website": WEBSITE_URL, "appPage": app_page_url(), "github": github_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert!(s.execute("help.discord", &json!({})).is_err());
        assert_eq!(s.execute("help.appPage", &json!({})).unwrap()["url"], "https://liviverse.com");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/sarayunova/livi-vector");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["website"], "https://liviverse.com");
    }
}
