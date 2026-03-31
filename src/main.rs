mod documents;

use crate::documents::{Document, br, div, h2, h3, h4, h6, p};

///
/// 
fn main() {
    let css = std::fs::read_to_string("style.css").unwrap();
    let html = Document::new()
        .title("Monthly Report")
        .style(css)
        .header(|header| header
            .h1(|el| el.text("📊 Monthly Performance Report"))
            .text(|el| el.text("Generated automatically by internal system"))
        )
        .section(|section| section
            .el(h2(), |el| el
                .id("section-1-header")
                .class("section-header")
                .text("Overview")
                .text("Overview")
            )
            .h2(|el| el.text("Overview"))
            .text(|el| el.text("This report summarizes key metrics and system performance for the current period."))
            .el(br(), |el| el)
            .text(|el| el.text("All values are aggregated and validated against the latest available data."))
        )
        .section(|section| section
            .el(div(), |el| el
                .el(h2(), |el| el
                    .class("section-2-header")
                    .text("Key Highlights")
                )
            )
            .h2(|el| el.text("Key Highlights"))
            .list(|list| list
                .item("System uptime: 99.98%")
                .item("Average response time reduced by 12%")
                .item("No critical incidents reported")
            )
        )
        .section(|section| section
            .h2(|el| el.text("Next Steps"))
            .text(|el| el.text("Focus will be placed on improving edge-case handling and further reducing latency."))
            .list(|list| list
                .el(h4(), |el| el.text("Optimize database queries"))
                .el(h4(), |el| el.text("Extend monitoring coverage"))
                .el(h4(), |el| el.text("Improve alerting system"))
            )
        )
        .footer(|footer| footer
            .text(|el| el.text("© 2026 Internal System • All rights reserved"))
        )
        .build();
    std::fs::File::create("index.html").unwrap();
    std::fs::write("index.html", html).unwrap();
    // println!("{}", html);
}