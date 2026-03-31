use std::fmt::{Display, Write};

use crate::documents::{Element, Footer, Header, Section, Tag, write_escaped};

///
/// Writes formatted data into a buffer.
macro_rules! w {
    ($o:expr, $($t:tt)*) => {
        write!($o, $($t)*).unwrap()
    };
}
///
/// Rendere HTML document
#[derive(Debug, Default)]
pub struct Document {
    title: String,
    styles: String,
    header: String,
    content: String,
    footer: String,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }
    ///
    /// Добавляем титул документу
    pub fn title(mut self, v: impl Display) -> Self {
        let mut title = String::with_capacity(32);
        write_escaped(&mut title, v);
        self.title = title;
        self
    }
    ///
    /// Добавляем css стили
    pub fn style(mut self, css: impl Display) -> Self {
        w!(self.styles, "{}\n", css);
        self
    }
    ///
    /// Добавляем Html елемент
    pub fn el<F>(mut self, t: Tag, build: F) -> Self 
    where 
        F: FnOnce(Element) -> Element 
    {
        let el: Element = build(Element::new(t));
        write_escaped(&mut self.content, el.build());
        self
    }
    ///
    /// Добавляем Header
    pub fn header<F>(mut self, build: F) -> Self 
    where 
        F: FnOnce(Header) -> Header 
    {
        let header: Header = build(Header::new());
        w!(self.header, "<header>\n{}\n</header>\n", header.build());
        self
    }
    ///
    /// Добавляем секцию
    pub fn section<F>(mut self, build: F) -> Self 
    where 
        F: FnOnce(Section) -> Section 
    {
        let section: Section = build(Section::new());
        w!(self.content, "<section>\n{}\n</section>\n", section.build());
        self
    }
    ///
    /// Добавляем footer
    pub fn footer<F>(mut self, build: F) -> Self 
    where 
        F: FnOnce(Footer) -> Footer 
    {
        let footer: Footer = build(Footer::new());
        w!(self.footer, "<footer>\n{}\n</footer>\n", footer.build());
        self
    }
    ///
    /// Возвращает скомпилированный документ
    pub fn build(self) -> String {
        let mut o = String::with_capacity(2048);
        w!(&mut o,"<!DOCTYPE html>\n");
        w!(&mut o,"<html lang=\"en\">\n");
        w!(&mut o,"<head>\n");
        w!(&mut o,"\t<meta charset=\"UTF-8\">\n");
        w!(&mut o,"\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        w!(&mut o,"\t<link rel=\"icon\" href=\"favicon.ico\" type=\"image/x-icon\">\n");
        w!(&mut o,"\t<title>{}</title>\n", self.title);
        w!(&mut o,"\t<style>\n{}\n\t</style>\n", self.styles);
        w!(&mut o,"</head>\n");
        w!(&mut o,"<body>\n");
        w!(&mut o,"{}\n", self.header);
        w!(&mut o,"\t<main>\n{}\n\t</main>\n", self.content);
        w!(&mut o,"{}\n", self.footer);
        w!(&mut o,"</body>\n");
        w!(&mut o,"</html>\n");
        o
    }
}
