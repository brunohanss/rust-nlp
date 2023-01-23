use rust_bert::{
    pipelines::translation::{Language, TranslationModel, TranslationModelBuilder},
    RustBertError,
};

pub async fn load() -> TranslationModel {
    let translation_model =
        tokio::task::spawn_blocking(|| -> Result<TranslationModel, RustBertError> {
            let models: TranslationModel = TranslationModelBuilder::new()
                .with_source_languages(vec![Language::English])
                .with_target_languages(vec![Language::Spanish, Language::French, Language::Italian])
                .create_model()?;
            Ok(models)
        });
    let result = translation_model.await.unwrap();
    match result {
        Ok(model) => model,
        Err(error) => panic!("Problem getting pos: {:?}", error),
    }
}
pub fn translate(
    translator: TranslationModel,
    input: &str,
    // source_language: &str,
    // target_language: &str,
) -> Result<Vec<String>, RustBertError> {
    translator.translate(&[input], None, Language::English)
}
