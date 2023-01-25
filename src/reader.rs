use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Expression {
	Number(f64),
	Bool(bool),
	Str(String),
	Symbol(String),
	List(Vec<Expression>), 
	Null,
}

impl From<String> for Expression {
	fn from(value: String) -> Self {
		match value.parse::<f64>() {
			Ok(n) => Self::Number(n),
			Err(_) => {
				match value.as_str() {
					"true" => Expression::Bool(true),
					"false" => Expression::Bool(false),
					_ => {
						let front = value.chars().nth(0).unwrap();
						let last = value.chars().last().unwrap();
						if front == last && front == '"' {
							let mut chars = value.chars();
							chars.next();
							chars.next_back();
							return Self::Str(chars.collect())
						}else if value != "" && value != "null" {
							return  Self::Symbol(value.to_string());
						}else{
							return Self::Null;
						}
					}
				}
			}
		}
	}
}

fn tokenize(src: &str) -> VecDeque<String> {
	src.replace("(", " ( ").replace(")", " ) ").replace("'", " ' ").split_whitespace().map(|s| s.to_string()).collect()
}

fn parse<'a>(tokens: &'a mut VecDeque<String>) -> Expression {
	let token: String = tokens.pop_front().expect("Unexpected EOF");
	match token.as_str() {
		"(" => {
			let mut stack: Vec<Expression> = Vec::new();
			while tokens[0] != ")" {
				stack.push(parse(tokens));
			}
			tokens.pop_front();
			return Expression::List(stack);
		},
		")" => panic!("unexpected )"),
		_ => Expression::from(token.to_string()),
	}
}

pub fn read(src: &str) -> Expression {
	parse(&mut tokenize(src))
}

#[test]
fn tokenize_test() {
	assert_eq!(tokenize("this is a test"), vec!["this", "is", "a", "test"]);
}

#[test]
fn read_test() {
	println!("{:?}", read("(1 symbol \"string\" true null (1.5))"));
	assert_eq!(read("(1 symbol \"string\" true null (1.5))"), Expression::List(vec![Expression::Number(1f64), Expression::Symbol("symbol".to_string()), Expression::Str("string".to_string()), Expression::Bool(true), Expression::Null, Expression::List(vec![Expression::Number(1.5)])]));
}
