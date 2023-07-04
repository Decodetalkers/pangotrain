use cairo::{self, Context};

fn draw_rectangle(content: &Context) {
    content.set_source_rgb(0.0, 0.0, 0.0);
    content.move_to(0.0, 0.0);
    content.line_to(0.0, 150.0);
    content.move_to(50.0, 0.0);
    content.line_to(50.0, 150.0);
    content.move_to(100.0, 0.0);
    content.line_to(100.0, 150.0);
    content.move_to(150.0, 0.0);
    content.line_to(150.0, 150.0);

    content.move_to(0.0, 0.0);
    content.line_to(150.0, 0.0);
    content.move_to(0.0, 50.0);
    content.line_to(150.0, 50.0);
    content.move_to(0.0, 100.0);
    content.line_to(150.0, 100.0);
    content.move_to(0.0, 150.0);
    content.line_to(150.0, 150.0);

    content.stroke().unwrap();

    let pangolayout = pangocairo::create_layout(content);
    let mut desc = pango::FontDescription::new();
    desc.set_family("Sans");
    desc.set_weight(pango::Weight::Bold);
    desc.set_size(27 * pango::SCALE);
    pangolayout.set_font_description(Some(&desc));

    pangolayout.set_text("0");
    content.save().unwrap();
    content.move_to(0.0, 0.0);
    pangocairo::show_layout(content, &pangolayout);
    content.restore().unwrap();

    pangolayout.set_text("1");
    content.save().unwrap();
    content.move_to(50.0, 0.0);
    pangocairo::show_layout(content, &pangolayout);
    content.restore().unwrap();

    pangolayout.set_text("2");
    content.save().unwrap();
    content.move_to(100.0, 0.0);
    pangocairo::show_layout(content, &pangolayout);
    content.restore().unwrap();

}

fn main() {
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 150, 150).unwrap();
    let cr = cairo::Context::new(&surface).unwrap();
    cr.set_source_rgb(1_f64, 1_f64, 1_f64);
    cr.paint().unwrap();

    draw_rectangle(&cr);

    let mut writer = std::fs::File::create("test.png").unwrap();
    surface.write_to_png(&mut writer).unwrap();
}
