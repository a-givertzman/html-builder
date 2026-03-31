use debugging::session::debug_session::{DebugSession, LogLevel};
use sal_core::dbg::Dbg;

use crate::{colspan, documents::{Document, br, button, div, form, h2, h4, label, strong, table, tbody, td, textarea, th, thead, tr}, img, svg, tests::chart};
///
/// Testing all basic functioms of the html builder
#[test]
fn basic() {
    DebugSession::new().filter(LogLevel::Debug).init();
    let css = std::fs::read_to_string("style.css").unwrap();
    let dbg = Dbg::own("basic-test");
    log::debug!("{dbg} | css loaded from 'style.css'");
    let html = Document::new()
        .title("Monthly Report")
        .style(css)
        .header(|header| header
            .class("main-header")
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
            .class("section-1")
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
            .class("section-2")
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
            .class("section-3")
            .h2(|el| el.text("Detailed Metrics"))
            .el(table(), |el| el
                .class("metrics-table")
                .el(thead(), |el| el
                    .el(tr(), |el| el
                        // Общий заголовок «Система», объединяет 3 столбца
                        .el(th(), |el| el
                            .attr(colspan(), "3")
                            .class("th-center")
                            .text("System Performance")
                        )
                        // Общий заголовок «Пользователи», объединяет 2 столбца
                        .el(th(), |el| el
                            .attr(colspan(), "2")
                            .class("th-center")
                            .text("User Activity")
                        )
                    )
                    .el(tr(), |el| el
                        // Конкретные метрики во второй строке
                        .el(th(), |el| el.text("Uptime (%)"))
                        .el(th(), |el| el.text("Latency (ms)"))
                        .el(th(), |el| el.text("Errors"))
                        .el(th(), |el| el.text("Active Users"))
                        .el(th(), |el| el.text("New Registrations"))
                    )
                )
                .el(tbody(), |el| el
                    .el(tr(), |el| el
                        .el(td(), |el| el.text("99.98"))
                        .el(td(), |el| el.text("120"))
                        .el(td(), |el| el.text("2"))
                        .el(td(), |el| el.text("1,450"))
                        .el(td(), |el| el.text("48"))
                    )
                    .el(tr(), |el| el
                        .el(td(), |el| el.text("99.95"))
                        .el(td(), |el| el.text("135"))
                        .el(td(), |el| el.text("5"))
                        .el(td(), |el| el.text("1,380"))
                        .el(td(), |el| el.text("32"))
                    )
                )
            )
        )
        // --- SECTION 4 ---
        .section(|section| section
            .class("section-4")
            .el(div(), |div| div
                .raw(chart::draw())
            )
        )
        // --- SECTION 5 ---
        .section(|section| section
            .class("section-5")
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
        // --- SECTION 6 (формы + void) ---
        .section(|section| section
            .class("section-6")
            .h2(|el| el.text("Quick Feedback"))
            .el(form(), |el| el
                .id("feedback-form")
                .class("feedback-form")
                .el(label(), |el| el
                    .text("Your feedback:")
                )
                .el(br(), |el| el)
                .el(textarea(), |el| el
                    .id("feedback")
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
            .class("main-footer")
            .text(|el| el
                .class("footer-text")
                .text("© 2026 Internal System • All rights reserved")
            )
        )
        .build();
    std::fs::File::create("index.html").unwrap();
    std::fs::write("index.html", html).unwrap();
    log::debug!("{dbg} | Result html stored into 'index.html'");
    log::debug!("{dbg} | All done");
}