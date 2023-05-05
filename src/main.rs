use macroquad::prelude::*;
use egui::{*, epaint::Shadow};

fn window_configuration() -> Conf
{
  return Conf
  {
    window_title: "egui ❤ macroquad".to_string(),
    window_width: 1290,
    window_height: 720,
    fullscreen: false,
    window_resizable: false,
    ..Default::default()
  };
}

#[macroquad::main(window_configuration)]
async fn main()
{
  loop
  {
    clear_background(WHITE);

    // Process keys, mouse etc.

    egui_macroquad::ui(|egui_ctx|
    {
      // Disabling all shadows on all windows
      egui_ctx.set_visuals(egui::Visuals{ window_shadow: Shadow::NONE, window_rounding: Rounding { nw: 10., ne: 0., sw: 10., se: 0. }, ..Default::default() });

      egui::Window::new("egui ❤ macroquad")
        .anchor(Align2::RIGHT_TOP, egui::Vec2::new(0., 0.))
        .constrain(true)
        .collapsible(false)
        .movable(false)
        .resizable(false)
        // ? Why is the height not working?
        .default_height(700.)
        .min_height(700.)
        .fixed_size(egui::vec2(200., 700.))
        // .frame(Frame { inner_margin: (), outer_margin: (), rounding: (), shadow: (), fill: (), stroke: () }) // Useful
        .show(egui_ctx, |ui|
        {
          // ui.style_mut().visuals.window_shadow = Shadow::NONE;
          // ui.style_mut().visuals.window_rounding = Rounding { nw: 10., ne: 0., sw: 10., se: 0. };
          ui.label("Hello World");
          ui.separator();
          ui.label("Hello again");
        });
    });

    // Draw things before egui

    egui_macroquad::draw();

    // Draw things after egui

    next_frame().await;
  }
}
