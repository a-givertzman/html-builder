use plotters::prelude::*;
///
/// 
pub fn draw() -> String {
    chart().unwrap_or_else(|err| error_svg(&err.to_string()))
}
///
/// 
fn chart() -> Result<String, Box<dyn std::error::Error>> {
    let mut svg = String::new();
    {
        let root = SVGBackend::with_string(&mut svg, (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;
    
        chart.configure_mesh().draw()?;
        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))?
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;
        root.present()?;
    }
    Ok(svg)
}
///
/// 
fn error_svg(err: &str) -> String {
    let mut svg = String::new();
    {
        // Создаем область рисования в памяти (размер 400x100)
        let root = SVGBackend::with_string(&mut svg, (400, 100)).into_drawing_area();
        // Заливаем фон (например, светло-красным)
        root.fill(&RGBColor(255, 230, 230)).unwrap();
        // Рисуем рамку
        root.draw(&Rectangle::new([(0, 0), (399, 99)], RED.stroke_width(2))).unwrap();
        // Добавляем текст ошибки
        root.draw(&Text::new(
            format!("Error: {}", err),
            (20, 40),
            ("sans-serif", 20).into_font().color(&RED),
        )).unwrap();
        // Сбрасываем буфер
        root.present().unwrap();
    }
    svg
}