// cspell:ignore pretrained izer


use tokenizers::tokenizer::{Result, Tokenizer};

fn main() -> Result<()> {
  let mut input: &str = "This is a test of the tokenizer.";
  load_pretrained_tokenizer(input)?; // ["This", "is", "a", "test", "of", "the", "token", "##izer", "."]

  input = "Hello, world!";
  load_pretrained_tokenizer(input)?; // ["Hello", ",", "world", "!"]

  Ok(())
}
// Deserialization and tokenization example



// Loading a pretrained tokenizer from the Hub
fn load_pretrained_tokenizer(input: &str) -> Result<()> {
  // #[cfg(feature = "http")]
  {
    let tokenizer: Tokenizer = Tokenizer::from_pretrained("bert-base-cased", None)?;
    let encoding: tokenizers::Encoding = tokenizer.encode(input, false)?;

    println!("{:?}", encoding.get_tokens());
  }
  Ok(())
}

// Training and serialization example

////////////////////////////////////////////////////////////////////////////////
//
// https://crates.io/crates/tokenizers
//
//////////////////////////////////////////////////////////////////////////////
