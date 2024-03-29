use std::{str::Chars, fmt::{Formatter, Display, Debug}};
use crate::IRElement;

pub trait Identifiable {
    fn id(&self) -> Identifier;
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct GlobalIdentifier<'s>(pub(crate) &'s str);

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct LocalIdentifier<'s>(&'s str);

#[derive(Debug, Clone)]
pub enum Identifier<'s> {
    Global(GlobalIdentifier<'s>),
    Local(LocalIdentifier<'s>)
}

#[derive(Debug)]
pub enum ParseError {
    NotEnoughTokens,
    UnexpectedToken,
    IllegalToken
}

impl<'s> Display for GlobalIdentifier<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("@{}", self.0))
    }
}
impl<'s> Display for LocalIdentifier<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("%{}", self.0))
    }
}
impl<'s> Display for Identifier<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Identifier::Global(id) => Display::fmt(id, f),
            Identifier::Local(id) => Display::fmt(id, f)
        }
    }
}

impl<'s> IRElement for GlobalIdentifier<'s> {}
impl<'s> IRElement for LocalIdentifier<'s> {}
impl<'s> IRElement for Identifier<'s> {}

//TODO: allow quotes and escapes

impl<'s> Identifier<'s> {
    fn is_char_valid(c: char) -> bool {
        c.is_digit(10) || c.is_alphabetic() || c == '$' || c == '.' || c == '_'
    }
    fn verify_num_ident(chars: &mut Chars) -> Result<(), ParseError> {
        let mut null = true;
        while let Some(c) = chars.next() {
            if null { null = false }
            if !c.is_digit(10) { return Err(ParseError::IllegalToken); }
        }
        if null { return Err(ParseError::NotEnoughTokens); }
        return Ok(());
    }
    fn verify_normal_ident(chars: &mut Chars) -> Result<(), ParseError> {
        let first = chars.next();
        if let Some(first) = first {
            if !Self::is_char_valid(first) { return Err(ParseError::IllegalToken); }
            while let Some(c) = chars.next() {
                if !Self::is_char_valid(c) || !c.is_digit(10) { return Err(ParseError::IllegalToken); }
            }
            return Ok(());
        }
        return Err(ParseError::NotEnoughTokens);
    }
    fn verify_special_ident(chars: &mut Chars) -> Result<(), ParseError> {
        let first = chars.next();
        if let Some(first) = first {
            if first != '"' { return Err(ParseError::IllegalToken); }
            while let Some(c) = chars.next() {
                if c == '"' { 
                    if chars.next().is_none() { return Err(ParseError::IllegalToken); }
                }
                if c == '\\' {
                    'inner:for _ in 0..2 {
                        if let Some(c) = chars.next() {
                            if c.is_digit(16) { continue 'inner; }
                        }
                        return Err(ParseError::IllegalToken);
                    }
                }
            }
            return Ok(());
        }
        return Err(ParseError::NotEnoughTokens);
    }
    fn verify_ident(string: &str) -> Result<(), ParseError> {
        Self::verify_num_ident(&mut string.chars()).or_else(|err| match err {
            ParseError::IllegalToken => Self::verify_normal_ident(&mut string.chars()).or_else(|err| match err {
                ParseError::IllegalToken => Self::verify_special_ident(&mut string.chars()),
                err => Err(err)
            }),
            err => Err(err)
        })
    }
}

impl<'s> From<GlobalIdentifier<'s>> for Identifier<'s> {
    fn from(identifier: GlobalIdentifier<'s>) -> Self {
        Identifier::Global(identifier)
    }
}

impl<'s> From<LocalIdentifier<'s>> for Identifier<'s> {
    fn from(identifier: LocalIdentifier<'s>) -> Self {
        Identifier::Local(identifier)
    }
}

impl<'s> TryFrom<&'s str> for GlobalIdentifier<'s> {
    type Error = ParseError;
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        if s.len() < 3 { return Err(ParseError::NotEnoughTokens); }
        let mut chars = s.chars();
        if chars.next().unwrap() != '@' { return Err(ParseError::UnexpectedToken); }
        Identifier::verify_ident(s)?;
        Ok(GlobalIdentifier(&s[1..]))
    }
}

impl<'s> TryFrom<&'s str> for LocalIdentifier<'s> {
    type Error = ParseError;
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        if s.len() < 3 { return Err(ParseError::NotEnoughTokens); }
        let mut chars = s.chars();
        if chars.next().unwrap() != '%' { return Err(ParseError::UnexpectedToken); }
        Identifier::verify_ident(s)?;
        Ok(LocalIdentifier(&s[1..]))
    }
}

impl<'s> TryFrom<&'s str> for Identifier<'s> {
    type Error = ParseError;
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        GlobalIdentifier::try_from(s).map(|it| Identifier::Global(it)).or_else(|err| match err {
            ParseError::UnexpectedToken => LocalIdentifier::try_from(s).map(|it| Identifier::Local(it)),
            _ => Err(err)
        })
    }
}

