use fastembed::TextEmbedding;

pub fn init_embedder() -> anyhow::Result<TextEmbedding> {
    // try_new with Default::default() uses a fast, lightweight quantized model.
    // It automatically downloads and caches the model locally on the first run.
    let model = TextEmbedding::try_new(Default::default())?;
    Ok(model)
}
