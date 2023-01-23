use rust_bert::{pipelines::pos_tagging::POSModel, RustBertError};

pub async fn load_pos_model() -> POSModel {
    let pos_model = tokio::task::spawn_blocking(|| -> Result<POSModel, RustBertError> {
        let models: POSModel = POSModel::new(Default::default())?;
        Ok(models)
    });
    let result = pos_model.await.unwrap();
    match result {
        Ok(model) => model,
        Err(error) => panic!("Problem getting pos: {:?}", error),
    }
}

#[derive(Clone)]
pub struct StringInput {
    pub text: String,
}

impl StringInput {
    pub fn new(text: String) -> StringInput {
        StringInput { text: text }
    }
}
