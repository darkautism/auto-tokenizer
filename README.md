### AutoTokenizer

`AutoTokenizer` 是一個用於自動從 Hugging Face 下載 token 配置並生成 prompt 的 Rust 庫，靈感來自 Python 的 AutoTokenizer。

#### 安裝

您可以通過在 `Cargo.toml` 文件中添加以下內容來安裝此庫：

```toml
[dependencies]
autotokenizer = "0.1.0"
```

#### 使用範例

以下是一個簡單的範例，展示如何使用 `AutoTokenizer`：

```rust
use autotokenizer::{AutoTokenizer, DefaultPromptMessage};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 從 Hugging Face 預訓練模型加載 Tokenizer
    let a = AutoTokenizer::from_pretrained("TinyLlama/TinyLlama-1.1B-Chat-v1.0".to_string(), None)?;

    // 定義對話上下文
    let ctx = vec![
        DefaultPromptMessage::new("system", "Hello"),
        DefaultPromptMessage::new("user", "Hello"),
        DefaultPromptMessage::new("assistant", "Hello"),
    ];

    // 應用對話模板並生成 prompt
    let chat = a.apply_chat_template(ctx, true)?;
    println!("This is chat: {}", chat);

    Ok(())
}
```

#### 功能

- 從 Hugging Face 下載 token 配置
- 自動生成對話 prompt
- 支援自訂對話角色和內容
