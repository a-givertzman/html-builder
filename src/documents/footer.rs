use crate::documents::{Element, ListBuilder, NodeBuilder, Tag};

#[derive(Debug, Default)]
pub struct Footer(NodeBuilder);
//
impl Footer {
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
        Footer(self.0.list(build))
    }
    ///
    /// Возвращает Id 
    pub fn id(&self) -> &str {
        &self.0.id
    }
    ///
    /// Возвращает classes
    pub fn classes(&self) -> &str {
        &self.0.classes
    }
    ///
    /// Возвращает сформированный контент
    pub fn build(self) -> String {
        self.0.build()
    }
}
