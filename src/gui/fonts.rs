use std::sync::Arc;

use eframe::egui::{FontData, FontDefinitions, FontFamily};

pub fn get_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    load_comic_sans(&mut fonts);
    load_icon_fonts(&mut fonts);

    fonts
}

fn load_comic_sans(fonts: &mut FontDefinitions) {
    let comic_sans_fonts: &[(&str, &[u8])] = &[
        (
            "comic_sans_regular",
            include_bytes!("../../assets/comic-sans/comic_sans_regular.ttf") as &[u8],
        ),
        (
            "comic_sans_bold",
            include_bytes!("../../assets/comic-sans/comic_sans_bold.ttf") as &[u8],
        ),
        (
            "comic_sans_light",
            include_bytes!("../../assets/comic-sans/comic_sans_light.ttf") as &[u8],
        ),
        (
            "comic_sans_hairline",
            include_bytes!("../../assets/comic-sans/comic_sans_harline.ttf") as &[u8],
        ),
    ];

    let font_names: Vec<String> = comic_sans_fonts
        .iter()
        .map(|(name, data)| {
            fonts
                .font_data
                .insert(name.to_string(), FontData::from_static(data).into());
            name.to_string()
        })
        .collect();

    fonts
        .families
        .insert(FontFamily::Name("ComicSans".into()), font_names);
}

fn load_icon_fonts(fonts: &mut FontDefinitions) {
    let phosphor_variants = [
        ("phosphor", egui_phosphor::Variant::Regular),
        ("phosphor-bold", egui_phosphor::Variant::Bold),
        ("phosphor-fill", egui_phosphor::Variant::Fill),
    ];

    for (name, variant) in phosphor_variants {
        fonts
            .font_data
            .insert(name.into(), Arc::new(variant.font_data()));
        fonts.families.insert(
            FontFamily::Name(name.into()),
            vec!["Ubuntu-Light".into(), name.into()],
        );
    }
}
