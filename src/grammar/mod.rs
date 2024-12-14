mod macros;

/*
    Current impl drawback:
    matching Token::* doesnt stop parsing the rest of list on failure.
    Currently you need to create a Rule to check for the `Token Variant`.
    This is because Token is a struct and the parse "stop logic" works on the type level.
    /!\ Mixing Rules and Tokens inside a rule might lead to performance problems.
*/

#[derive(Clone)]
pub struct TokenStream<'a, Tok: Clone> {
    tokens: &'a Vec<Tok>,
    pub current: usize,
}

impl<'a, Tok: Clone> TokenStream<'a, Tok> {
    pub fn new(tokens: &'a Vec<Tok>) -> TokenStream<'a, Tok> {
        TokenStream { tokens, current: 0 }
    }

    pub fn peek(&self) -> Option<Tok> {
        self.tokens.get(self.current).cloned()
    }

    pub fn advance_by(&mut self, n: usize) -> Option<Tok> {
        let t = self.peek();
        self.current += n;
        t
    }

    /// Runs [`Parse::try_parse`] whith T as Self
    pub fn try_parse<T: Parse<Tok>>(&mut self) -> crate::Result<Tok, ParseMeta<T>> {
        T::try_parse(self)
    }
}

impl<'a, T: Clone> From<&'a Vec<T>> for TokenStream<'a, T> {
    fn from(tokens: &'a Vec<T>) -> Self {
        TokenStream::new(tokens)
    }
}

#[derive(Debug)]
pub struct ParseMeta<T> {
    pub start: usize,
    pub end: usize,
    pub value: T,
}

impl<T> ParseMeta<T> {
    pub fn box_value(self) -> ParseMeta<Box<T>> {
        ParseMeta {
            start: self.start,
            end: self.end,
            value: Box::new(self.value),
        }
    }
}

pub trait Parse<Token: Clone>: Sized + std::fmt::Debug {
    /// Tries to parse the given grammar rule from the current position in the TokenStream.
    ///
    /// Returns Ok(ParseMeta<T>) if the rule is successfully parsed.
    /// start and end are the start and end positions of the parsed rule as token indices in the TokenStream.
    /// value is the value returned by the rule.
    ///
    /// Returns Err(ParseError) if the rule fails to parse.
    ///
    ///TODO: Impl parse whole input?!
    fn try_parse(stream: &mut TokenStream<Token>) -> crate::Result<Token, ParseMeta<Self>>;
}

impl<Tok: Clone, T: Parse<Tok>> Parse<Tok> for Box<T> {
    fn try_parse(tokens: &mut TokenStream<Tok>) -> crate::Result<Tok, ParseMeta<Self>> {
        T::try_parse(tokens).map(ParseMeta::box_value)
    }
}

impl<Tok: Clone> Parse<Tok> for () {
    fn try_parse(tokens: &mut TokenStream<Tok>) -> crate::Result<Tok, ParseMeta<Self>> {
        Ok(ParseMeta {
            start: tokens.current,
            end: tokens.current,
            value: (),
        })
    }
}

#[derive(Debug)]
pub struct NoToken;
impl<Tok: Clone> Parse<Tok> for NoToken {
    fn try_parse(tokens: &mut TokenStream<Tok>) -> crate::Result<Tok, ParseMeta<Self>> {
        Ok(ParseMeta {
            start: tokens.current,
            end: tokens.current,
            value: NoToken,
        })
    }
}
