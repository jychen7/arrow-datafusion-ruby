use magnus::{
    define_module, exception::ExceptionClass, function, memoize, method, prelude::*, Error, RModule,
};

mod context;
mod dataframe;
mod errors;
mod record_batch;
mod utils;

fn datafusion() -> RModule {
    *memoize!(RModule: define_module("Datafusion").unwrap())
}

fn datafusion_error() -> ExceptionClass {
    *memoize!(ExceptionClass: datafusion().define_error("Error", Default::default()).unwrap())
}

#[magnus::init]
fn init() -> Result<(), Error> {
    // ensure error is defined on load
    datafusion_error();

    let ctx_class = datafusion().define_class("SessionContext", Default::default())?;
    ctx_class.define_singleton_method("new", function!(context::RbSessionContext::new, 0))?;
    ctx_class.define_method(
        "register_csv",
        method!(context::RbSessionContext::register_csv, 2),
    )?;
    ctx_class.define_method("sql", method!(context::RbSessionContext::sql, 1))?;

    let df_class = datafusion().define_class("DataFrame", Default::default())?;
    df_class.define_method("collect", method!(dataframe::RbDataFrame::collect, 0))?;

    let rb_class = datafusion().define_class("RecordBatch", Default::default())?;
    rb_class.define_method("to_h", method!(record_batch::RbRecordBatch::to_hash, 0))?;

    Ok(())
}
