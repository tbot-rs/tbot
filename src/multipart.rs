use std::collections::HashSet;

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
        match value {
            Some(value) => self.str(name, value),
            None => self,
        }
    }

    pub fn maybe_str(self, name: &'static str, value: Option<&'a str>) -> Self {
        match value {
            Some(value) => self.str(name, value),
            None => self,
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
        let mut line_lengths = HashSet::new();

        for part in &self.parts {
            let mut current_line_length = 0;

            for line in part.body.split(|byte| *byte == b'\n') {
                current_line_length += line.len();

                if line.ends_with(b"\r") {
                    line_lengths.insert(current_line_length - 1);
                    current_line_length = 0;
                } else {
                    current_line_length += 1; // to count the \n
                }
            }

            line_lengths.insert(current_line_length - 1);
        }

        let boundary_length = (1..)
            .filter(|n| {
                !line_lengths.contains(&(*n + 2)) // --boundary
            && !line_lengths.contains(&(*n + 4)) // --boundary--
            })
            .next()
            .unwrap();

        let boundary = vec![b'-'; boundary_length];

        let mut body = Vec::new();

        for part in self.parts {
            body.extend_from_slice(b"--");
            body.extend_from_slice(&boundary);
            body.extend_from_slice(b"\r\n");

            body.extend_from_slice(b"Content-Disposition: form-data; ");
            body.extend_from_slice(
                part.header.content_disposition().as_bytes(),
            );
            body.extend_from_slice(b"\r\n\r\n");

            body.extend_from_slice(part.body);
            body.extend_from_slice(b"\r\n");
        }

        body.extend_from_slice(b"--");
        body.extend_from_slice(&boundary);
        body.extend_from_slice(b"--\r\n");

        (String::from_utf8(boundary).unwrap(), body)
    }
}
