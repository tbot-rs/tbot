use crate::types::parameters::ChatId;
use serde::Serialize;
use std::{borrow::Cow, collections::HashSet, iter::repeat};

enum Header<'a> {
    Field(&'static str),
    File {
        name: Cow<'a, str>,
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
    body: Cow<'a, [u8]>,
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
            body: Cow::Borrowed(value.as_bytes()),
        });
        self
    }

    fn part(mut self, name: &'static str, body: Cow<'a, [u8]>) -> Self {
        self.parts.push(Part {
            header: Header::Field(name),
            body,
        });
        self
    }

    pub fn string(self, name: &'static str, value: &impl ToString) -> Self {
        self.part(name, Cow::Owned(value.to_string().into_bytes()))
    }

    pub fn json(self, name: &'static str, value: impl Serialize) -> Self {
        self.part(name, Cow::Owned(serde_json::to_vec(&value).unwrap()))
    }

    pub fn maybe_str(self, name: &'static str, value: Option<&'a str>) -> Self {
        match value {
            Some(value) => self.str(name, value),
            None => self,
        }
    }

    pub fn maybe_string(
        self,
        name: &'static str,
        value: Option<impl ToString>,
    ) -> Self {
        match value {
            Some(value) => self.string(name, &value),
            None => self,
        }
    }

    pub fn maybe_json(
        self,
        name: &'static str,
        value: Option<impl Serialize>,
    ) -> Self {
        match value {
            Some(value) => self.json(name, &value),
            None => self,
        }
    }

    pub fn chat_id(self, name: &'static str, id: ChatId<'a>) -> Self {
        match id {
            ChatId::Id(id) => self.string(name, &id),
            ChatId::Username(username) => self.str(name, username),
        }
    }

    fn file_cow(
        mut self,
        name: Cow<'a, str>,
        filename: &'a str,
        body: &'a [u8],
    ) -> Self {
        self.parts.push(Part {
            header: Header::File {
                name,
                filename,
            },
            body: Cow::Borrowed(body),
        });
        self
    }

    pub fn file(
        self,
        name: &'a str,
        filename: &'a str,
        body: &'a [u8],
    ) -> Self {
        self.file_cow(Cow::Borrowed(name), filename, body)
    }

    pub fn file_owned_name(
        self,
        name: String,
        filename: &'a str,
        body: &'a [u8],
    ) -> Self {
        self.file_cow(Cow::Owned(name), filename, body)
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

        let boundary_length = (1..=usize::max_value())
            .find(|n| {
                !line_lengths.contains(&(*n + 2)) // --boundary
                && !line_lengths.contains(&(*n + 4)) // --boundary--
            })
            .unwrap();

        let boundary_string: String =
            repeat('-').take(boundary_length).collect();
        let boundary = boundary_string.as_bytes();

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

            match part.body {
                Cow::Owned(bytes) => body.extend_from_slice(&bytes[..]),
                Cow::Borrowed(bytes) => body.extend_from_slice(bytes),
            }
            body.extend_from_slice(b"\r\n");
        }

        body.extend_from_slice(b"--");
        body.extend_from_slice(&boundary);
        body.extend_from_slice(b"--\r\n");

        (boundary_string, body)
    }
}
