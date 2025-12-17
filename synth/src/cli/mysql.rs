use crate::cli::export::{create_and_insert_values, ExportStrategy};
use crate::cli::import::ImportStrategy;
use crate::cli::import_utils::build_namespace_import;
use crate::datasource::mysql_datasource::{MySqlConnectParams, MySqlDataSource};
use crate::datasource::DataSource;
use crate::sampler::SamplerOutput;
use anyhow::Result;
use synth_core::schema::Namespace;

#[derive(Clone, Debug)]
pub struct MySqlExportStrategy {
    pub uri_string: String,
}

impl ExportStrategy for MySqlExportStrategy {
    fn export(&self, _namespace: Namespace, sample: SamplerOutput) -> Result<()> {
        let datasource = MySqlDataSource::new(&MySqlConnectParams {
            uri: self.uri_string.clone(),
            max_connections: None,
            timeout_ms: None, // Export doesn't use timeout from command line
        })?;

        create_and_insert_values(sample, &datasource)
    }
}

#[derive(Clone, Debug)]
pub struct MySqlImportStrategy {
    pub uri_string: String,
    pub timeout_ms: Option<u64>,
}

impl ImportStrategy for MySqlImportStrategy {
    fn import(&self) -> Result<Namespace> {
        let datasource = MySqlDataSource::new(&MySqlConnectParams {
            uri: self.uri_string.clone(),
            max_connections: None,
            timeout_ms: self.timeout_ms,
        })?;

        build_namespace_import(&datasource)
    }
}
