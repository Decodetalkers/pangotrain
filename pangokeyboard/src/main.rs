use cairo::{self, Context};

fn draw_rectangle(content: &Context) {
    content.set_source_rgb(0.0, 0.0, 0.0);
    content.move_to(0.0, 0.0);
    content.line_to(100.0, 100.0);
    content.move_to(100.0, 0.0);
    content.line_to(0.0, 100.0);
    content.set_line_width(2.0);
    content.stroke().unwrap();

    content.rectangle(0.0, 0.0, 50.0, 50.0);
    content.set_source_rgba(1.0, 0.0, 0.0, 0.8);
    content.fill().unwrap();

    content.rectangle(0.0, 50.0, 50.0, 50.0);
    content.set_source_rgba(0.0, 1.0, 0.0, 0.6);
    content.fill().unwrap();

    content.rectangle(50.0, 50.0, 50.0, 50.0);
    content.set_source_rgba(0.0, 1.0, 1.0, 0.4);
    content.fill().unwrap();
}

fn main() {
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 100, 100).unwrap();
    let cr = cairo::Context::new(&surface).unwrap();
    cr.set_source_rgb(1_f64, 1_f64, 1_f64);
    cr.paint().unwrap();

    draw_rectangle(&cr);

    let mut writer = std::fs::File::create("test.png").unwrap();
    surface.write_to_png(&mut writer).unwrap();
}
