use std::{fmt::{Display, Write}, sync::Arc};

use crate::{Attribute, Translation, documents::{Tag, write_escaped}};

///
/// Atomic Html element builder
#[derive(Debug)]
pub struct Element {
    locale: Arc<Translation>,
    tag: Tag,
    id: String,
    classes: String,
    attrs: String,
    child: Vec<Child>,
}
//
impl Element {
    ///
    /// New Html [Element]
    pub fn new(t: Tag) -> Self {
        Self {
            locale: Arc::new(Translation::empty()),
            tag: t,
            id: String::new(),
            classes: String::new(),
            attrs: String::new(),
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
        let el = Element::new(t)
            .localize(self.locale.clone());
        let el: Element = build(el);
        self.child.push(Child::El(el));
        self
    }
    ///
    /// Добавляем переводы элементу
    pub fn localize(mut self, t: impl Into<Arc<Translation>>) -> Self {
        self.locale = t.into();
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
    /// Add attribute to the Html element
    pub fn attr(mut self, attr: Attribute, value: impl Display) -> Self {
        if !self.attrs.is_empty() {
            write!(self.attrs, " ").unwrap();
        }
        if attr.is_flag {
            write!(self.attrs, "{}", attr).unwrap();
        } else {
            write!(self.attrs, "{}=\"", attr).unwrap();
            match attr.name {
                "title" | "placeholder" | "value" => write_escaped(&mut self.attrs, self.locale.tr(&value)),
                _ => write_escaped(&mut self.attrs, value),
            }
            write!(self.attrs, "\"").unwrap();
        }
        self
    }
    ///
    /// Add text to the html element 
    pub fn text(mut self, v: impl Display) -> Self {
        let mut text = String::with_capacity(32);
        write_escaped(&mut text, self.locale.tr(&v));
        self.child.push(Child::Text(text));
        self
    }
    ///
    /// Add raw text to the html element 
    pub fn raw(mut self, v: impl Display) -> Self {
        self.child.push(Child::Text(v.to_string()));
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
        if !self.attrs.is_empty() {
            write!(out, " {}", self.attrs).unwrap();
        }
        if self.tag.is_void {
            for child in self.child {
                match child {
                    Child::Text(_) => log::warn!("Element.build | Void tag '{}' can't contains text", self.tag),
                    Child::El(_) => log::warn!("Element.build | Void tag '{}' can't have children", self.tag),
                }
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
            write!(out, "</{}>", self.tag).unwrap();
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
