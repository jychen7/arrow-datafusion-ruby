use datafusion::execution::context::SessionContext;
use datafusion::prelude::CsvReadOptions;
use magnus::Error;

use crate::dataframe::RbDataFrame;
use crate::errors::DataFusionError;
use crate::utils::wait_for_future;

#[magnus::wrap(class = "Datafusion::SessionContext")]
pub(crate) struct RbSessionContext {
    ctx: SessionContext,
}

impl RbSessionContext {
    pub(crate) fn new() -> Self {
        Self {
            ctx: SessionContext::new(),
        }
    }

    pub(crate) fn register_csv(&self, name: String, table_path: String) -> Result<(), Error> {
        let result =
            self.ctx
                .register_csv(name.as_ref(), table_path.as_ref(), CsvReadOptions::new());
        wait_for_future(result).map_err(DataFusionError::from)?;
        Ok(())
    }

    pub(crate) fn sql(&self, query: String) -> Result<RbDataFrame, Error> {
        let result = self.ctx.sql(query.as_ref());
        let df = wait_for_future(result).map_err(DataFusionError::from)?;
        Ok(RbDataFrame::new(df))
    }
}
