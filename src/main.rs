mod documents;

use crate::documents::{Document, br, div, h2};

///
/// 
fn main() {
    let css = std::fs::read_to_string("style.css").unwrap();
    let html = Document::new()
        .title("Monthly Report")
        .style(css)
        .header(|header| header
            .h1("📊 Monthly Performance Report")
            .text("Generated automatically by internal system")
        )
        .section(|section| section
            .el(h2(), |el| el
                .id("section-1-header")
                .class("section-header")
                .text("Overview")
                .text("Overview")
            )
            .h2("Overview")
            .text("This report summarizes key metrics and system performance for the current period.")
            .el(br(), |el| el)
            .text("All values are aggregated and validated against the latest available data.")
        )
        .section(|section| section
            .el(div(), |el| el
                .el(h2(), |el| el
                    .class("section-2-header")
                    .text("Key Highlights")
                )
            )
            .h2("Key Highlights")
            .list(|list| list
                .item("System uptime: 99.98%")
                .item("Average response time reduced by 12%")
                .item("No critical incidents reported")
            )
        )
        .section(|section| section
            .h2("Next Steps")
            .text("Focus will be placed on improving edge-case handling and further reducing latency.")
            .list(|list| list
                .item("Optimize database queries")
                .item("Extend monitoring coverage")
                .item("Improve alerting system")
            )
        )
        .footer(|footer| footer
            .text("© 2026 Internal System • All rights reserved")
        )
        .build();
    std::fs::File::create("index.html").unwrap();
    std::fs::write("index.html", html).unwrap();
    // println!("{}", html);
}