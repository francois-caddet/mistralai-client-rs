use reqwest::Url;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Img {
    detail: Option<String>,
    url: String,
}

impl Img {
    pub fn with_detail(self, detail: impl Into<String>) -> Self {
        Self {
            detail: Some(detail.into()),
            ..self
        }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Self {
        use base64::{engine::general_purpose::STANDARD, write::EncoderStringWriter};
        use mime_guess::from_path;
        use std::{fs::File, io::copy, io::Read};
        let mime = from_path(&path).first().unwrap();
        let img_b64 = {
            let mut f = File::open(path).unwrap();
            let mut encoder = EncoderStringWriter::new(&STANDARD);
            copy(&mut f, &mut encoder).unwrap();
            encoder.into_inner()
        };
        Self {
            url: format!("data:{mime};base64,{img_b64}"),
            detail: None,
        }
    }

    pub fn from_url(url: impl AsRef<str>) -> Self {
        let url: Url = url.as_ref().parse().unwrap();
        Self {
            url: url.to_string(),
            detail: None,
        }
    }
}
