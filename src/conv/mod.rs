use std::borrow::Borrow;
use std::collections::HashMap;
use rust_bert::pipelines::conversation::{Conversation, ConversationConfig, ConversationManager, ConversationModel};
use rust_bert::RustBertError;
use uuid::{Error, Uuid};
#[derive(Debug)]
pub enum Author {
    Bot,
    User,
}
#[derive(Debug)]
pub struct Messages {
    author: Author,
    content: String,
}
#[derive(Debug)]
pub struct ConversationItem {
    id: Uuid,
    messages: Vec<Messages>
}
pub struct ConvModule {
    manager: ConversationManager,
    model: ConversationModel,
    conversations: Vec<ConversationItem>

}

impl ConvModule {
    pub async fn new() -> ConvModule {
        Self {
            model: load_conv_model().await,
            manager: ConversationManager::new(),
            conversations: Vec::new(),
        }
    }
    pub fn discuss(&mut self, input: String, id: Uuid) {
        let conversation = match self.find_or_create(id) {
            None => println!("Error creating the conversation"),
            Some(conv) => {
                let conv = self.manager.get(&id);
            },

        };

    }
    pub fn saveMessage() {}

    pub fn create(&mut self) -> &ConversationItem {
        let uuid = self.manager.add(Conversation::new_empty());
        self.conversations.push(ConversationItem {
            id: uuid,
            messages: Vec::new(),
        });
        self.conversations.iter().find(|&x| x.id == uuid).unwrap()
    }
    pub fn find_or_create(&mut self, uuid: Uuid) -> Option<& ConversationItem> {
        let found_conv =  self.conversations.iter().find(|&x| x.id == uuid);
        found_conv

    }
    pub fn say(&mut self, input: &str) -> Uuid {
        let uuid = self.manager.add(Conversation::new(input));
        uuid

    }
    pub fn answer<'a>(&mut self) -> HashMap<& Uuid, & str> {
        let resp = self.model.generate_responses(&mut self.manager);
    trace!("  Got responses: {:?}", resp);
    println!("  Got responses: {:?}", resp);
        resp
    }
}
async fn load_conv_model() -> ConversationModel {
    let conv_model = tokio::task::spawn_blocking(|| -> Result<ConversationModel, RustBertError> {
        let conv = ConversationModel::new(ConversationConfig {
            do_sample: false,
            num_beams: 3,
            ..Default::default()
        })?;
        Ok(conv)
    });
    let result = conv_model.await.unwrap();
    match result {
        Ok(model) => model,
        Err(error) => panic!("Problem getting conv: {:?}", error),
    }

}