use rust_bert::{
    pipelines::{
        summarization::SummarizationModel,
        translation::{Language, TranslationModel, TranslationModelBuilder},
    },
    RustBertError,
};

pub async fn load() -> SummarizationModel {
    let summarization_model =
        tokio::task::spawn_blocking(|| -> Result<SummarizationModel, RustBertError> {
            let models = SummarizationModel::new(Default::default())?;
            Ok(models)
        });
    let result = summarization_model.await.unwrap();
    match result {
        Ok(model) => model,
        Err(error) => panic!("Problem getting pos: {:?}", error),
    }
}
pub fn summarize(
    summarize_model: SummarizationModel,
    input: &str,
    // source_language: &str,
    // target_language: &str,
) -> Vec<String> {
    summarize_model.summarize(&[input])
}
