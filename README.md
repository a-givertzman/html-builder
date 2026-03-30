# HTML document builder

Can be used like...
 
```rust
fn main() {
    let css = std::fs::read_to_string("style.css").unwrap();
    let html = Document::new()
        .add_title("Monthly Report")
        .add_style(css)
        .add_header(|header| header
            .add_h1("📊 Monthly Performance Report")
            .add_text("Generated automatically by internal system")
        )
        .add_section(|section| section
            .add_h2("Overview")
            .add_text("This report summarizes key metrics and system performance for the current period.")
            .add_text("All values are aggregated and validated against the latest available data.")
        )
        .add_section(|section| section
            .add_h2("Key Highlights")
            .add_list(|list| list
                .add_item("System uptime: 99.98%")
                .add_item("Average response time reduced by 12%")
                .add_item("No critical incidents reported")
            )
        )
        .add_section(|section| section
            .add_h2("Next Steps")
            .add_text("Focus will be placed on improving edge-case handling and further reducing latency.")
            .add_list(|list| list
                .add_item("Optimize database queries")
                .add_item("Extend monitoring coverage")
                .add_item("Improve alerting system")
            )
        )
        .add_footer(|footer| footer
            .add_text("© 2026 Internal System • All rights reserved")
        )
        .build();
    std::fs::File::create("index.html").unwrap();
    std::fs::write("index.html", html).unwrap();
    // println!("{}", html);
}
```