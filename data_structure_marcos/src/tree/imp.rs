use proc_macro2::{TokenStream, TokenTree};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::str::FromStr;

pub fn from(input: TokenStream) -> TokenStream {
    TokenStream::from_str(transform_input_token_stream(input).as_str()).unwrap()
}

fn transform_input_token_stream(input: TokenStream) -> String {
    if input.is_empty() {
        format!("None")
    } else {
        format!(
            "Some(std::rc::Rc::new(std::cell::RefCell::new({})))",
            TreeNode::from(input)
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TreeNode {{ val: {}, ", self.val)?;

        if let Some(ref left) = self.left {
            write!(
                f,
                "left: Some(std::rc::Rc::new(std::cell::RefCell::new({}))), ",
                left.as_ref().borrow()
            )?;
        } else {
            write!(f, "left: None, ")?;
        };

        if let Some(ref right) = self.right {
            write!(
                f,
                "right: Some(std::rc::Rc::new(std::cell::RefCell::new({}))), ",
                right.as_ref().borrow()
            )?;
        } else {
            write!(f, "right: None, ")?;
        };

        write!(f, "}}")
    }
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val: val,
            left: None,
            right: None,
        }
    }
}

impl From<TokenStream> for TreeNode {
    fn from(stream: TokenStream) -> Self {
        let mut queue = VecDeque::new();
        let mut tree_root = None;
        let mut null_count = 0;

        let mut append_node = |node: Option<Rc<RefCell<TreeNode>>>| match node {
            Some(rc_node) => {
                if tree_root.is_none() {
                    tree_root = Some(rc_node.clone());
                    queue.push_back(rc_node);
                } else {
                    let parent = queue.front().unwrap().clone();
                    if parent.borrow().left.is_none() && (null_count == 0) {
                        queue.push_back(rc_node.clone());
                        parent.borrow_mut().left = Some(rc_node);
                    } else if parent.borrow().right.is_none() && (null_count & 2 == 0) {
                        queue.push_back(rc_node.clone());
                        parent.borrow_mut().right = Some(rc_node);
                        queue.pop_front();
                        null_count = 0;
                    }
                }
            }
            None => {
                null_count += 1;
                let parent = queue.front().unwrap().clone();
                if parent.borrow().left.is_none() && null_count == 2 {
                    queue.pop_front();
                    null_count = 0;
                } else if parent.borrow().left.is_some()
                    && parent.borrow().right.is_none()
                    && null_count == 1
                {
                    queue.pop_front();
                    null_count = 0;
                }
            }
        };

        for token in stream.into_iter() {
            match token {
                TokenTree::Group(group) => return Self::from(group.stream()),
                TokenTree::Literal(literal) => {
                    let val = literal.to_string().parse().unwrap();
                    let node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    append_node(node);
                }
                TokenTree::Ident(ident) => {
                    if ident == "null" {
                        append_node(None);
                    }
                }
                _ => {}
            }
        }

        drop(queue);

        Rc::try_unwrap(tree_root.unwrap()).unwrap().into_inner()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_group_token() {
        let token_stream = TokenStream::from_str("[1,2,3]").unwrap();
        let binary_tree = self::from(token_stream);
        dbg!(binary_tree);
    }

    #[test]
    fn test_with_literal_token() {
        let token_stream = TokenStream::from_str("1,3,2,null,null,null,3").unwrap();
        let binary_tree = self::from(token_stream);
        dbg!(binary_tree);
    }

    #[test]
    fn test_with_single_literal_token() {
        let token_stream = TokenStream::from_str("1").unwrap();
        let binary_tree = self::from(token_stream);
        dbg!(binary_tree);
    }

    #[test]
    fn test_with_empty_token() {
        let token_stream = TokenStream::from_str("").unwrap();
        let binary_tree = self::from(token_stream);
        dbg!(binary_tree);
    }
}
