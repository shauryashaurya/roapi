use std::fs;
use std::sync::Arc;

use super::TableSource;

pub fn to_mem_table(t: &TableSource) -> anyhow::Result<datafusion::datasource::MemTable> {
    // TODO: read csv option from config
    let has_header = true;
    let delimiter = b',';
    let batch_size = 1024;
    let projection = None;

    let schema_ref: arrow::datatypes::SchemaRef = match &t.schema {
        Some(s) => Arc::new(s.into()),
        None => Arc::new(arrow::csv::reader::infer_schema_from_files(
            &[t.uri.clone()],
            delimiter,
            None,
            has_header,
        )?),
    };

    let csv_reader = arrow::csv::Reader::new(
        fs::File::open(&t.uri)?,
        schema_ref.clone(),
        has_header,
        Some(delimiter),
        batch_size,
        None,
        projection,
    );

    let batches = csv_reader
        .into_iter()
        .map(|batch| Ok(batch?))
        .collect::<anyhow::Result<Vec<arrow::record_batch::RecordBatch>>>()?;

    let partitions = vec![batches];
    Ok(datafusion::datasource::MemTable::try_new(
        schema_ref, partitions,
    )?)
}
