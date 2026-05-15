use std::fmt::Display;

///
/// Html Element [Attribute]
pub struct Attribute {
    pub name: &'static str,
    pub is_flag: bool, // boolean attribute
}
//
impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
///
/// Macro defines short methods to construct all [Attribute]s
/// pub fn href()  -> Attribute { Attribute { name: "href" } }
/// pub fn colspan() -> Attribute { Attribute { name: "colspan" } }
/// pub fn disabled() -> Attribute { Attribute { name: "disabled" } }
///
macro_rules! define_attrs {
    ($($name:ident => ($attr:expr, $flag:expr, $doc:expr)),* $(,)?) => {
        $(
            #[doc = $doc]
            pub fn $name() -> Attribute {
                Attribute {
                    name: $attr,
                    is_flag: $flag,
                }
            }
        )*
    };
}
define_attrs! {
    // глобальные
    id_attr => ("id", false, "Element id"),
    class_attr => ("class", false, "CSS class"),
    style => ("style", false, "Inline styles"),
    title => ("title", false, "Tooltip text"),

    // ссылки
    href => ("href", false, "Link target"),
    target => ("target", false, "Link target behavior"),

    // формы
    type_attr => ("type", false, "Input type"),
    name => ("name", false, "Field name"),
    value => ("value", false, "Field value"),
    placeholder => ("placeholder", false, "Placeholder text"),

    // boolean
    disabled => ("disabled", true, "Disabled state"),
    checked => ("checked", true, "Checked state"),
    readonly => ("readonly", true, "Readonly state"),

    // таблицы
    colspan => ("colspan", false, "Column span"),
    rowspan => ("rowspan", false, "Row span"),

    // data-*
    data_id => ("data-id", false, "Custom data id"),
}