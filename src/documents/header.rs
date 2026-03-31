use std::fmt::Display;

use crate::documents::{Element, ListBuilder, NodeBuilder, Tag};

#[derive(Debug, Default)]
pub struct Header(NodeBuilder);
//
impl Header {
    pub fn new() -> Self {
        Self::default()
    }
    ///
    /// Добавляем Html елемент
    pub fn el<F>(self, t: Tag, build: F) -> Self 
    where 
        F: FnOnce(Element) -> Element 
    {
        Self(self.0.el(t, build))
    }
    ///
    /// Добавляем заголовок H1
    pub fn h1(self, v: &str) -> Self {
        Self(self.0.h1(v))
    }
    ///
    /// Добавляем заголовок H2
    pub fn h2(self, v: &str) -> Self {
        Self(self.0.h2(v))
    }
    ///
    /// Добавляем текст
    pub fn text(self, v: impl Display) -> Self {
        Self(self.0.text(v))
    }
    ///
    /// Добавляем список без номерации
    pub fn list<F>(self, build: F) -> Self 
    where 
        F: FnOnce(ListBuilder) -> ListBuilder 
    {
        Self(self.0.list(build))
    }
    ///
    /// Возвращает сформированный контент
    pub fn build(self) -> String {
        self.0.build()
    }
}
