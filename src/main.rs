mod documents;

use crate::documents::Document;

///
/// 
fn main() {
    let css = "body { font-family: sans-serif; }";
    
    let html = Document::new()
        .add_title("Document Title")
        .add_style(css)
        .add_header(|header| header
            .add_h1("Main Header")
        )
        .add_section(|section| section
            .add_h2("Header to Section 2")
            .add_text("some text content")
            .add_list(|list| list
                .add_item("Item 1")
                .add_item("Item 2")
            )
        )
        .add_footer(|footer| footer
            .add_text("The end of document")
        )
        .build();

    println!("{}", html);
}