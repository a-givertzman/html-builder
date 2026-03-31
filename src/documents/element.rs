use std::fmt::{Display, Write};

use crate::documents::{Tag, write_escaped};

///
/// Atomic Html element builder
#[derive(Debug)]
pub struct Element {
    tag: Tag,
    id: String,
    classes: String,
    child: Vec<Child>,
}
//
impl Element {
    ///
    /// New Html [Element]
    pub fn new(t: Tag) -> Self {
        Self {
            tag: t,
            id: String::new(),
            classes: String::new(),
            child: vec![],
        }
    }
    ///
    /// Добавляем вложенный Html елемент
    pub fn el<F>(mut self, t: Tag, build: F) -> Self 
    where 
        F: FnOnce(Element) -> Element 
    {
        if self.tag.is_void {
            log::warn!("Element.el | Void tag '{}' can't have children", self.tag);
            return self;
        }
        let el: Element = build(Element::new(t));
        self.child.push(Child::El(el));
        self
    }
    ///
    /// Set Id to the Html element
    pub fn id(mut self, v: impl Display) -> Self {
        self.id.clear();
        write_escaped(&mut self.id, v);
        self
    }
    ///
    /// Add class to the Html element
    pub fn class(mut self, v: impl Display) -> Self {
        if !self.classes.is_empty() {
            write!(self.classes, " ").unwrap();
        }
        write_escaped(&mut self.classes, v);
        self
    }
    ///
    /// Add text to the html element 
    pub fn text(mut self, v: impl Display) -> Self {
        let mut text = String::with_capacity(32);
        write_escaped(&mut text, v);
        self.child.push(Child::Text(text));

        self
    }
    ///
    /// Returns rendered html element text
    pub(super) fn build(self) -> String {
        let mut out = String::with_capacity(32);
        write!(out, "<{}", self.tag).unwrap();
        if !self.id.is_empty() {
            write!(out, " id=\"{}\"", self.id).unwrap();
        }
        if !self.classes.is_empty() {
            write!(out, " class=\"{}\"", self.classes).unwrap();
        }
        if self.tag.is_void {
            for child in self.child {
                let text = match child {
                    Child::Text(t) => t,
                    Child::El(el) => el.build(),
                };
                write!(out, "{text}").unwrap();
            }
            write!(out, "/>").unwrap();
        } else {
            write!(out, ">").unwrap();
            for child in self.child {
                let text = match child {
                    Child::Text(t) => t,
                    Child::El(el) => el.build(),
                };
                write!(out, "{text}").unwrap();
            }
            write!(out, "</{}>\n", self.tag).unwrap();
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
