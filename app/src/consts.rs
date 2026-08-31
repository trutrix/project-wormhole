use egui::*;





pub const COLOR_BACKGROUND_DARK: Color32 = Color32::from_rgb(0, 0, 20);
pub const COLOR_TEXT_LIGHT: Color32 = Color32::from_rgb(200, 200, 200);



pub const MARGIN_DEFAULT_SIZE: i8 = 8;
pub const MARGIN_DEFAULT_SAME: Margin = Margin::same(MARGIN_DEFAULT_SIZE);

pub const PADDING_DEFAULT_SAME: Margin = MARGIN_DEFAULT_SAME;



pub const NAVBAR_FRAME: Frame = Frame { 
    inner_margin: PADDING_DEFAULT_SAME,
    fill: COLOR_BACKGROUND_DARK,
    stroke: Stroke::NONE,
    corner_radius: CornerRadius::ZERO,
    outer_margin: Margin::ZERO,
    shadow: Shadow::NONE,
};