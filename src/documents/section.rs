use std::{fmt::Display, sync::Arc};

use crate::{TableBuilder, Translation, documents::{Element, ListBuilder, NodeBuilder, Tag}};

#[derive(Debug, Default)]
pub struct Section(NodeBuilder);
//
impl Section {
    pub fn new() -> Self {
        Self::default()
    }
    ///
    /// Добавляем переводы элементу
    pub fn localize(self, t: impl Into<Arc<Translation>>) -> Self {
        Self(self.0.localize(t))
    }
    ///
    /// Add class to the Html element
    pub fn class(self, v: impl Display) -> Self {
        Self(self.0.class(v))
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
    pub fn h1<F>(self, build: F) -> Self
    where
        F: FnOnce(Element) -> Element
    {
        Self(self.0.h1(build))
    }
    ///
    /// Добавляем заголовок H2
    pub fn h2<F>(self, build: F) -> Self
    where
        F: FnOnce(Element) -> Element
    {
        Self(self.0.h2(build))
    }
    ///
    /// Добавляем текст
    pub fn text<F>(self, build: F) -> Self
    where
        F: FnOnce(Element) -> Element
    {
        Self(self.0.text(build))
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
    /// Добавляем таблицу
    pub fn table<F>(self, build: F) -> Self 
    where 
        F: FnOnce(TableBuilder) -> TableBuilder 
    {
        Self(self.0.table(build))
    }
    ///
    /// Возвращает Id 
    pub(super) fn id(&self) -> &str {
        &self.0.id
    }
    ///
    /// Возвращает classes
    pub(super) fn classes(&self) -> &str {
        &self.0.classes
    }
    ///
    /// Возвращает сформированный контент
    pub fn build(self) -> String {
        self.0.build()
    }
}
