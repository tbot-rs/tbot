#![allow(clippy::non_ascii_literal)]

use crate::types::message::{
    text::{Entity as RawEntity, EntityKind},
    Text,
};
use crate::util::{
    entities,
    entities::{Entity, FormattedText, SemanticEntity},
};

#[test]
fn entities_parsing() {
    let text = Text {
        value: "a b c".to_string(),
        entities: vec![
            RawEntity {
                kind: EntityKind::Code,
                offset: 0,
                length: 1,
            },
            RawEntity {
                kind: EntityKind::Code,
                offset: 4,
                length: 1,
            },
        ],
    };

    assert_eq!(
        entities(&text),
        [
            Entity::Code("a".to_string()),
            Entity::Semantic(SemanticEntity {
                kind: None,
                value: vec![FormattedText {
                    value: " b ".to_string(),
                    is_bold: false,
                    is_italic: false,
                    is_strikethrough: false,
                    is_underline: false,
                }],
            }),
            Entity::Code("c".to_string()),
        ]
    );

    let text = Text {
        value: "foo".to_string(),
        entities: vec![RawEntity {
            kind: EntityKind::Bold,
            offset: 0,
            length: 3,
        }],
    };

    assert_eq!(
        entities(&text),
        [Entity::Semantic(SemanticEntity {
            kind: None,
            value: vec![FormattedText {
                value: "foo".to_string(),
                is_bold: true,
                is_italic: false,
                is_strikethrough: false,
                is_underline: false,
            }],
        })]
    );

    let text = Text {
        value: "foo".to_string(),
        entities: vec![],
    };

    assert_eq!(
        entities(&text),
        [Entity::Semantic(SemanticEntity {
            kind: None,
            value: vec![FormattedText {
                value: "foo".to_string(),
                is_bold: false,
                is_italic: false,
                is_strikethrough: false,
                is_underline: false,
            }],
        })]
    );

    let text = Text {
        value: "🦔 igelkott".to_string(),
        entities: vec![RawEntity {
            kind: EntityKind::Bold,
            offset: 3,
            length: 8,
        }],
    };

    assert_eq!(
        entities(&text),
        [Entity::Semantic(SemanticEntity {
            kind: None,
            value: vec![
                FormattedText {
                    value: "🦔 ".to_string(),
                    is_bold: false,
                    is_italic: false,
                    is_strikethrough: false,
                    is_underline: false,
                },
                FormattedText {
                    value: "igelkott".to_string(),
                    is_bold: true,
                    is_italic: false,
                    is_strikethrough: false,
                    is_underline: false,
                },
            ],
        })]
    );
}
