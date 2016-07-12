#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Atom(String),
    List(Vec<Token>),
}

fn create_tokens(mut chars: &mut Vec<char>, mut tokens: &mut Vec<Token>) {
    while chars.len() > 0 {
        let token = chars.remove(0);
        if token == '(' {
            let mut list = vec![];
            create_tokens(&mut chars, &mut list);
            tokens.push(Token::List(list));
        } else if token == ')' {
            return;
        } else if token == ';' {
            while chars.len() > 0 {
                if chars.remove(0) == '\n' {
                    break;
                }
            }
        } else if !token.is_whitespace() {
            let mut in_string = false;
            if token == '"' {
                in_string = true;
            }

            let mut literal = vec![token];
            while chars.len() > 0 {
                if !in_string && (chars[0].is_whitespace() || chars[0] == '(' || chars[0] == ')') {
                    break;
                }
                if chars[0] == '"' {
                    in_string = !in_string;
                }
                literal.push(chars.remove(0));
            }
            tokens.push(Token::Atom(literal.into_iter().collect()));
        }
    }
}

pub fn tokenise(program: String) -> Vec<Token> {
    let mut tokens = vec![];

    if program.len() == 0 {
        return tokens;
    }

    let mut chars = vec![];
    for c in program.chars() {
        chars.push(c);
    }

    create_tokens(&mut chars, &mut tokens);

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_list() -> Token {
        Token::List(vec![])
    }

    #[test]
    fn test_tokenise() {
        assert_eq!(tokenise("()".to_string()), vec![empty_list()]);
        assert_eq!(tokenise("() ()".to_string()),
                   vec![empty_list(), empty_list()]);
        assert_eq!(tokenise("(())".to_string()),
                   vec![Token::List(vec![empty_list()])]);
        assert_eq!(tokenise("(print \"()\")".to_string()),
                   vec![Token::List(vec![Token::Atom("print".to_string()),
                                         Token::Atom("\"()\"".to_string())])]);
        assert_eq!(tokenise("; This is a comment".to_string()), vec![]);
        assert_eq!(tokenise("; This is a comment
                             (this-is-code)"
                       .to_string()),
                   vec![Token::List(vec![Token::Atom("this-is-code".to_string())])]);
    }
}
