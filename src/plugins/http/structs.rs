
pub struct HttpDestination {
  pub url: String,
  pub method: String,
  pub headers: Option<std::collections::HashMap<String, String>>,
}