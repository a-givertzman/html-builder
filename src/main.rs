mod documents;
use crate::documents::{Document, br, button, div, form, h2, h3, h4, h6, label, p, strong, table, tbody, td, textarea, th, thead, tr};
///
/// 
fn main() {
    let css = std::fs::read_to_string("style.css").unwrap();
    let html = Document::new()
        .title("Monthly Report")
        .style(css)
        .header(|header| header
            .h1(|el| el
                .class("main-title")
                .text("📊 Monthly Performance Report")
            )
            .text(|el| el
                .class("subtitle")
                .text("Generated automatically by internal system")
            )
        )
        // --- SECTION 1 ---
        .section(|section| section
            .el(div(), |el| el
                .id("section-1")
                .class("card")
                .el(h2(), |el| el
                    .class("section-header")
                    .text("Overview")
                )
                .text("This report summarizes key metrics and system performance.")
                .el(br(), |el| el)
                .text("All values are aggregated and validated.")
                // вложенный контейнер
                .el(div(), |el| el
                    .class("note-box")
                    .el(strong(), |el| el.text("Note: "))
                    .text("Data may be delayed up to 5 minutes.")
                )
            )
        )
        // --- SECTION 2 ---
        .section(|section| section
            .h2(|el| el.text("Key Highlights"))
            .list(|list| list
                .class("general-list")
                // обычные элементы
                .item("System uptime: 99.98%")
                .item("Latency reduced by 12%")
                // кастомный элемент внутри списка
                .el(div(), |el| el
                    .class("highlight")
                    .el(strong(), |el| el.text("No critical incidents"))
                )
            )
        )
        // --- SECTION 3 ---
        .section(|section| section
            .h2(|el| el.text("Detailed Metrics"))
            .el(table(), |el| el
                .class("metrics-table")
                .el(thead(), |el| el
                    .el(tr(), |el| el
                        .el(th(), |el| el.text("Metric"))
                        .el(th(), |el| el.text("Value"))
                    )
                )
                .el(tbody(), |el| el
                    .el(tr(), |el| el
                        .el(td(), |el| el.text("Uptime"))
                        .el(td(), |el| el.text("99.98%"))
                    )
                    .el(tr(), |el| el
                        .el(td(), |el| el.text("Latency"))
                        .el(td(), |el| el.text("120ms"))
                    )
                )
            )
        )
        // --- SECTION 4 ---
        .section(|section| section
            .h2(|el| el.text("Next Steps"))
            .text(|el| el.text("Focus areas for the next iteration:"))
            .list(|list| list
                .class("general-list")
                .el(div(), |el| el
                    .el(h4(), |el| el.class("list-item").text("Optimize database queries"))
                )
                .el(div(), |el| el
                    .el(h4(), |el| el.class("list-item").text("Extend monitoring coverage"))
                )
                .el(div(), |el| el
                    .el(h4(), |el| el.class("list-item").text("Improve alerting system"))
                )
            )
        )
        // --- SECTION 5 (формы + void) ---
        .section(|section| section
            .h2(|el| el.text("Quick Feedback"))
            .el(form(), |el| el
                .class("feedback-form")
                .el(label(), |el| el
                    .text("Your feedback:")
                )
                .el(br(), |el| el)
                .el(textarea(), |el| el
                    .class("input")
                )
                .el(br(), |el| el)
                .el(button(), |el| el
                    .class("btn")
                    .text("Submit")
                )
            )
        )
        .footer(|footer| footer
            .text(|el| el
                .class("footer-text")
                .text("© 2026 Internal System • All rights reserved")
            )
        )
        .build();
    std::fs::write("index.html", html).unwrap();
}