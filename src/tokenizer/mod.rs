use crate::tokens::constants::misc::{A_TEXT, G_NUMALLOWED};
use crate::tokens::constants::TokenPattern;
use crate::tokens::mappings::{
    get_accent_mappings, get_arrow_mapping, get_font_mappings, get_greek_mappings,
    get_grouping_mappings, get_logical_mappings, get_misc_mappings, get_operation_mappings,
    get_relation_mapping,
};
use crate::tokens::{
    Accent, Arrow, FontCommand, Greek, Grouping, Logical, Misc, Operation, Relation, Text, Token,
};
use charred::tapemachine::CharTapeMachine;
use std::collections::HashMap;

pub struct Tokenizer {
    ctm: CharTapeMachine,
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        let mut chars = text.chars().collect::<Vec<char>>();
        chars.push('\n');
        Self {
            ctm: CharTapeMachine::new(chars),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();
        self.ctm.seek_whitespace();

        while !self.ctm.check_eof() {
            if let Some(grouping) = self.parse_grouping() {
                tokens.push(Token::Grouping(grouping))
            } else if let Some(arrow) = self.parse_arrows() {
                tokens.push(Token::Arrow(arrow))
            } else if let Some(relation) = self.parse_relation() {
                tokens.push(Token::Relation(relation))
            } else if let Some(operation) = self.parse_operation() {
                tokens.push(Token::Operation(operation))
            } else if let Some(misc) = self.parse_misc() {
                tokens.push(Token::Misc(misc))
            } else if let Some(logical) = self.parse_logical() {
                tokens.push(Token::Logical(logical))
            } else if let Some(accent) = self.parse_accent() {
                tokens.push(Token::Accent(accent))
            } else if let Some(greek) = self.parse_greek() {
                tokens.push(Token::Greek(greek))
            } else if let Some(font) = self.parse_font_command() {
                tokens.push(Token::Font(font))
            } else if let Some(whitespace) = self.parse_whitespace() {
                tokens.push(Token::Text(whitespace))
            } else if let Some(text) = self.parse_text() {
                tokens.push(Token::Text(text))
            } else if let Some(number) = self.parse_number() {
                tokens.push(Token::Text(number))
            } else {
                tokens.push(Token::Text(Text::Symbol(
                    self.ctm.get_current().to_string(),
                )))
            }
            let _ = self.ctm.seek_one();
        }
        // stripping the whitespace at the end
        if let Some(Token::Text(Text::Whitespace)) = tokens.last() {
            tokens.pop().unwrap();
        }

        tokens
    }

    fn parse_misc(&mut self) -> Option<Misc> {
        lazy_static! {
            static ref MISC_MAPPINGS: Vec<HashMap<TokenPattern, Misc>> = get_misc_mappings();
        }
        for mapping in MISC_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_operation(&mut self) -> Option<Operation> {
        lazy_static! {
            static ref OPERATION_MAPPINGS: Vec<HashMap<TokenPattern, Operation>> =
                get_operation_mappings();
        }
        for mapping in OPERATION_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_relation(&mut self) -> Option<Relation> {
        lazy_static! {
            static ref RELATION_MAPPINGS: Vec<HashMap<TokenPattern, Relation>> =
                get_relation_mapping();
        }
        for mapping in RELATION_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_logical(&mut self) -> Option<Logical> {
        lazy_static! {
            static ref LOGICAL_MAPPINGS: Vec<HashMap<TokenPattern, Logical>> =
                get_logical_mappings();
        }
        for mapping in LOGICAL_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_grouping(&mut self) -> Option<Grouping> {
        lazy_static! {
            static ref GROUPING_MAPPINGS: Vec<HashMap<TokenPattern, Grouping>> =
                get_grouping_mappings();
        }
        for mapping in GROUPING_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_arrows(&mut self) -> Option<Arrow> {
        lazy_static! {
            static ref ARROW_MAPPINGS: Vec<HashMap<TokenPattern, Arrow>> = get_arrow_mapping();
        }
        for mapping in ARROW_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_accent(&mut self) -> Option<Accent> {
        lazy_static! {
            static ref ACCENT_MAPPINGS: Vec<HashMap<TokenPattern, Accent>> = get_accent_mappings();
        }
        for mapping in ACCENT_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_greek(&mut self) -> Option<Greek> {
        lazy_static! {
            static ref GREEK_MAPPINGS: Vec<HashMap<TokenPattern, Greek>> = get_greek_mappings();
        }
        for mapping in GREEK_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_font_command(&mut self) -> Option<FontCommand> {
        lazy_static! {
            static ref FONTC_MAPPING: Vec<HashMap<&'static str, FontCommand>> = get_font_mappings();
        }
        for mapping in FONTC_MAPPING.iter() {
            for key in mapping.keys() {
                if self.ctm.check_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_whitespace(&mut self) -> Option<Text> {
        if self.ctm.get_current().is_whitespace() {
            self.ctm.seek_whitespace();
            self.ctm.rewind(self.ctm.get_index() - 1);
            Some(Text::Whitespace)
        } else {
            None
        }
    }

    fn parse_text(&mut self) -> Option<Text> {
        if self.ctm.check_char(&A_TEXT) {
            let mut string = String::new();

            while let Some(ch) = self.ctm.next_char() {
                if ch == A_TEXT {
                    break;
                }
                string.push(ch);
            }
            Some(Text::Plain(string))
        } else {
            None
        }
    }

    fn parse_number(&mut self) -> Option<Text> {
        if self.ctm.get_current().is_numeric() {
            let mut string = self.ctm.get_current().to_string();

            while let Some(ch) = self.ctm.next_char() {
                if ch.is_numeric() || self.ctm.check_any(&G_NUMALLOWED) {
                    string.push(ch);
                } else {
                    break;
                }
            }
            self.ctm.rewind(self.ctm.get_index() - 1);
            Some(Text::Number(string))
        } else {
            None
        }
    }
}
