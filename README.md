<h1 align=center>🤖 Autoparser 💬</h1>

## Rust library to easily generate *(Recursive Descent)* Parser using macros.

This can be used to generate parsers for any language easily such as programming, markup, etc.
All the parser generation logic (+regex building) is executed at **compile time, ensuring zero runtime overhead**.

## Some context

While reading [Crafting Interpreters](craftinginterpreters.com) and implementing my own programming language in Rust, I explored ways to make parser creation more efficient as outlined in the book. Writing parsers can be quite repetitive, especially for verbose languages, so automating parts of this process can save significant effort.

In the book, the author uses Java and demonstrates how to write code that generates Java parser code: a form of [metaprogramming](https://en.wikipedia.org/wiki/Metaprogramming#:~:text=Metaprogramming%20is%20a%20computer%20programming,even%20modify%20itself%2C%20while%20running.). Since I’m working with Rust, I’ve opted to leverage **Rust’s powerful macro as well as type system** to achieve a similar result. To that end, I’ve developed a library that uses macros to generate parsers.

However, I didn’t stop at replicating the simple approach of the book. And went for a solution to generate automatically the Scanner, [Abstract-Syntax-Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree), and Parser logic.

## Usage

- Add the dependency to your `Cargo.toml` file:
```toml
[dependencies]
autoparser = { git = "https://github.com/Swiiz/autoparser" }
```

- Define your tokens, this will generate a `Scanner` struct and a `Token` enum.
```rust
autoparser::impl_scanner! {
  Whitespace @regex => "^(?<__>\\s)", // All regexes need to have a named capture group.

  Minus => "-",
  Plus => "+",

  NumberLiteral { number: u32 } @regex => "^(?<number>(\\d)+)", // Named capture group can be used as data in Token.
}
```

- Define your grammar, each rules will define struct representing a node in the Abstract Syntax Tree. Rules are declared in [Order of precedence](https://en.wikipedia.org/wiki/Order_of_operations).
```rust
autoparser::impl_rules! {
// For performance reason you don't want to parse (.., <Token>, <Rule, ..) as the Rule. The parser can stop early based on types, you may only use Rules : (.., <TokenRule>, <Rule>, ..)
  AddOperator => Token::Plus, 
  SubOperator => Token::Minus,

// On the right side of the `=>` you can use any match pattern. The type of the provided pattern will be the parsed.
// Then your pattern will be tested. You can use @, if and more... for data manipulation, see rust match-pattern docs.
  Literal { number: u32 } => Token::NumberLiteral { number },

// Rules can also be an union of rules using the `enum` keyword and the `|` operator.
  enum Expr => AddOperation | SubOperation | Unary,
// Rules can be recursive, however you need to use the Box<T> type to avoid infinite ast node size.
  AddOperation { left: Factor, right: Box<Expr> } => (left, AddOperator {}, right),
  SubOperation { left: Factor, right: Box<Expr> } => (left, SubOperator {}, right),

  enum Unary => InverseOperation | Literal,
  InverseOperation { literal: Literal } => (SubOperator {}, literal),
  Literal { number: u32 } => Token::NumberLiteral { number },
}
```	

The `Token` enum, `Scanner` struct and each AST Node can now be used together:
```rust
  let source = autoparser::Source {
      name: None,
      content: "1 + 2 - 3".into(),
  };
  let scanner = Scanner::new();

  let scan = scanner
      .scan(source)
      .into_iter()
      .filter(|t| t != &Token::Whitespace)
      .collect::<Vec<_>>();

  let mut tokens = autoparser::TokenStream::new(&scan);
  println!("{#?}", Expr::try_parse(&mut tokens));
```


## Example(s)

- ### [Calculator](https://github.com/Swiiz/autoparser/tree/master/examples/calculator.rs)
  **Supporting -, +, \*, /, parenthesis, variables and operator priority in <100 LOC.**

  Run the example:
  ```
  cargo run --example calculator
  ```
  See the generated code documentation:
  ```
  cargo doc --example calculator --no-deps --open 
  ```

## How does it work?

- **impl_scanner**
   - The scanner scans the source code for the token strings and regexes. The scanner will always match static strings before trying to match regexes. Return a Vec of tokens.
   - Each token is defined as a variant on the Token enum.
   - `Parse<Self>` is implemented for the newly created Token enum.
  
- **impl_scanner**
    - Generates a struct/enum for each rule with it's data. (representing the ast node)
    - `Parse<Token>` is implemented for each node struct/enum, it first matches the rule, then the data.
    - Composite rules can be created using tuples: (A, B, C, ..), in contrast to enums the all of the children rules need to be matched for it to succeed.

****************************

> [!NOTE]
> This is a work in progress. Error reporting needs to be improved.
> 
> Feel free to open an issue or contribute!
