use std::{fmt::{Display, Write}, sync::Arc};

use crate::{TableBuilder, Translation, documents::{Element, ListBuilder, Tag, h1, h2, p}};
///
/// Implements all possible content
#[derive(Debug, Default)]
pub struct NodeBuilder {
    locale: Arc<Translation>,
    pub(super) id: String,
    pub(super) classes: String,
    content: String,
}
//
impl NodeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    ///
    /// Добавляем переводы элементу
    pub fn localize(mut self, t: impl Into<Arc<Translation>>) -> Self {
        self.locale = t.into();
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
    /// Добавляем Html елемент
    pub fn el<F>(mut self, t: Tag, build: F) -> Self 
    where 
        F: FnOnce(Element) -> Element 
    {
        let el = Element::new(t)
            .localize(self.locale.clone());
        let el: Element = build(el);
        write!(self.content, "{}", el.build()).unwrap();
        self
    }
    ///
    /// Добавляем заголовок H1
    pub fn h1<F>(self, build: F) -> Self
    where
        F: FnOnce(Element) -> Element
    {
        self.el(h1(), build)
    }
    ///
    /// Добавляем заголовок H2
    pub fn h2<F>(self, build: F) -> Self
    where
        F: FnOnce(Element) -> Element
    {
        self.el(h2(), build)
    }
    ///
    /// Добавляем текст
    pub fn text<F>(self, build: F) -> Self
    where
        F: FnOnce(Element) -> Element
    {
        self.el(p(), build)
    }
    ///
    /// Добавляем список без номерации
    pub fn list<F>(mut self, build: F) -> Self 
    where 
        F: FnOnce(ListBuilder) -> ListBuilder 
    {
        let list = ListBuilder::new()
            .localize(self.locale.clone());
        let list: ListBuilder = build(list);
        self.content.push_str(&list.build());
        self
    }
    ///
    /// Добавляем таблицу
    pub fn table<F>(mut self, build: F) -> Self 
    where 
        F: FnOnce(TableBuilder) -> TableBuilder 
    {
        let table = TableBuilder::new()
            .localize(self.locale.clone());
        let table: TableBuilder = build(table);
        self.content.push_str(&table.build());
        self
    }
    ///
    /// Возвращает сформированный контент
    pub fn build(self) -> String {
        self.content
    }
}
///
/// Escapes html input
pub(super) fn write_escaped<W: Write>(o: &mut W, v: impl Display) {
    struct Escaper<'a, W: Write> {
        out: &'a mut W,
    }
    impl<W: Write> Write for Escaper<'_, W> {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            for c in s.chars() {
                match c {
                    '<' => self.out.write_str("&lt;")?,
                    '>' => self.out.write_str("&gt;")?,
                    '&' => self.out.write_str("&amp;")?,
                    '"' => self.out.write_str("&quot;")?,
                    '\'' => self.out.write_str("&#39;")?,
                    _ => self.out.write_char(c)?,
                }
            }
            Ok(())
        }
    }
    let mut escaper = Escaper { out: o };
    write!(escaper, "{}", v).unwrap();
}
// ///
// /// Writes escaped html tag
// fn write_tag<W: Write>(
//     out: &mut W,
//     tag: &str,
//     v: impl Display,
// ) {
//     write!(out, "<{}>", tag).unwrap();
//     write_escaped(out, v);
//     write!(out, "</{}>", tag).unwrap();
// }
// ///
// /// Escapes html input
// pub(super) fn escape(input: impl Display) -> String {
//     let mut o = String::new();
//     write_escaped(&mut o, input);
//     o
// }
