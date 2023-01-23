use nlprule::{
    types::{Suggestion, Token},
    Error, Rules, Tokenizer,
};
use rust_bert::{pipelines::pos_tagging::POSModel, RustBertError};

// pub async fn load_rules() -> Rules {
//     let corrector_model = tokio::task::spawn_blocking(|| -> Result<Rules, Error> {
//         let rules = Rules::new("C:/Users/hanss/Desktop/Dev/test-rust/rules/en_rules.bin")?;
//         Ok(rules)
//     });
//     let result = corrector_model.await.unwrap();
//     match result {
//         Ok(corrector) => corrector,
//         Err(error) => panic!("Problem getting corrector rules: {:?}", error),
//     }
// }
// pub async fn load_tokenizer() -> Tokenizer {
//     let tokenizer_model = tokio::task::spawn_blocking(|| -> Result<Tokenizer, Error> {
//         let tokenizer =
//             Tokenizer::new("C:/Users/hanss/Desktop/Dev/test-rust/rules/en_tokenizer.bin")?;
//         Ok(tokenizer)
//     });
//     let result = tokenizer_model.await.unwrap();
//     match result {
//         Ok(tokenizer) => tokenizer,
//         Err(error) => panic!("Problem getting corrector tokenizer: {:?}", error),
//     }
// }

// pub struct CorrectorModel {
//     pub rules: Rules,
//     pub tokenizer: Tokenizer,
// }
// impl CorrectorModel {
//     pub fn init(rules: Rules, tokenizer: Tokenizer) -> CorrectorModel {
//         CorrectorModel {
//             rules: rules,
//             tokenizer: tokenizer,
//         }
//     }
// }

// use nlprule::{Rules, Tokenizer};

pub struct NlpRuleChecker {
    tokenizer: Tokenizer,
    rules: Rules,
}

impl NlpRuleChecker {
    pub fn new() -> Self {
        // utils::set_panic_hook();

        let tokenizer_bytes: &'static [u8] = include_bytes!("../../rules/en_tokenizer.bin");
        let rules_bytes: &'static [u8] = include_bytes!("../../rules/en_rules.bin");

        println!("Init Tokenizer");
        let tokenizer = Tokenizer::from_reader(tokenizer_bytes).expect("tokenizer binary is valid");

        println!("Init Rules");
        let rules = Rules::from_reader(rules_bytes).expect("rules binary is valid");

        println!("NlpRuleChecker is ready.");
        NlpRuleChecker { tokenizer, rules }
    }

    pub fn check(&self, text: &str) -> Vec<Suggestion> {
        let suggestions = self.rules.suggest(text, &self.tokenizer);
        println!("{:?}", suggestions);
        suggestions
    }

    pub fn sentencize(&self, text: &str) -> Vec<String> {
        let sentences = self
            .tokenizer
            .sentencize(text)
            .map(|it| it.text().to_string())
            .collect::<Vec<String>>();
        sentences
    }

    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let sentences = self
            .tokenizer
            .pipe(text)
            .map(|it| {
                it.tokens()
                    .iter()
                    .map(|token| format!("{:?}", token))
                    .collect()
            })
            .collect::<Vec<String>>();
        sentences
    }
}
