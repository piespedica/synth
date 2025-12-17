use crate::cli::export::{create_and_insert_values, ExportStrategy};
use crate::cli::import::ImportStrategy;
use crate::cli::import_utils::build_namespace_import;
use crate::datasource::postgres_datasource::{PostgresConnectParams, PostgresDataSource};
use crate::datasource::DataSource;
use crate::sampler::SamplerOutput;
use anyhow::Result;
use synth_core::schema::Namespace;

#[derive(Clone, Debug)]
pub struct PostgresExportStrategy {
    pub uri_string: String,
    pub schema: Option<String>,
}

impl ExportStrategy for PostgresExportStrategy {
    fn export(&self, _namespace: Namespace, sample: SamplerOutput) -> Result<()> {
        let connect_params = PostgresConnectParams {
            uri: self.uri_string.clone(),
            schema: self.schema.clone(),
            max_connections: None, // Use default (3)
            timeout_ms: None, // Export doesn't use timeout from command line
        };

        let datasource = PostgresDataSource::new(&connect_params)?;

        create_and_insert_values(sample, &datasource)
    }
}

#[derive(Clone, Debug)]
pub struct PostgresImportStrategy {
    pub uri_string: String,
    pub schema: Option<String>,
    pub timeout_ms: Option<u64>,
}

impl ImportStrategy for PostgresImportStrategy {
    fn import(&self) -> Result<Namespace> {
        let connect_params = PostgresConnectParams {
            uri: self.uri_string.clone(),
            schema: self.schema.clone(),
            max_connections: None, // Use default (3)
            timeout_ms: self.timeout_ms,
        };

        let datasource = PostgresDataSource::new(&connect_params)?;

        build_namespace_import(&datasource)
    }
}
