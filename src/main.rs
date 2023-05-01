use macroquad::prelude::*;

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

    egui_macroquad::ui(|egui_ctx| {
      egui::Window::new("egui ❤ macroquad")
        .show(egui_ctx, |ui| {
            ui.label("Test");
        });
    });

    // Draw things before egui

    egui_macroquad::draw();

    // Draw things after egui

    next_frame().await;
  }
}
