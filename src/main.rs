use cairo;
use pango;

const WIDTH: f64 = 256.0;
const HEIGHT: f64 = 256.0;

fn main() {
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 100, 30).unwrap();
    let cr = cairo::Context::new(&surface).unwrap();
    cr.rectangle(0_f64, 0_f64, 100_f64, 30_f64);
    cr.set_source_rgb(0_f64, 0_f64, 0.5);
    cr.fill().unwrap();

    let pango_scale = pango::units_from_double(1_f64);

    let layout = pangocairo::create_layout(&cr);
    let mut desc = pango::FontDescription::new();
    desc.set_family("Sans");
    desc.set_weight(pango::Weight::Bold);
    desc.set_size(48 * pango_scale);
    layout.set_font_description(Some(&desc));

    layout.set_markup("A <b>bold</b>\nidea");
    layout.set_alignment(pango::Alignment::Center);

    let (_ink_box, log_box) = layout.extents();
    let (text_width, text_height): (f64, f64) = (
        1.0 * (log_box.width() as f64) / (pango_scale as f64),
        1.0 * (log_box.height() as f64) / (pango_scale as f64),
    );

    cr.move_to(
        WIDTH / 2.0 - text_width / 2.0,
        HEIGHT / 2.0 - text_height / 2.0,
    );
    cr.set_source_rgb(1.0, 1.0, 1.0);
    pangocairo::show_layout(&cr, &layout);
    cr.fill().unwrap();

    let mut writer = std::fs::File::create("test.png").unwrap();
    surface.write_to_png(&mut writer).unwrap();
}
