use std::{fmt::{Display, Write}, sync::Arc};

use crate::{Translation, documents::{Element, write_escaped}};

///
/// ### Simple Html list
pub struct TableBuilder {
    locale: Arc<Translation>,
    id: String,
    classes: String,
    header: Vec<Child>,
    rows: Vec<Vec<Child>>,
}
//
impl TableBuilder {
    pub fn new() -> Self {
        Self {
            locale: Arc::new(Translation::empty()),
            id: String::new(),
            classes: String::new(),
            header: vec![],
            rows: vec![],
        }
    }
    ///
    /// ### Добавляем переводы элементу
    pub fn localize(mut self, t: impl Into<Arc<Translation>>) -> Self {
        self.locale = t.into();
        self
    }
    ///
    /// ### Set Id to the Html element
    pub fn id(mut self, v: impl Display) -> Self {
        self.id.clear();
        write_escaped(&mut self.id, v);
        self
    }
    ///
    /// ### Add class to the Html element
    pub fn class(mut self, v: impl Display) -> Self {
        if !self.classes.is_empty() {
            write!(self.classes, " ").unwrap();
        }
        write_escaped(&mut self.classes, v);
        self
    }
    ///
    /// ### Добавляем шапку таблицы
    pub fn header<T: Display>(mut self, cols: impl IntoIterator<Item = T>) -> Self {
        let mut out = String::with_capacity(32);
        self.header = cols.into_iter().map(|v| {
            out.clear();
            write_escaped(&mut out, self.locale.tr(&v));
            Child::Text(out.clone())
        }).collect();
        self
    }
    ///
    /// ### Добавляем строку таблицы
    /// 
    /// > **Внимание!** Значения ячеек не переводятся автоматически!
    pub fn row<T: Display>(mut self, cels: impl IntoIterator<Item = T>) -> Self {
        let mut out = String::with_capacity(32);
        let row = cels.into_iter().map(|v| {
            write_escaped(&mut out, v);
            Child::Text(out.clone())
        }).collect();
        self.rows.push(row);
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
        if !self.header.is_empty() {
            write!(out, "<thead><tr>").unwrap();
            for cell in self.header {
                let cell = match cell {
                    Child::Text(t) => t,
                    Child::El(el) => el.build(),
                };
                write!(out, "<th>{}</th>", cell).unwrap();
            }
            write!(out, "</tr></thead>").unwrap();
        }
        write!(out, "<tbody>").unwrap();

        for row in self.rows {
            write!(out, "<tr>").unwrap();
            for cell in row {
                let cell = match cell {
                    Child::Text(t) => t,
                    Child::El(el) => el.build(),
                };
                write!(out, "<td>{}</td>", cell).unwrap();
            }
            write!(out, "</tr>").unwrap();
        }
        write!(out, "</tbody></table>").unwrap();
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
