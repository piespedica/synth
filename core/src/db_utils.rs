use uriparse::URI;

pub struct DataSourceParams<'a> {
    pub uri: URI<'a>,
    pub schema: Option<String>, // PostgreSQL
    pub timeout_ms: Option<u64>, // Connection timeout in milliseconds
}
