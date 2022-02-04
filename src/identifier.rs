use std::str::Chars;

#[repr(transparent)]
pub struct GlobalIdentifier<'s>(&'s str);

#[repr(transparent)]
pub struct LocalIdentifier<'s>(&'s str);

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

//TODO: allow quotes and escapes

impl<'s> Identifier<'s> {
    fn is_char_valid(c: char) -> bool {
        c.is_digit(10) || c.is_alphabetic() || c == '$' || c == '.' || c == '_'
    }
    fn is_string_valid(chars: &mut Chars) -> bool {
        let first = chars.next().unwrap();
        if first.is_digit(10) {
            while let Some(next) = chars.next() {
                if !next.is_digit(10) { return false; }
            }
            return true;
        } else if Self::is_char_valid(first) {
            while let Some(next) = chars.next() {
                if !Self::is_char_valid(next) { return false; }
            }
            return true;
        } else { return false; }
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
        if !Identifier::is_string_valid(&mut chars) { return Err(ParseError::IllegalToken) }
        Ok(GlobalIdentifier(&s[1..]))
    }
}

impl<'s> TryFrom<&'s str> for LocalIdentifier<'s> {
    type Error = ParseError;
    fn try_from(s: &'s str) -> Result<Self, Self::Error> {
        if s.len() < 3 { return Err(ParseError::NotEnoughTokens); }
        let mut chars = s.chars();
        if chars.next().unwrap() != '%' { return Err(ParseError::UnexpectedToken); }
        if !Identifier::is_string_valid(&mut chars) { return Err(ParseError::IllegalToken); }
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

