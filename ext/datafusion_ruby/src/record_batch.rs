use datafusion::arrow::{
    array::{Float64Array, Int64Array, StringArray},
    datatypes::DataType,
    record_batch::RecordBatch,
};
use magnus::{Value, Error};

use crate::errors::DataFusionError;
use std::collections::HashMap;

#[magnus::wrap(class = "Datafusion::RecordBatch")]
pub(crate) struct RbRecordBatch {
    rb: RecordBatch,
}

impl RbRecordBatch {
    pub(crate) fn new(rb: RecordBatch) -> Self {
        Self { rb }
    }

    pub(crate) fn to_hash(&self) -> Result<HashMap<String, Vec<Value>>, Error> {
        let mut columns_by_name: HashMap<String, Vec<Value>> = HashMap::new();
        for (i, field) in self.rb.schema().fields().iter().enumerate() {
            let column = self.rb.column(i);
            columns_by_name.insert(
                field.name().clone(),
                match column.data_type() {
                    DataType::Int64 => {
                        let array = column.as_any().downcast_ref::<Int64Array>().unwrap();
                        array.values().iter().map(|v| (*v as i64).into()).collect()
                    }
                    DataType::Float64 => {
                        let array = column.as_any().downcast_ref::<Float64Array>().unwrap();
                        array
                            .values()
                            .iter()
                            .map(|v| (*v as f64).into())
                            .collect()
                    },
                    DataType::Utf8 => {
                        let array = column.as_any().downcast_ref::<StringArray>().unwrap();
                        let mut values: Vec<Value> = vec![];
                        for i in 0..(column.len()) {
                            values.push(std::string::String::from(array.value(i)).into())
                        }
                        values
                    },
                    unknown => {
                        return Err(DataFusionError::CommonError(format!(
                            "unhandle data type: {}",
                            unknown
                        ))
                        .into())
                    }
                },
            );
        }
        Ok(columns_by_name)
    }
}
