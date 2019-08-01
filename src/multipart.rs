use crate::types::{
    parameters::ChatId,
    value::{self, Bytes},
};
use serde::Serialize;
use std::collections::HashSet;

enum Header<'a> {
    Field(&'static str),
    File {
        name: value::String<'a>,
        filename: value::String<'a>,
    },
}

impl<'a> Header<'a> {
    pub fn content_disposition(&self) -> String {
        match self {
            Header::Field(name) => format!("name=\"{}\"", name),
            Header::File {
                name,
                filename,
            } => format!(
                "name=\"{}\"; filename=\"{}\"",
                name.as_str(),
                filename.as_str()
            ),
        }
    }
}

struct Part<'a> {
    header: Header<'a>,
    body: Bytes<'a>,
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

    pub fn str(
        mut self,
        name: &'static str,
        value: impl Into<value::String<'a>>,
    ) -> Self {
        let value: value::String<'a> = value.into();
        self.parts.push(Part {
            header: Header::Field(name),
            body: value.into_bytes(),
        });
        self
    }

    pub fn from(self, name: &'static str, value: &impl ToString) -> Self {
        self.str(name, value.to_string())
    }

    fn part(mut self, name: &'static str, body: Bytes<'a>) -> Self {
        self.parts.push(Part {
            header: Header::Field(name),
            body,
        });
        self
    }

    pub fn json(self, name: &'static str, value: impl Serialize) -> Self {
        self.part(name, serde_json::to_vec(&value).unwrap().into())
    }

    pub fn maybe_str(
        self,
        name: &'static str,
        value: Option<impl Into<value::String<'a>>>,
    ) -> Self {
        match value {
            Some(value) => self.str(name, value),
            None => self,
        }
    }

    pub fn maybe_from(
        self,
        name: &'static str,
        value: Option<impl ToString>,
    ) -> Self {
        self.maybe_str(name, value.map(|value| value.to_string()))
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
            ChatId::Id(id) => self.str(name, id.to_string()),
            ChatId::Username(username) => self.str(name, username),
        }
    }

    pub fn file(
        mut self,
        name: impl Into<value::String<'a>>,
        filename: impl Into<value::String<'a>>,
        body: impl Into<Bytes<'a>>,
    ) -> Self {
        self.parts.push(Part {
            header: Header::File {
                name: name.into(),
                filename: filename.into(),
            },
            body: body.into(),
        });
        self
    }

    pub fn finish(self) -> (String, Vec<u8>) {
        let mut line_lengths = HashSet::new();

        for part in &self.parts {
            let mut current_line_length = 0;

            for line in part.body.as_slice().split(|byte| *byte == b'\n') {
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

            body.extend_from_slice(part.body.as_slice());
            body.extend_from_slice(b"\r\n");
        }

        body.extend_from_slice(b"--");
        body.extend_from_slice(&boundary);
        body.extend_from_slice(b"--\r\n");

        (String::from_utf8(boundary).unwrap(), body)
    }
}
