use eframe::Error;
use eframe::egui::{Context, FontData, FontDefinitions, FontFamily};

pub fn setup(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    let comic_sans_regular = include_bytes!("../../assets/comic-sans/Ldfcomicsans-jj7l.ttf");
    fonts.font_data.insert(
        "comic_sans_regular".to_owned(),
        FontData::from_static(comic_sans_regular).into(),
    );

    // Load the bold Comic Sans font
    let comic_sans_bold = include_bytes!("../../assets/comic-sans/Ldfcomicsansbold-zgma.ttf");
    fonts.font_data.insert(
        "comic_sans_bold".to_owned(),
        FontData::from_static(comic_sans_bold).into(),
    );

    // Load the light Comic Sans font
    let comic_sans_light = include_bytes!("../../assets/comic-sans/Ldfcomicsanslight-6dZo.ttf");
    fonts.font_data.insert(
        "comic_sans_light".to_owned(),
        FontData::from_static(comic_sans_light).into(),
    );

    // Load the hairline Comic Sans font
    let comic_sans_hairline =
        include_bytes!("../../assets/comic-sans/Ldfcomicsanshairline-5PmL.ttf");
    fonts.font_data.insert(
        "comic_sans_hairline".to_owned(),
        FontData::from_static(comic_sans_hairline).into(),
    );

    // Create Comic Sans font family
    fonts.families.insert(
        FontFamily::Name("ComicSans".into()),
        vec![
            "comic_sans_regular".to_owned(),
            "comic_sans_bold".to_owned(),
            "comic_sans_light".to_owned(),
            "comic_sans_hairline".to_owned(),
        ],
    );

    // Load Inter variable font
    let inter_variable =
        include_bytes!("../../assets/inter/Inter-VariableFont_opsz,wght.ttf");
    fonts.font_data.insert(
        "inter_variable".to_owned(),
        FontData::from_static(inter_variable).into(),
    );

    // Create Inter font family
    fonts.families.insert(
        FontFamily::Name("Inter".into()),
        vec!["inter_variable".to_owned()],
    );

    // Set Inter as the default proportional font
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "inter_variable".to_owned());

    ctx.set_fonts(fonts);
}
