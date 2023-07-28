use egui::Slider;
use macroquad::prelude::*;

fn window_configuration() -> Conf
{ return Conf
  { window_title: "egui ❤ macroquad".to_string(),
    window_width: 1290,
    window_height: 720,
    fullscreen: false,
    window_resizable: false,
    ..Default::default()
  };
}

#[macroquad::main(window_configuration)]
async fn main()
{ let mut mode = Mode::Move;
  let mut slider_1 = 5.;
  let mut slider_2 = 5.;
  let mut slider_3 = 5.;
  let mut colour = [255., 0., 255.];

  loop
  { clear_background(BLACK);

    // Process keys, mouse etc.

    egui_macroquad::ui(|egui_ctx|
    // Disabling all shadows on all windows
    { egui_ctx.set_visuals(egui_macroquad::egui::Visuals
      { window_shadow: egui_macroquad::egui::epaint::Shadow::NONE,
        window_rounding: egui_macroquad::egui::Rounding
        { nw: 10.,
          ne: 0.,
          sw: 10.,
          se: 0.,
        },
        ..Default::default()
      });

      // egui ❤ macroquad
      egui::Window::new("Rust Graph Visualiser ❤")
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(0., 10.))
        .constrain(true)
        .collapsible(false)
        .movable(false)
        .resizable(false)
        // ? Why is the height not working?
        //.default_height(f32::INFINITY)
        //.min_height(700.)
        .fixed_size(egui::vec2(200., 700.))
        // .frame(Frame { inner_margin: (), outer_margin: (), rounding: (), shadow: (), fill: (), stroke: () }) // Useful
        .show(egui_ctx, |ui|
        { // ui.style_mut().visuals.window_shadow = Shadow::NONE;
          // ui.style_mut().visuals.window_rounding = Rounding { nw: 10., ne: 0., sw: 10., se: 0. };

          ui.label("Select a mode:");
          ui.horizontal(|ui|
          { ui.selectable_value(&mut mode, Mode::Move, "Move");
            ui.selectable_value(&mut mode, Mode::Line, "Line");
            ui.selectable_value(&mut mode, Mode::Point, "Point");
            ui.selectable_value(&mut mode, Mode::Path, "Path");
          });

          /*
          ui.separator();

          ui.label("Mode");
          ui.horizontal(|ui|
          { ui.radio_value(&mut mode, Mode::Move, "Move");
            ui.radio_value(&mut mode, Mode::Line, "Line");
            ui.radio_value(&mut mode, Mode::Point, "Point");
            ui.radio_value(&mut mode, Mode::Path, "Path");
          });
          */

          // The newlines are a hack to make all text fill up the same amount of vertical space
          match mode
          { Mode::Move => ui.label("Left click on a point to select it and hold left click to move it around.\n\n"),
            Mode::Line => ui.label("Left click on a point to select it and left click on another point to create a line or right click to delete an existing line."),
            Mode::Point => ui.label("Left click somewhere to create a point or right click on a point to delete it.\n"),
            Mode::Path => ui.label("Left click on a point to set the start and right click on a point to set the end.\n")
          };

          ui.separator();

          ui.label("Add in a pre-made graph:");
          ui.horizontal(|ui|
          { let _ = ui.button("Small");
            let _ = ui.button("Medium");
            let _ = ui.button("Large");
          });

          if mode == Mode::Path
          { ui.separator();
            let _ = ui.button("Find shortest path");
            ui.horizontal(|ui|
            { ui.label("Pick the color of the path:");
              ui.color_edit_button_rgb(&mut colour);
            });
          }

          ui.separator();

          // 51. difference
          ui.add_space(if mode == Mode::Path { 334. } else { 385. });

          ui.separator();

          ui.label("Adjust angle:");
          ui.add(Slider::new(&mut slider_1, 0.0..=10.0));
          ui.label("Adjust wing size:");
          ui.add(Slider::new(&mut slider_2, 0.0..=10.0));
          ui.label("Adjust base point:");
          ui.add(Slider::new(&mut slider_3, 0.0..=10.0));

          // ui.separator();

          // ui.label("Hello again");
        });
    });

    // Draw things before egui

    egui_macroquad::draw();

    // Draw things after egui

    next_frame().await;
  }
}

#[derive(PartialEq, Eq)]
enum Mode
{ Move, Point, Line, Path
}
