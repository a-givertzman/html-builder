use std::fmt::{Display, Write};

use crate::documents::{Element, Tag, write_escaped};

///
/// Simple Html list
pub struct TableBuilder {
    id: String,
    classes: String,
    header: Vec<Child>,
    rows: Vec<Child>,
}
//
impl TableBuilder {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            classes: String::new(),
            header: vec![],
            rows: vec![],
        }
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
    // ///
    // /// Добавляем Html елемент
    // pub fn el<F>(mut self, t: Tag, build: F) -> Self 
    // where 
    //     F: FnOnce(Element) -> Element 
    // {
    //     let el: Element = build(Element::new(t));
    //     self.content.push(Child::El(el));
    //     self
    // }
    ///
    /// Добавляем шапку таблицы
    pub fn header(mut self, text: impl Display) -> Self {
        self.header.push(Child::Text(text.to_string()));
        self
    }
    /// Добавляем строку таблицы
    pub fn row(mut self, text: impl Display) -> Self {
        self.rows.push(Child::Text(text.to_string()));
        self
    }
    ///
    /// 
    pub fn build(self) -> String {
        let mut out = String::with_capacity(32);
        write!(out, "<table").unwrap();
        if !self.id.is_empty() {
            write!(out, " id=\"{}\"", self.id).unwrap();
        }
        if !self.classes.is_empty() {
            write!(out, " class=\"{}\"", self.classes).unwrap();
        }
        write!(out, ">").unwrap();
        for child in self.rows {
            let text = match child {
                Child::Text(t) => t,
                Child::El(el) => el.build(),
            };
            write!(out, "  <li>{}</li>", text).unwrap();
        }
        write!(out, "</table>").unwrap();
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
