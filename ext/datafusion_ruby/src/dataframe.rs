use datafusion::dataframe::DataFrame;
use magnus::Error;
use std::sync::Arc;

use crate::errors::DataFusionError;
use crate::record_batch::RbRecordBatch;
use crate::utils::wait_for_future;

#[magnus::wrap(class = "Datafusion::DataFrame")]
pub(crate) struct RbDataFrame {
    df: Arc<DataFrame>,
}

impl RbDataFrame {
    pub(crate) fn new(df: Arc<DataFrame>) -> Self {
        Self { df }
    }

    pub(crate) fn collect(&self) -> Result<Vec<RbRecordBatch>, Error> {
        let result = self.df.collect();
        let batches = wait_for_future(result).map_err(DataFusionError::from)?;
        Ok(batches
            .into_iter()
            .map(|batch| RbRecordBatch::new(batch))
            .collect())
    }
}
