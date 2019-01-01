use rand::{distributions::Alphanumeric, rngs::SmallRng, FromEntropy, Rng};

fn generate_boundary() -> Vec<u8> {
    let mut rng = SmallRng::from_entropy();
    let ascii = rng.sample_iter(&Alphanumeric).map(|x| x as u8);

    ascii.take(40).collect()
}

enum Header<'a> {
    Field(&'static str),
    File {
        name: &'a str,
        filename: &'a str,
    },
}

impl<'a> Header<'a> {
    pub fn content_disposition(&self) -> String {
        match self {
            Header::Field(name) => format!("name=\"{}\"", name),
            Header::File {
                name,
                filename,
            } => format!("name=\"{}\"; filename=\"{}\"", name, filename),
        }
    }
}

struct Part<'a> {
    header: Header<'a>,
    body: &'a [u8],
}

pub struct Multipart<'a> {
    parts: Vec<Part<'a>>,
}

impl<'a> Multipart<'a> {
    pub fn new(capacity: usize) -> Self {
        Self {
            parts: Vec::with_capacity(capacity),
        }
    }

    pub fn str(mut self, name: &'static str, value: &'a str) -> Self {
        self.parts.push(Part {
            header: Header::Field(name),
            body: value.as_bytes(),
        });
        self
    }

    pub fn maybe_string(
        self,
        name: &'static str,
        value: &'a Option<String>,
    ) -> Self {
        if let Some(value) = &value {
            self.str(name, value)
        } else {
            self
        }
    }

    pub fn maybe_str(self, name: &'static str, value: Option<&'a str>) -> Self {
        if let Some(value) = value {
            self.str(name, value)
        } else {
            self
        }
    }

    pub fn file(
        mut self,
        name: &'a str,
        filename: &'a str,
        body: &'a [u8],
    ) -> Self {
        self.parts.push(Part {
            header: Header::File {
                name,
                filename,
            },
            body,
        });
        self
    }

    pub fn finish(self) -> (String, Vec<u8>) {
        let boundary = generate_boundary();
        let mut body = Vec::new();
        for part in self.parts {
            if body.is_empty() {
                body.extend_from_slice(b"--");
                body.extend_from_slice(&boundary);
            }
            body.extend_from_slice(b"\r\n");
            body.extend_from_slice(b"Content-Disposition: form-data; ");
            body.extend_from_slice(
                &part.header.content_disposition().as_bytes(),
            );
            body.extend_from_slice(b"\r\n\r\n");
            body.extend_from_slice(&part.body);
            body.extend_from_slice(b"\r\n--");
            body.extend_from_slice(&boundary);
        }
        body.extend_from_slice(b"--\r\n");
        (String::from_utf8(boundary).unwrap(), body)
    }
}
