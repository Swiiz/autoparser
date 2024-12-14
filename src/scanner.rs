#[macro_export]
macro_rules! impl_scanner {
    ($($variant:ident $({ $($var:ident: $ty:ty)* })? $(@regex $($is_regex:expr)?)? => $pattern:literal ),* $(,)?) => {
        #[derive(Default, Debug, Clone, Eq, PartialEq)]
        pub enum Token {
            #[default]
            $(
                $variant $({
                            $($var: $ty)*
                        })?,
            )*
        }

        mod regex {
            $(
                $(
                  $($is_regex)?

                  #[allow(non_snake_case)]
                  pub mod $variant {
                      ctreg::regex! { pub Regex = $pattern }
                  }
                )?
            )*
        }

        #[allow(non_snake_case)]
        pub struct Scanner {
            $(
              $(
                $($is_regex)?

                $variant: regex::$variant::Regex,
              )?
            )*
        }

        impl Scanner {
            pub fn new() -> Self {
                Self {
                    $(
                      $(
                          $($is_regex)?

                          $variant: regex::$variant::Regex::new(),
                      )?
                    )*
                }
            }


            pub fn scan(&self, source: $crate::Source) -> $crate::Result<Token, Vec<Token>> {
                let source_len = source.content.len();

                let mut tokens = Vec::new();
                let mut idx = 0;

                while idx < source_len {
                    let mut lookahead = 1;
                    let mut run_regex = false;

                    'scanner: loop {
                        $(
                            $crate::impl_scan! (
                               $(
                                  $($is_regex)?

                                  regex
                               )?

                              $({
                                $($var: $ty)*
                              })?

                              'scanner
                               run_regex
                               $variant
                               $pattern
                               self
                               tokens
                               source
                               idx
                               lookahead
                            );
                        )*
                        const MAX_STATIC_PATTERN_LEN: usize =
                            $crate::max!($(
                                {
                                    #[allow(unused_mut, unused_assignments)]
                                    let mut v = $pattern.len();
                                    $(
                                        $($is_regex)? v = 0;
                                    )?
                                    v
                                }
                            ),*);


                        if idx + lookahead + 1 > source_len || lookahead + 1 > MAX_STATIC_PATTERN_LEN {
                            if !run_regex {
                                run_regex = true;
                                continue;
                            }

                          return Err($crate::ParseError::ScanError {
                              sample: source.content[idx..idx + lookahead].into()
                          });
                        }

                        lookahead += 1;
                    }
                }

                Ok(tokens)
            }
        }

        impl $crate::Parse<Token> for Token {

            fn try_parse(stream: &mut $crate::TokenStream<Token>) -> $crate::Result<Token, $crate::ParseMeta<Token>> {
                Ok($crate::ParseMeta {
                    start: stream.current,
                    end: stream.current + 1,
                    value: stream.advance_by(1).ok_or($crate::ParseError::Eof)?,
                })
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_scan {
    (regex $({ $($var:ident: $ty:ty)* })? $ctx:lifetime $run_regex:tt  $variant:ident $pattern:literal $scanner:tt $tokens:tt $source:tt $idx:tt $lookahead:tt) => {
        if $run_regex {
            let sample = &$source.content[$idx..];
            if let Some(capture) = $scanner.$variant.find(sample) {
                let regex::$variant::RegexCaptures { $($($var,)*)? .. } = $scanner.$variant.captures(sample).unwrap();
                $idx += capture.content.len();
                $tokens.push(Token::$variant $({
                        $($var: <$ty as std::str::FromStr>::from_str($var.content).unwrap())*
                    })?);
                break $ctx;
            }
        }
    };
    ($({ $($var:ident: $ty:ty)* })?  $ctx:lifetime $run_regex:tt $variant:ident $pattern:literal $scanner:tt $tokens:tt $source:tt $idx:tt $lookahead:tt) => {
        let sample = &$source.content[$idx..$idx + $lookahead];
        if sample == $pattern {
            $tokens.push(Token::$variant);
            $idx += $pattern.len();
            break $ctx;
        }
    };
}
