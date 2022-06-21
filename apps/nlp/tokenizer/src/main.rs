// cspell:ignore pretrained izer

use std::path::{self, Path};
use tokenizers::models::bpe::{BpeTrainerBuilder, BPE};
use tokenizers::normalizers::{strip::Strip, unicode::NFC, utils::Sequence, NormalizerWrapper};
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::tokenizer::{EncodeInput, Result, Tokenizer};
use tokenizers::{AddedToken, Model, TokenizerBuilder};

// use tokenizers::decoders::DecoderWrapper;
// use tokenizers::pre_tokenizers::PreTokenizerWrapper;
// use tokenizers::processors::PostProcessorWrapper;

fn main() -> Result<()> {
    // Loading a pretrained tokenizer from the Hub
    let mut input: &str = "This is a test of the tokenizer.";
    load_pretrained_tokenizer(input)?; // ["This", "is", "a", "test", "of", "the", "token", "##izer", "."]

    input = "Hello, world!";
    load_pretrained_tokenizer(input)?; // ["Hello", ",", "world", "!"]

    // Deserialization and tokenization example
    // encode_deserialize_tokenizer()?;

    // Training and serialization example
    // train_serialize_tokenizer()?;

    Ok(())
}

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
// Deserialization and tokenization example
fn encode_deserialize_tokenizer() -> Result<()> {
    let path_to_vocab_json = path::Path::new("vocab.json");
    let path_to_merges_txt = path::Path::new("merges.txt");
    let path_vocab_str = path_to_vocab_json.to_str().unwrap();
    let path_merges_str = path_to_merges_txt.to_str().unwrap();

    let bpe_builder = BPE::from_file(path_vocab_str, path_merges_str);
    let bpe = bpe_builder.dropout(0.1).unk_token("[UNK]".into()).build()?;

    let tokenizer = Tokenizer::new(bpe);

    let encoding = tokenizer.encode("Hey there!", false)?;
    println!("{:?}", encoding.get_tokens());

    Ok(())
}

// Training and serialization example
fn train_serialize_tokenizer() -> Result<()> {
    let vocab_size: usize = 100;

    let mut trainer = BpeTrainerBuilder::new()
        .show_progress(true)
        .vocab_size(vocab_size)
        .min_frequency(0)
        .special_tokens(vec![
            AddedToken::from(String::from("<s>"), true),
            AddedToken::from(String::from("<pad>"), true),
            AddedToken::from(String::from("</s>"), true),
            AddedToken::from(String::from("<unk>"), true),
            AddedToken::from(String::from("<mask>"), true),
        ])
        .build();

    let mut tokenizer = TokenizerBuilder::new()
        .with_model(BPE::default())
        .with_normalizer(Some(Sequence::new(vec![
            Strip::new(true, true).into(),
            NFC.into(),
        ])))
        .with_pre_tokenizer(Some(ByteLevel::default()))
        .with_post_processor(Some(ByteLevel::default()))
        .with_decoder(Some(ByteLevel::default()))
        .build()?;

    let path_to_vocab: &Path = std::path::Path::new("./vocab.txt");
    let path_to_vocab_string: String = path_to_vocab.to_str().unwrap().to_string();
    let pretty = false;

    tokenizer
        .train_from_files(&mut trainer, vec![path_to_vocab_string])?
        .save("tokenizer.json", pretty)?;

    Ok(())
}

////////////////////////////////////////////////////////////
// https://crates.io/crates/tokenizers
