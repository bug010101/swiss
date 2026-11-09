use anyhow::{Context, Result};
use digest::Digest;

use crate::cli::{HashAlgorithm, HashArgs};
use crate::util::{input_or_stdin, trim_eol};

pub fn run(args: &HashArgs) -> Result<()> {
    let output = match &args.file {
        Some(path) => hash_file(path, args.algorithm)?,
        None => {
            let raw = input_or_stdin(args.text.as_deref())?;
            let input = trim_eol(&raw);
            hash_bytes(input.as_bytes(), args.algorithm)
        }
    };
    println!("{output}");
    Ok(())
}

fn hash_bytes(data: &[u8], algo: HashAlgorithm) -> String {
    match algo {
        HashAlgorithm::Md5 => hex::encode(md5::Md5::digest(data)),
        HashAlgorithm::Sha1 => hex::encode(sha1::Sha1::digest(data)),
        HashAlgorithm::Sha256 => hex::encode(sha2::Sha256::digest(data)),
        HashAlgorithm::Sha512 => hex::encode(sha2::Sha512::digest(data)),
    }
}

fn hash_file(path: &std::path::Path, algo: HashAlgorithm) -> Result<String> {
    macro_rules! hash_streaming {
        ($hasher:ty) => {{
            let mut hasher = <$hasher as Digest>::new();
            let mut file = std::fs::File::open(path)
                .with_context(|| format!("无法打开文件 {}", path.display()))?;
            std::io::copy(&mut file, &mut HashSink(&mut hasher))
                .context("读取文件失败")?;
            hex::encode(hasher.finalize())
        }};
    }
    Ok(match algo {
        HashAlgorithm::Md5 => hash_streaming!(md5::Md5),
        HashAlgorithm::Sha1 => hash_streaming!(sha1::Sha1),
        HashAlgorithm::Sha256 => hash_streaming!(sha2::Sha256),
        HashAlgorithm::Sha512 => hash_streaming!(sha2::Sha512),
    })
}

/// 把 Digest 适配成 Write，方便 std::io::copy 流式喂数据（大文件不占内存）。
struct HashSink<'a, D>(&'a mut D);

impl<D: Digest> std::io::Write for HashSink<'_, D> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Digest::update(self.0, buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
