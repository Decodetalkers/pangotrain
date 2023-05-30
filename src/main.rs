use pango::prelude::*;
use pangocairo::cairo;
use pangocairo::glib;
use pangocairo::glib::GStr;
use pangocairo::pango;

const TEXT: &GStr = glib::gstr!("AAAA BBBB CCCC DDDD EEEE FFFF gggg hhhh iiii jjjj kkkk llll");

fn main() {
    let item = {
        let map = pangocairo::FontMap::default();
        let pc = map.create_context();
        let layout = pango::Layout::new(&pc);
        layout.set_width(256 * pango::SCALE);
        layout.set_height(256 * pango::SCALE);
        layout.set_text(TEXT);
        let mut layout_iter = layout.iter();
        layout_iter.run().unwrap()
    };
    let iter = pango::GlyphItemIter::new_start(&item, TEXT).unwrap();
    assert_eq!(iter.glyph_item(), &item);
    assert_eq!(iter.text(), TEXT);
    for (i, (sg, si, sc, eg, ei, ec)) in iter.into_iter().enumerate() {
        let i = i as i32;
        // ensure these are all single-byte ASCII characters
        assert_eq!(sg, i);
        assert_eq!(si, i);
        assert_eq!(sc, i);
        assert_eq!(eg, i + 1);
        assert_eq!(ei, i + 1);
        assert_eq!(ec, i + 1);
    }
    let mut surface =
        pangocairo::cairo::ImageSurface::create(cairo::Format::ARgb32, 100, 30).unwrap();
    let cr = cairo::Context::new(&surface).unwrap();
    cr.rectangle(0_f64, 0_f64, 100_f64, 30_f64);
    cr.set_source_rgb(0_f64, 0_f64, 0.5);
    cr.fill().unwrap();

    let pango_scale = pango::units_from_double(1_f64);

    let layout = pangocairo::create_layout(&cr);
    let mut desc = pango::FontDescription::new();
    desc.set_family("Serif");
    desc.set_size(48 * pango_scale);
    layout.set_font_description(Some(&desc));

    layout.set_markup("A <b>bold</b>\nidea");
    layout.set_alignment(pango::Alignment::Center);

    pangocairo::show_layout(&cr, &layout);
    cr.fill().unwrap();
    println!("{:?}", surface.data());
}
