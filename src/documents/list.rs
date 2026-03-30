pub struct ListBuilder {
    content: String,
}

impl ListBuilder {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
    ///
    /// Добавляем элемент списка
    pub fn add_item(mut self, text: &str) -> Self {
        self.content.push_str(&format!("  <li>{}</li>\n", text));
        self
    }
    ///
    /// 
    pub fn build(self) -> String {
        self.content
    }
}
