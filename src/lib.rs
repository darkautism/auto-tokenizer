use minijinja::context;

mod from_pretrained;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename = "message")]
pub struct DefaultPromptMessage {
    #[serde(rename = "role")]
    role: String,
    #[serde(rename = "content")]
    content: String,
}

impl DefaultPromptMessage {
    pub fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TokenObj {
    #[serde(rename = "__type")]
    pub token_type: String,
    pub content: String,
    pub lstrip: bool,
    pub normalized: bool,
    pub rstrip: bool,
    pub single_word: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Token {
    String(String),
    TokenObj(TokenObj),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct AutoTokenizer {
    add_bos_token: Option<bool>,
    add_eos_token: Option<bool>,
    clean_up_tokenization_spaces: bool,
    legacy: bool,
    tokenizer_class: String,
    model_max_length: usize,
    bos_token: Option<Token>,
    eos_token: Option<Token>,
    pad_token: Option<Token>,
    unk_token: Option<Token>,
    chat_template: String,
}

impl AutoTokenizer {
    pub fn from_file<P: AsRef<std::path::Path>>(
        file: P,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = std::fs::read_to_string(file)?;
        let tokenizer = serde_json::from_str(&content)?;
        Ok(tokenizer)
    }

    pub fn from_pretrained(
        identifier: String,
        params: Option<crate::from_pretrained::FromPretrainedParameters>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let tokenizer_file = from_pretrained::from_pretrained(identifier, params)?;
        AutoTokenizer::from_file(tokenizer_file)
    }

    pub fn apply_chat_template<S: serde::Serialize>(
        &self,
        ctx: S,
        add_generation_prompt: bool,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut env = minijinja::Environment::new();
        env.add_template("default", &self.chat_template).unwrap();
        let tmpl = env.get_template("default").unwrap();
        let eos = if let Some(eos) = &self.eos_token {
            match eos {
                Token::String(realeos) => realeos,
                Token::TokenObj(token_obj) => &token_obj.content,
            }
        } else {
            &String::new()
        };
        let bos = if let Some(bos) = &self.bos_token {
            match bos {
                Token::String(realbos) => realbos,
                Token::TokenObj(token_obj) => &token_obj.content,
            }
        } else {
            &String::new()
        };
        let pad = if let Some(pad) = &self.pad_token {
            match pad {
                Token::String(realpad) => realpad,
                Token::TokenObj(token_obj) => &token_obj.content,
            }
        } else {
            &String::new()
        };
        let unk: &String = if let Some(unk) = &self.unk_token {
            match unk {
                Token::String(realunk) => realunk,
                Token::TokenObj(token_obj) => &token_obj.content,
            }
        } else {
            &String::new()
        };

        match tmpl.render(context! {
            messages=> ctx,
            unk_token=> *unk,
            pad_token=> *pad,
            bos_token=> *bos,
            eos_token=> *eos,
            add_generation_prompt=> add_generation_prompt
        }) {
            Ok(result) => Ok(result),
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))),
        }
    }
}
