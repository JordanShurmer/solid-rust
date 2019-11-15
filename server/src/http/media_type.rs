// example:  text/turle => ContentType{
//   category: "text",
//   format: "turtle",
// }
pub struct MediaType {
    pub category: String,
    pub format: String,
}

impl From<&str> for MediaType {
    fn from(string: &str) -> Self {
        let mut category = string.to_owned().clone();
        let slash_at = string.find("/").unwrap_or(string.len());
        let format = category
            .split_off(slash_at)
            .trim_start_matches("/")
            .to_owned();

        MediaType {
            category: category,
            format: format,
        }
    }
}

impl From<String> for MediaType {
    fn from(string: String) -> Self {
        string.as_str().into()
    }
}

impl MediaType {
    pub fn matches(&self, other: &str) -> bool {
        let other: MediaType = other.into();
        if other.category == "*" && other.format == "*" {
            return true;
        }

        if other.category == self.category {
            return other.format == "*" || other.format == "" || other.format == self.format;
        }

        return false;
    }
}

#[test]
fn basic_test() {
    let media_type: MediaType = "text/turtle".to_owned().into();
    assert_eq!(media_type.category, "text");
    assert_eq!(media_type.format, "turtle");
}

#[test]
fn non_hierarchical() {
    let media_type: MediaType = "foobar".to_owned().into();
    assert_eq!(media_type.category, "foobar");
    assert_eq!(media_type.format, "");
}

#[test]
fn matches_string() {
    let media_type: MediaType = "text/turtle".to_owned().into();
    assert_eq!(media_type.matches("text/turtle"), true);
    assert_eq!(media_type.matches("text/*"), true);
    assert_eq!(media_type.matches("text"), true);
    assert_eq!(media_type.matches("*/*"), true);
    assert_eq!(media_type.matches("text/html"), false);
    assert_eq!(media_type.matches("application/turtle"), false);
    assert_eq!(media_type.matches("application/*"), false);
}
