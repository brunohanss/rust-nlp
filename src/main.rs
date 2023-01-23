#[macro_use]
extern crate rocket;
use corrector::NlpRuleChecker;
use dialog::Conv;
use lingua::{Language, LanguageDetector};
use rocket::futures::lock::Mutex;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use rust_bert::{
    pipelines::{
        conversation::{ConversationManager, ConversationModel},
        pos_tagging::{POSModel, POSTag},
        question_answering::{Answer, QaInput, QuestionAnsweringModel},
        sequence_classification::Label,
        summarization::SummarizationModel,
        token_classification::TokenClassificationModel,
        translation::{Language as RustBertLanguage, TranslationModel, TranslationModelBuilder},
    },
    RustBertError,
};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc, sync::Arc};
use std::borrow::{Borrow, BorrowMut};
use std::ops::DerefMut;
use std::str::FromStr;
use rust_bert::pipelines::conversation::ConversationConfig;
use uuid::{Uuid, uuid};
use crate::conv::ConvModule;

mod corrector;
pub mod dialog;
mod language_detection;
mod pos;
mod question_answering;
mod spell_check;
mod summarization;
mod translation;
mod conv;

// #[post("/dialog", format = "json", data = "<input>")]
// async fn dialog_endpoint(
//     model: &State<Mutex<Models>>,
//     input: Json<ConvInput>,
// ) -> ConvModule {
//
//     // let conv_module = model
//     //     .lock()
//     //     .await;
//     // conv_module.conversation_module
//     // let output = model.lock().await.conversation_model.generate_responses(manager.borrow_mut());
//     // println!("{:?}", output);
// // Json(conv_id)
// }



#[post("/summarize", format = "json", data = "<input>")]
async fn summarize_endpoint(
    model: &State<Mutex<Models>>,
    input: Json<DataInput>,
) -> Json<Vec<std::string::String>> {
    println!("Request - Input : {}", &input.input);
    Json(model.lock().await.summarize.summarize(&[&input.input]))
}

#[post("/question_answering", format = "json", data = "<input>")]
async fn question_answering_endpoint(
    model: &State<Mutex<Models>>,
    input: Json<DataInput>,
) -> Json<Vec<Vec<Answer>>> {
    println!("Request - Input : {}", &input.input);
    println!("Request - Context : {}", &input.context);
    let question: String = String::from(&input.input);
    let context: String = String::from(&input.context);
    Json(
        model
            .lock()
            .await
            .question_answering
            .predict(&[QaInput { question, context }], 1, 32),
    )
}

#[post("/pos", format = "json", data = "<input>")]
async fn pos_endpoint(
    model: &State<Mutex<Models>>,
    input: Json<DataInput>,
) -> Json<Vec<Vec<POSTag>>> {
    println!("Request - Input : {}", &input.input);
    Json(model.lock().await.pos.predict(&[&input.input]))
}

#[post("/lang_detect", format = "json", data = "<input>")]
async fn lang_detect_endpoint<'a>(
    model: &'a State<Mutex<Models>>,
    input: Json<DataInput>,
) -> Json<&'a str> {
    println!("Request - Input : {}", &input.input);
    let lang_detected = model
        .lock()
        .await
        .language_detect
        .detect_language_of(&input.input);
    let result = match lang_detected {
        Some(English) => "en",
        Some(French) => "fr",
        None => "none",
    };
    Json(result)
}

#[post("/translation", format = "json", data = "<input>")]
async fn translation_endpoint(
    model: &State<Mutex<Models>>,
    input: Json<DataInput>,
) -> Json<Vec<std::string::String>> {
    println!("Request - Input : {}", &input.input);
    let input_translated =
        model
            .lock()
            .await
            .translation
            .translate(&[&input.input], None, RustBertLanguage::French);
    let result = match input_translated {
        Ok(translation) => translation,
        Err(error) => panic!("Problem getting translation: {:?}", error),
    };
    Json(result)
}

// #[post("/nlp/<name>", format = "json", data = "<input>")] // <- route attribute
// async fn nlp(name: &str, model: &State<Mutex<Models>>, input: Json<DataInput>,) -> &'static str {
//     model
//         .lock()
//         .await
//         .conversation_module.say(&input.input);
//     let answer = model
//         .lock()
//         .await
//         .conversation_module.answer();
//     model
//         .lock()
//         .await
//         .conversation_module.find_or_create(Uuid::from_str(name).unwrap());
//     // let final_answer = answer.get(&Uuid::from_str(name).unwrap());
//     "Hello"

// }

// async fn say(input: &str) {
//     let conv = tokio::task::spawn_blocking(|| -> Result<Conv, RustBertError> {
//         let conv: Conv = dialog::Conv::new("default", 100);
//         Ok(conv)
//     });
//     let result = conv.await.unwrap();
//     let answer = match result {
//         Ok(conversation) => conversation,
//         Err(error) => panic!("Problem getting Conv: {:?}", error),
//     };

//     // NlpRuleChecker::new().check("She was not been here since Monday.");
//     // let conv = dialog::Conv::new("default", 10);
//     let response = answer.say(input);
//     let response_txt = match response {
//         Ok(res) => res,
//         Err(_) => todo!(),
//     };

//     println!("Here is the answer {:?}", response_txt);
// }

#[launch]
async fn rocket() -> Rocket<Build> {
    // rocket::ignite()

    rocket::build()
        .manage(Mutex::new(Models {
            pos: pos::load_pos_model().await,
            // corrector: NlpRuleChecker::new(),
            language_detect: language_detection::load(),
            translation: translation::load().await,
            question_answering: question_answering::load().await,
            summarize: summarization::load().await, // symspell: spell_check::load(),
            // conversation_module: ConvModule::new().await,
        }))
        .mount(
            "/",
            routes![
                question_answering_endpoint,
                pos_endpoint,
                lang_detect_endpoint,
                translation_endpoint,
                summarize_endpoint,
                nlp,
                // dialog_endpoint
            ],
        )
    // .mount("/", routes![nlp])
}
// async fn load_conv_model() -> ConversationModel {
//     let conv_model = tokio::task::spawn_blocking(|| -> Result<ConversationModel, RustBertError> {
//         let conv = ConversationModel::new(ConversationConfig {
//             do_sample: false,
//             num_beams: 3,
//             ..Default::default()
//         })?;
//         Ok(conv)
//     });
//     let result = conv_model.await.unwrap();
//     match result {
//         Ok(model) => model,
//         Err(error) => panic!("Problem getting conv: {:?}", error),
//     }

// }

fn get_word_position(original: String, word: &String) -> std::option::Option<usize> {
    return original.find(word);
}

fn from_string_to_vec(original: pos::StringInput) -> Vec<char> {
    let src3: String = String::from(original.text);
    let str3: &str = &src3;
    let char3: Vec<char> = src3.chars().collect::<Vec<_>>();
    let byte3: Vec<u8> = src3.as_bytes().to_vec();
    return char3;
}

struct Word {
    text: String,
    start: u32,
    end: u32,
    label: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DataInput {
    input: String,
    context: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConvInput {
    input: String,
    id: String,
}
pub struct Models {
    pos: POSModel,
    // corrector: NlpRuleChecker,
    language_detect: LanguageDetector,
    translation: TranslationModel,
    question_answering: QuestionAnsweringModel,
    summarize: SummarizationModel,
    // conversation_module: ConvModule,
}
pub struct StateApp {
    // managers: Managers,
    models: Models,
}
