use async_compression::tokio::write::GzipEncoder;
use tokio::io::AsyncWriteExt;

pub async fn gzip_response(body: &[u8]) -> anyhow::Result<Vec<u8>> {
    let buf = Vec::new();
    let mut encoder = GzipEncoder::new(buf);
    encoder.write_all(body).await?;
    encoder.shutdown().await?;
    let compressed = encoder.into_inner();
    Ok(compressed)
}
