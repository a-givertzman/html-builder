use std::fmt::Display;

use crate::documents::{ListBuilder, NodeBuilder};

#[derive(Debug, Default)]
pub struct Footer(NodeBuilder);
//
impl Footer {
    pub fn new() -> Self {
        Self::default()
    }
    ///
    /// Добавляем заголовок H1
    pub fn add_h1(self, v: &str) -> Self {
        Self(self.0.add_h1(v))
    }
    ///
    /// Добавляем заголовок H2
    pub fn add_h2(self, v: &str) -> Self {
        Self(self.0.add_h2(v))
    }
    ///
    /// Добавляем текст
    pub fn add_text(self, v: impl Display) -> Self {
        Self(self.0.add_text(v))
    }
    ///
    /// Добавляем список без номерации
    pub fn add_list<F>(self, build: F) -> Self 
    where 
        F: FnOnce(ListBuilder) -> ListBuilder 
    {
        Footer(self.0.add_list(build))
    }
    ///
    /// Возвращает сформированный контент
    pub fn build(self) -> String {
        self.0.build()
    }
}
