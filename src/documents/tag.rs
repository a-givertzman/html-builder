use std::fmt::{Debug, Display};

///
/// Html Tag name
pub struct Tag {
    name: &'static str,
    pub is_void: bool,
}
//
impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
///
/// Macro defines short methods to construct all tags
/// pub fn p()  -> Tag { Tag { name: "p" } }
/// pub fn h1() -> Tag { Tag { name: "h1" } }
/// pub fn h2() -> Tag { Tag { name: "h2" } }
///
macro_rules! define_tags {
    ($($name:ident => ($tag:expr, $void:expr, $doc:expr)),* $(,)?) => {
        $(
            #[doc = $doc]
            pub fn $name() -> Tag {
                Tag {
                    name: $tag,
                    is_void: $void,
                }
            }
        )*
    };
}
//
define_tags! {
    // текст
    //          tag name        isVoid tag      doc comment to the tag
    p =>        ("p",           false,          "Paragraph text block"),
    span =>     ("span",        false,          "Inline text container"),
    strong =>   ("strong",      false,          "Important text (bold)"),
    em =>       ("em",          false,          "Emphasized text (italic)"),
    small =>    ("small",       false,          "Secondary or fine print text"),

    // заголовки
    h1 => ("h1", false, "Heading level 1"),
    h2 => ("h2", false, "Heading level 2"),
    h3 => ("h3", false, "Heading level 3"),
    h4 => ("h4", false, "Heading level 4"),
    h5 => ("h5", false, "Heading level 5"),
    h6 => ("h6", false, "Heading level 6"),

    // контейнеры
    div => ("div", false, "Generic block container"),
    section => ("section", false, "Thematic section of content"),
    article => ("article", false, "Independent content block"),
    header => ("header", false, "Header of a section or page"),
    footer => ("footer", false, "Footer of a section or page"),
    main => ("main", false, "Main content area"),

    // списки
    ul => ("ul", false, "Unordered list"),
    ol => ("ol", false, "Ordered list"),
    li => ("li", false, "List item"),

    // ссылки / медиа
    a => ("a", false, "Hyperlink"),
    img => ("img", true, "Image (self-closing)"),
    svg => ("svg", false, "SVG (Scalable Vector Graphics)"),


    // таблицы
    table => ("table", false, "Table container"),
    thead => ("thead", false, "Table header group"),
    tbody => ("tbody", false, "Table body"),
    tr => ("tr", false, "Table row"),
    td => ("td", false, "Table cell"),
    th => ("th", false, "Table header cell"),

    // формы
    form => ("form", false, "Form container"),
    input => ("input", true, "Input field (self-closing)"),
    button => ("button", false, "Clickable button"),
    label => ("label", false, "Label for input"),
    textarea => ("textarea", false, "Multiline text input"),

    // прочее
    br => ("br", true, "Line break (self-closing)"),
    hr => ("hr", true, "Horizontal rule (self-closing)"),
}
//
// All void tags
// <area>: Defines a clickable area inside an image map.
// <base>: Specifies the base URL for all relative URLs in a document.
// <br>: Produces a single line break.
// <col>: Specifies column properties for each column within a <colgroup> element.
// <embed>: Provides an integration point for an external application or interactive content.
// <hr>: Represents a thematic break (horizontal rule).
// <img>: Embeds an image.
// <input>: Defines an input control for web-based forms.
// <link>: Specifies relationships between the current document and an external resource (e.g., CSS).
// <meta>: Provides metadata about the HTML document.
// <param>: Defines parameters for an <object> element.
// <source>: Specifies multiple media resources for <picture>, <audio>, or <video> elements.
// <track>: Specifies external text tracks for <audio> or <video> elements.
// <wbr>: Represents a "Word Break Opportunity" where the browser may optionally break a line