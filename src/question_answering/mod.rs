use rust_bert::{pipelines::question_answering::QuestionAnsweringModel, RustBertError};

pub async fn load() -> QuestionAnsweringModel {
    let pos_model =
        tokio::task::spawn_blocking(|| -> Result<QuestionAnsweringModel, RustBertError> {
            let models: QuestionAnsweringModel = QuestionAnsweringModel::new(Default::default())?;
            Ok(models)
        });
    let result = pos_model.await.unwrap();
    match result {
        Ok(model) => model,
        Err(error) => panic!("Problem getting question answering: {:?}", error),
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
