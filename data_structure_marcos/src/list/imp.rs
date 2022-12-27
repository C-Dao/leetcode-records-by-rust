use proc_macro2::{TokenStream, TokenTree};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

pub fn from(input: TokenStream) -> TokenStream {
    TokenStream::from_str(transform_input_token_stream(input).as_str()).unwrap()
}

fn transform_input_token_stream(input: TokenStream) -> String {
    if input.is_empty() {
        format!("None")
    } else {
        format!(
            "Some(std::rc::Rc::new(std::cell::RefCell::new({})))",
            ListNode::from(input)
        )
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref next) = self.next {
            write!(f, "ListNode {{ val: {}, next: Some(std::rc::Rc::new(std::cell::RefCell::new({}))) }}",
             self.val, next.as_ref().borrow()
            )
        } else {
            write!(f, "ListNode {{ val: {}, next: None }}", self.val)
        }
    }
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val: val,
            next: None,
        }
    }
}

impl From<TokenStream> for ListNode {
    fn from(stream: TokenStream) -> Self {
        let mut dummy = Some(Rc::new(RefCell::new(ListNode::new(0))));
        let mut cur = dummy.clone();

        for token in stream.into_iter() {
            match token {
                TokenTree::Group(group) => return Self::from(group.stream()),
                TokenTree::Literal(literal) => {
                    let val = literal.to_string().parse().unwrap();
                    let node = Some(Rc::new(RefCell::new(ListNode::new(val))));
                    cur.as_mut().unwrap().borrow_mut().next = node.clone();
                    cur = node;
                }
                _ => {}
            }
        }

        
        let next = dummy.as_mut().unwrap().borrow_mut().next.take();

        drop(cur);

        Rc::try_unwrap(next.unwrap()).unwrap().into_inner()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_group_token() {
        let token_stream = TokenStream::from_str("[1,2,3]").unwrap();
        let single_list = self::from(token_stream);
        dbg!(single_list);
    }

    #[test]
    fn test_with_literal_token() {
        let token_stream = TokenStream::from_str("1,2").unwrap();
        let single_list = self::from(token_stream);
        dbg!(single_list);
    }

    #[test]
    fn test_with_single_literal_token() {
        let token_stream = TokenStream::from_str("1").unwrap();
        let single_list = self::from(token_stream);
        dbg!(single_list);
    }

    #[test]
    fn test_with_empty_token() {
        let token_stream = TokenStream::from_str("").unwrap();
        let single_list = self::from(token_stream);
        dbg!(single_list);
    }
}
