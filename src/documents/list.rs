use std::fmt::{Display, Write};

use crate::documents::{Element, Tag};

///
/// Simple Html list
pub struct ListBuilder {
    content: Vec<Child>,
}
//
impl ListBuilder {
    pub fn new() -> Self {
        Self {
            content: vec![],
        }
    }
    ///
    /// Добавляем Html елемент
    pub fn el<F>(mut self, t: Tag, build: F) -> Self 
    where 
        F: FnOnce(Element) -> Element 
    {
        let el: Element = build(Element::new(t));
        self.content.push(Child::El(el));
        self
    }
    ///
    /// Добавляем элемент списка
    pub fn item(mut self, text: impl Display) -> Self {
        self.content.push(Child::Text(text.to_string()));
        self
    }
    ///
    /// 
    pub fn build(self) -> String {
        let mut out = String::with_capacity(32);
        for child in self.content {
            let text = match child {
                Child::Text(t) => t,
                Child::El(el) => el.build(),
            };
            write!(out, "  <li>{}</li>\n", text).unwrap();
        }
        out
    }
}
///
/// Child variants
#[derive(Debug)]
enum Child {
    Text(String),
    El(Element),
}
