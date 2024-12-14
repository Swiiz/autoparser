#[macro_export]
macro_rules! impl_rules {
    ($variant:ident $({$( $var:ident: $ty:ty),*})? => $pattern:pat , $($rest:tt)*) => {
        #[derive(Debug)]
        pub struct $variant {
            $($(pub $var: $ty),*)?
        }

        impl $crate::Parse<Token> for $variant {
            fn try_parse(tokens: &mut $crate::TokenStream<Token>) -> $crate::Result<Token, $crate::ParseMeta<Self>> {
                match (tokens.clone()).try_parse()? {
                    $crate::ParseMeta { start, end, value: $pattern } => {
                        tokens.advance_by(end - start);
                        Ok($crate::ParseMeta {
                            start,
                            end,
                            value: $variant {
                                $($( $var ),*)?
                            }
                        })
                    }


                    #[allow(unreachable_patterns)]
                    _ => Err($crate::ParseError::UnexpectedToken {
                        token: tokens.peek().ok_or($crate::ParseError::Eof)?,
                    })
                }
            }
        }

        $crate::impl_rules! { $($rest)* }
    };

    (enum $variant:ident => $first:tt $(| $next:tt)* , $($rest:tt)*) => {

        #[derive(Debug)]
        pub enum $variant {
            $first ($first),
            $($next ($next),)*
        }

        impl $crate::Parse<Token> for $variant  {
            fn try_parse(tokens: &mut $crate::TokenStream<Token>) -> $crate::Result<Token, $crate::ParseMeta<Self>> {
                if let Ok($crate::ParseMeta {
                    start,
                    end,
                    value
                }) = (tokens.clone()).try_parse::<$first>() {
                    return Ok($crate::ParseMeta {
                        start,
                        end,
                        value: Self::$first(value)
                    });
                };

                $(
                    if let Ok($crate::ParseMeta {
                        start,
                        end,
                        value
                    }) = (tokens.clone()).try_parse::<$next>() {
                        return Ok($crate::ParseMeta {
                            start,
                            end,
                            value: Self::$next(value)
                        });
                    };
                )*

                Err($crate::ParseError::UnexpectedToken {
                    token: tokens.peek().ok_or($crate::ParseError::Eof)?,
                })
            }
        }

        $crate::impl_rules! { $($rest)* }
    };

    () => {};
}

macro_rules! impl_grammar_rule_for_tuples {
    ($first:ident $(, $name:ident)*) => {
        impl<Tok: Clone, $first: super::Parse<Tok> $(, $name: super::Parse<Tok>)*> super::Parse<Tok> for ($first, $($name,)*) {

            #[allow(non_snake_case, unused_variables, irrefutable_let_patterns)]
            fn try_parse(tokens: &mut super::TokenStream<Tok>) -> $crate::Result<Tok, super::ParseMeta<Self>> {
                use $crate::ParseError;

                let super::ParseMeta {
                    start,
                    end,
                    value: $first
                } = (tokens.clone()).try_parse()? else {
                    return Err(ParseError::UnexpectedToken {
                        token: tokens.peek().ok_or(ParseError::Eof)?,
                    });
                };
                let last_start = start;

                $(
                    let t = tokens.advance_by(end - last_start);

                    let super::ParseMeta {
                        start: last_start,
                        end,
                        value: $name
                    } = (tokens.clone()).try_parse()? else {
                        return Err(ParseError::UnexpectedToken {
                            token: tokens.peek().ok_or(ParseError::Eof)?,
                        })
                    };
                )*

                Ok(super::ParseMeta {
                    start,
                    end,
                    value: ($first, $($name,)*)
                })
            }
        }
    };
}

impl_grammar_rule_for_tuples!(A);
impl_grammar_rule_for_tuples!(A, B);
impl_grammar_rule_for_tuples!(A, B, C);
impl_grammar_rule_for_tuples!(A, B, C, D);
impl_grammar_rule_for_tuples!(A, B, C, D, E);
impl_grammar_rule_for_tuples!(A, B, C, D, E, F);
impl_grammar_rule_for_tuples!(A, B, C, D, E, F, G);
impl_grammar_rule_for_tuples!(A, B, C, D, E, F, G, H);
impl_grammar_rule_for_tuples!(A, B, C, D, E, F, G, H, I);
impl_grammar_rule_for_tuples!(A, B, C, D, E, F, G, H, I, J);
impl_grammar_rule_for_tuples!(A, B, C, D, E, F, G, H, I, J, K);
impl_grammar_rule_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);
