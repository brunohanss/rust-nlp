use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::conversation::{
    Conversation, ConversationConfig, ConversationManager, ConversationModel,
};
use rust_bert::resources::LocalResource;
use uuid::Uuid;

use scopeguard::defer_on_unwind;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::RecvTimeoutError;
use std::sync::{Arc, Mutex};

use err_derive::Error;
use inflector::cases::{
    sentencecase::{is_sentence_case, to_sentence_case},
    snakecase::to_snake_case,
};
use serde::{Deserialize, Serialize};

// use log::*;

// use crate::appctl::AppCtl;
// use crate::sumi::Sumi;
// use crate::Error;
// use crate::RX_TIMEOUT;

pub struct Conv {
    model: ConversationModel,
    manager: Mutex<ConversationManager>,
    uuid: Uuid,
    past: Mutex<Vec<String>>,
    max_context: usize,
    history: Mutex<Vec<Past>>,
    do_summary: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Speaker {
    Me,
    Bot,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Past {
    speaker: Speaker,
    id: u64,
    message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct History {
    history: Vec<Past>,
}

impl Conv {
    pub fn new(model_name: &str, max_context: usize) -> Self {
        let mut conversation_config;
        if model_name == "default" {
            conversation_config = ConversationConfig::default();
            conversation_config.min_length = 2;
        } else {
            conversation_config = ConversationConfig {
                model_type: ModelType::GPT2,
                model_resource: Box::new(LocalResource {
                    local_path: PathBuf::from(format!("./{}.model/model.ot", model_name)),
                }),
                config_resource: Box::new(LocalResource {
                    local_path: PathBuf::from(format!("./{}.model/config.json", model_name)),
                }),
                vocab_resource: Box::new(LocalResource {
                    local_path: PathBuf::from(format!("./{}.model/vocab.json", model_name)),
                }),
                merges_resource: Box::new(LocalResource {
                    local_path: PathBuf::from(format!("./{}.model/merges.txt", model_name)),
                }),
                min_length: 2,
                max_length: 100,
                min_length_for_response: 32,
                do_sample: true,
                early_stopping: false,
                num_beams: 5,
                temperature: 1.3,
                top_k: 50,
                top_p: 0.95,
                repetition_penalty: 1.5,
                length_penalty: 1.0,
                no_repeat_ngram_size: 0,
                num_return_sequences: 1,
                diversity_penalty: None,
                num_beam_groups: None,
                ..Default::default()
            };
        }

        let conversation_model =
            ConversationModel::new(conversation_config).expect("Unable to setup model");

        let conversation = Conversation::new_empty();

        let mut conversation_manager = ConversationManager::new();
        let conversation_uuid = conversation_manager.add(conversation);

        Self {
            model: conversation_model,
            manager: Mutex::new(conversation_manager),
            uuid: conversation_uuid,
            past: Mutex::new(vec![]),
            max_context,
            history: Mutex::new(Default::default()),
            do_summary: false,
        }
    }

    // pub fn remember_past(&self, file_path: &str) -> Result<(), Error> {
    //     let history_path: PathBuf = PathBuf::from(file_path);
    //     let user_past_str = fs::read_to_string(&history_path).unwrap_or_else(|_| {
    //         info!("They do not know you yet");
    //         "".to_string()
    //     });

    //     let mut history_file: History =
    //         toml::from_str(&user_past_str).expect("Couldn't load history.");

    //     let mut conversation_manager = self.manager.lock().unwrap();
    //     if let Some(conversation) = conversation_manager.get(&self.uuid).as_mut() {
    //         history_file.history.sort_unstable_by_key(|k| k.id);

    //         if self.do_summary {
    //             let sumi = Sumi::new();
    //             let all_me_messages: Vec<String> = history_file
    //                 .history
    //                 .iter()
    //                 .flat_map(|m| match m.speaker {
    //                     Speaker::Me => Some(m.message.clone()),
    //                     _ => None,
    //                 })
    //                 .collect();

    //             let msg_start = std::cmp::max(all_me_messages.len() - 50, 0);
    //             info!(
    //                 "Summary Me: {}",
    //                 sumi.summary(&all_me_messages[msg_start..].join("\n"))
    //                     .unwrap()
    //             );

    //             let all_bot_messages: Vec<String> = history_file
    //                 .history
    //                 .iter()
    //                 .flat_map(|m| match m.speaker {
    //                     Speaker::Bot => Some(m.message.clone()),
    //                     _ => None,
    //                 })
    //                 .collect();

    //             let msg_start = std::cmp::max(all_bot_messages.len() - 50, 0);
    //             info!(
    //                 "Summary Bot: {}",
    //                 sumi.summary(&all_bot_messages[msg_start..].join("\n"))
    //                     .unwrap()
    //             );

    //             let all_messages: Vec<String> = history_file
    //                 .history
    //                 .iter()
    //                 .map(|m| match m.speaker {
    //                     Speaker::Me => m.message.clone(),
    //                     Speaker::Bot => m.message.clone(),
    //                 })
    //                 .collect();
    //             let msg_start = std::cmp::max(all_messages.len() - 50, 0);

    //             info!(
    //                 "Summary: {}",
    //                 sumi.summary(&all_messages[msg_start..].join("\n")).unwrap()
    //             );
    //         }

    //         let mut my_history = self.history.lock().unwrap();
    //         for past in history_file.history {
    //             match past.speaker {
    //                 Speaker::Me => {
    //                     conversation.past_user_inputs.push(past.message.clone());
    //                 }
    //                 Speaker::Bot => {
    //                     conversation.generated_responses.push(past.message.clone());
    //                 }
    //             }
    //             (*my_history).push(past.clone());
    //         }
    //         (*my_history).sort_unstable_by_key(|k| k.id);
    //         let history_texts: Vec<&str> =
    //             if self.max_context > 0 && (*my_history).len() > self.max_context * 2 {
    //                 let max_range = std::cmp::min(self.max_context * 2, (*my_history).len() - 1);
    //                 (*my_history)[0..max_range]
    //                     .iter()
    //                     .map(|k| k.message.as_str())
    //                     .collect()
    //             } else {
    //                 (*my_history).iter().map(|k| k.message.as_str()).collect()
    //             };
    //         let history_ids = self.model.encode_prompts(&history_texts);
    //         conversation.load_from_history(&history_texts, &history_ids);
    //         Ok(())
    //     } else {
    //         Err(Error::ConversationUnknown)
    //     }
    // }

    pub fn add_to_journel(&self, speaker: Speaker, message: &str) {
        let mut my_history = self.history.lock().unwrap();
        let new_id;
        if let Some(last_item) = my_history.last() {
            new_id = last_item.id + 1;
        } else {
            new_id = 0
        }
        (*my_history).push(Past {
            speaker,
            id: new_id,
            message: message.to_string(),
        })
    }

    pub fn say(&self, input: &str) -> Result<String, Error> {
        trace!("  Conv recieved: {}", input);
        let mut conversation_manager = self.manager.lock().unwrap();
        if let Some(convo) = conversation_manager.get(&self.uuid).as_mut() {
            self.trim_context(convo);
            //let input = Self::swap_persons(input);
            //trace!("  Persons swapped: {}", input);
            if convo.add_user_input(input).is_err() {
                return Err(Error::UnableToSpeak);
            }
        } else {
            return Err(Error::ConversationUnknown);
        }
        let output = {
            trace!("  Generating responses");
            let resp = self.model.generate_responses(&mut conversation_manager);
            trace!("  Got responses: {:?}", resp);
            if let Some(my_resp) = resp.get(&self.uuid) {
                Ok(my_resp.to_string())
            } else {
                Err(Error::UnableToSpeak)
            }
        }?;
        self.past.lock().unwrap().push(input.to_owned());
        Ok(output)
    }

    fn trim_context(&self, convo: &mut Conversation) {
        if self.max_context > 0 {
            if convo.past_user_inputs.len() > self.max_context {
                trace!("Old UserInput len: {:?}", convo.past_user_inputs.len());
                let drain_amount = convo.past_user_inputs.len() - self.max_context;
                convo.past_user_inputs.drain(0..drain_amount);
                trace!("New UserInput len: {:?}", convo.past_user_inputs.len());
            }
            if convo.generated_responses.len() > self.max_context {
                trace!("Old GenResp len: {:?}", convo.generated_responses.len());
                let drain_amount = convo.generated_responses.len() - self.max_context;
                convo.generated_responses.drain(0..drain_amount);
                trace!("New GenResp len: {:?}", convo.generated_responses.len());
            }
            let expected_history_size =
                convo.generated_responses.len() + convo.past_user_inputs.len();
            if convo.history.len() > expected_history_size {
                trace!("Old Hist len: {:?}", convo.generated_responses.len());
                let drain_amount = convo.history.len() - expected_history_size;
                convo.history.drain(0..drain_amount);
                trace!("New Hist len: {:?}", convo.generated_responses.len());
            }
        }
    }

    // pub fn save_journal(&self, file_path: &str) -> Result<(), Error> {
    //     let my_history = self.history.lock().unwrap();
    //     if std::fs::write(
    //         file_path,
    //         toml::to_vec(&History {
    //             history: (*my_history).clone(),
    //         })
    //         .unwrap(),
    //     )
    //     .is_err()
    //     {
    //         Err(Error::UnableToWriteJournel)
    //     } else {
    //         Ok(())
    //     }
    // }

    #[allow(dead_code)]
    fn swap_persons(input: &str) -> String {
        let mut words = vec![];
        for word in input.split(' ').filter(|i| !i.is_empty()) {
            let pass_one = match to_snake_case(word).as_str() {
                "i" => "You",
                "me" => "you",
                "mine" => "yours",
                "my" => "your",
                "myself" => "yourself",
                "you" => "I",
                "yours" => "mine",
                "your" => "my",
                "yourself" => "myself",
                "am" => "are",
                "are" => "am",
                "i\'m" => "You're",
                "you\'re" => "I'm",
                n => n,
            }
            .to_string();
            let mut new_word = match word {
                "You" => "I",
                "you" => "me",
                _ => pass_one.as_str(),
            }
            .to_string();

            if is_sentence_case(word) && !is_sentence_case(&new_word) {
                new_word = to_sentence_case(&new_word);
            }
            words.push(new_word);
        }

        words.join(" ")
    }
}

// pub fn start_conv(appctl: &AppCtl, model_name: &str, max_context: usize, do_summary: bool) {
//     defer_on_unwind! { appctl.stop() }
//     let mut get_from_me = appctl.listen_me_channel();

//     debug!("Conversation model: Loading");
//     let mut conv_prep = Conv::new(model_name, max_context);
//     conv_prep.do_summary = do_summary;

//     let conv = Arc::new(conv_prep);
//     if conv.remember_past("./journal.toml").is_err() {
//         error!("They couldn't remember the past.");
//     }

//     while appctl.is_alive() {
//         match get_from_me.recv_timeout(RX_TIMEOUT) {
//             Ok(input) => {
//                 conv.add_to_journel(Speaker::Me, &input);

//                 match conv.say(&input) {
//                     Err(Error::UnableToHear) => error!("Couldn't hear you"),
//                     Err(Error::UnableToSpeak) => error!("Couldn't speak to you"),
//                     Err(Error::ConversationUnknown) => error!("Doesn't know you"),
//                     Err(_) => {}
//                     Ok(output) => {
//                         conv.add_to_journel(Speaker::Bot, &output);
//                         appctl.broadcast_bot_channel(&output);
//                     }
//                 }
//             }
//             Err(RecvTimeoutError::Disconnected) => {
//                 appctl.stop();
//                 error!("User communication channel dropped.");
//                 break;
//             }
//             Err(RecvTimeoutError::Timeout) => {
//                 continue;
//             }
//         }
//     }
//     info!("Leaving town");
//     if conv.save_journal("./journal.toml").is_err() {
//         error!("Failed to write journal.");
//     }
//     appctl.stop();
// }
/// Enum of applicable errors
#[derive(Debug, Error)]
pub enum Error {
    /// Occurs when the conversation is not found
    /// in the conversation manager
    #[error(display = "Unknown Speaker")]
    ConversationUnknown,
    /// Occurs when conversation model errors
    /// during receiveing a users message
    #[error(display = "Can't Hear")]
    UnableToHear,
    /// Occurs when the model errors when generating
    /// reply
    #[error(display = "Can't Speak")]
    UnableToSpeak,
    /// Occurs if the model fails to load the history
    /// file
    #[error(display = "Can't remember what happened")]
    UnableToWriteJournel,
    /// Occurs if the config file fails to validate
    // #[error(display = "Config file invalid")]
    // ValidationError(#[error(source)] validator::ValidationErrors),
    /// Occurs if the config file fails to deseralise
    // #[error(display = "Config syntax invalid")]
    // ConfigError(#[error(source)] toml::de::Error),
    /// Occurs if there is an io error during reading of
    /// the config
    #[error(display = "Cannot read config")]
    IoError(#[error(source)] std::io::Error),
}
