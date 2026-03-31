use std::fmt::{Display, Write};

use crate::documents::{Element, ListBuilder, Tag};
///
/// Implements all possible content
#[derive(Debug, Default)]
pub struct NodeBuilder {
    content: String,
}
//
impl NodeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    ///
    /// Добавляем Html елемент
    pub fn el<F>(mut self, t: Tag, build: F) -> Self 
    where 
        F: FnOnce(Element) -> Element 
    {
        let el: Element = build(Element::new(t));
        write!(self.content, "{}\n", el.build()).unwrap();
        self
    }
    ///
    /// Добавляем заголовок H1
    pub fn h1(mut self, v: impl Display) -> Self {
        write_tag(&mut self.content, "h1", v);
        self
    }
    ///
    /// Добавляем заголовок H2
    pub fn h2(mut self, v: impl Display) -> Self {
        write_tag(&mut self.content, "h2", v);
        self
    }
    ///
    /// Добавляем текст
    pub fn text(mut self, v: impl Display) -> Self {
        write_tag(&mut self.content, "p", v);
        self
    }
    ///
    /// Добавляем список без номерации
    pub fn list<F>(mut self, build: F) -> Self 
    where 
        F: FnOnce(ListBuilder) -> ListBuilder 
    {
        let list: ListBuilder = build(ListBuilder::new());
        self.content.push_str(&list.build());
        self
    }
    ///
    /// Возвращает сформированный контент
    pub fn build(self) -> String {
        self.content
    }
}
///
/// Writes escaped html tag
fn write_tag<W: Write>(
    out: &mut W,
    tag: &str,
    v: impl Display,
) {
    write!(out, "<{}>", tag).unwrap();
    write_escaped(out, v);
    write!(out, "</{}>\n", tag).unwrap();
}
///
/// Escapes html input
pub(super) fn escape(input: impl Display) -> String {
    let mut o = String::new();
    write_escaped(&mut o, input);
    o
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
