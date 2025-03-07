use kovi::PluginBuilder as plugin;
use openai_api_rust::chat::*;
use openai_api_rust::*;
// use openai_api_rust::completions::*;
use std::env;

#[kovi::plugin]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");
    plugin::on_msg(|event| async move {
        if let Some(text) = event.borrow_text() {
            if !text.starts_with('%') {
                return;
            }

            let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
            let api_url = env::var("OPENAI_API_URL").expect("OPENAI_API_URL not set");
            let auth = Auth::new(&api_key);
            let openai = OpenAI::new(auth, &api_url);

            let body = ChatBody {
                model: "gpt-3.5-turbo".to_string(),
                max_tokens: None,
                temperature: None,
                top_p: None,
                n: None,
                stream: Some(false),
                stop: None,
                presence_penalty: None,
                frequency_penalty: None,
                logit_bias: None,
                user: None,
                messages: vec![Message {
                    role: Role::User,
                    content: text[1..].to_string(),
                }],
            };

            match openai.chat_completion_create(&body) {
                Ok(rs) => {
                    let choice = rs.choices;
                    if let Some(message) = &choice[0].message {
                        event.reply(&message.content);
                    }
                }
                Err(err) => {
                    event.reply(format!("Error: {}", err));
                }
            }
        }
    });
}
