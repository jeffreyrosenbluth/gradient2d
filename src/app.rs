use eframe::{
    egui,
    epaint::{Color32, ColorImage},
    epi,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use wassily::prelude::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state

fn wave(color_wave: &Gradient2dApp) -> ColorImage {
    let mut canvas = Canvas::new(720, 720);
    let mut coscol = CosColorXY::default();
    coscol.r.a = color_wave.red_a;
    coscol.r.b = color_wave.red_b;
    coscol.r.freq_x = color_wave.red_freq_x;
    coscol.r.phase_x = color_wave.red_phase_x * TAU;
    coscol.r.freq_y = color_wave.red_freq_y;
    coscol.r.phase_y = color_wave.red_phase_y * TAU;
    coscol.g.a = color_wave.green_a;
    coscol.g.b = color_wave.green_b;
    coscol.g.freq_x = color_wave.green_freq_x;
    coscol.g.phase_x = color_wave.green_phase_x * TAU;
    coscol.g.freq_y = color_wave.green_freq_y;
    coscol.g.phase_y = color_wave.green_phase_y * TAU;
    coscol.b.a = color_wave.blue_a;
    coscol.b.b = color_wave.blue_b;
    coscol.b.freq_x = color_wave.blue_freq_x;
    coscol.b.phase_x = color_wave.blue_phase_x * TAU;
    coscol.b.freq_y = color_wave.blue_freq_y;
    coscol.b.phase_y = color_wave.blue_phase_y * TAU;
    for i in 0..720 {
        for j in 0..720 {
            let c = coscol.cos_color_xy(i as f32 / 360.0 * PI, j as f32 / 360.0);
            canvas.dot(i as f32, j as f32, c);
        }
    }
    let mut buffer: Vec<Color32> = vec![];
    for p in canvas.pixels() {
        buffer.push(Color32::from_rgba_unmultiplied(
            p.red(),
            p.green(),
            p.blue(),
            p.alpha(),
        ));
    }
    ColorImage::from_rgba_unmultiplied([canvas.w_usize(), canvas.h_usize()], canvas.data())
}
pub struct Gradient2dApp {
    red_a: f32,
    red_b: f32,
    red_freq_x: f32,
    red_phase_x: f32,
    red_freq_y: f32,
    red_phase_y: f32,
    green_a: f32,
    green_b: f32,
    green_freq_x: f32,
    green_phase_x: f32,
    green_freq_y: f32,
    green_phase_y: f32,
    blue_a: f32,
    blue_b: f32,
    blue_freq_x: f32,
    blue_phase_x: f32,
    blue_freq_y: f32,
    blue_phase_y: f32,
}

impl Default for Gradient2dApp {
    fn default() -> Self {
        Self {
            red_a: 0.5,
            red_b: 0.5,
            red_freq_x: 1.0,
            red_phase_x: 0.0,
            red_freq_y: 1.0,
            red_phase_y: 0.0,
            green_a: 0.5,
            green_b: 0.5,
            green_freq_x: 1.0,
            green_phase_x: 0.1,
            green_freq_y: 1.0,
            green_phase_y: 0.10,
            blue_a: 0.5,
            blue_b: 0.5,
            blue_freq_x: 1.0,
            blue_phase_x: 0.2,
            blue_freq_y: 1.0,
            blue_phase_y: 0.20,
        }
    }
}

impl Distribution<Gradient2dApp> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Gradient2dApp {
        let mut b: f32;
        let red_a = rng.gen_range(0.25..=0.75);
        b = 1.0 - red_a;
        let red_b = rng.gen_range(b / 2.0..=b);
        let red_freq_x = rng.gen_range(0.5..=2.0);
        let red_phase_x = rng.gen_range(0.0..=0.5);
        let red_freq_y = rng.gen_range(0.5..=2.0);
        let red_phase_y = rng.gen_range(0.0..=0.5);
        let green_a = rng.gen_range(0.25..=0.75);
        b = 1.0 - green_a;
        let green_b = rng.gen_range(b / 2.0..=b);
        let green_freq_x = rng.gen_range(0.5..=2.0);
        let green_phase_x = rng.gen_range(0.0..=0.5);
        let green_freq_y = rng.gen_range(0.5..=2.0);
        let green_phase_y = rng.gen_range(0.0..=0.5);
        let blue_a = rng.gen_range(0.25..0.75);
        b = 1.0 - blue_a;
        let blue_b = rng.gen_range(b / 2.0..=b);
        let blue_freq_x = rng.gen_range(0.5..=2.0);
        let blue_phase_x = rng.gen_range(0.0..=0.5);
        let blue_freq_y = rng.gen_range(0.5..=2.0);
        let blue_phase_y = rng.gen_range(0.0..=0.5);
        Gradient2dApp {
            red_a,
            red_b,
            red_freq_x,
            red_phase_x,
            red_freq_y,
            red_phase_y,
            green_a,
            green_b,
            green_freq_x,
            green_phase_x,
            green_freq_y,
            green_phase_y,
            blue_a,
            blue_b,
            blue_freq_x,
            blue_phase_x,
            blue_freq_y,
            blue_phase_y,
        }
    }
}

impl epi::App for Gradient2dApp {
    fn name(&self) -> &str {
        "Procedural Color Generator"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self {
            red_a,
            red_b,
            red_freq_x,
            red_phase_x,
            red_freq_y,
            red_phase_y,
            green_a,
            green_b,
            green_freq_x,
            green_phase_x,
            green_freq_y,
            green_phase_y,
            blue_a,
            blue_b,
            blue_freq_x,
            blue_phase_x,
            blue_freq_y,
            blue_phase_y,
        } = self;
        frame.set_window_size(eframe::epaint::Vec2 {
            x: 1040.0,
            y: 840.0,
        });

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .min_width(250.0)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.heading("Controls");
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Red").color(Color32::RED));
                ui.add(egui::Slider::new(red_a, 0.0..=1.0).text("a"));
                ui.add(egui::Slider::new(red_b, 0.0..=1.0).text("b"));
                ui.add(egui::Slider::new(red_freq_x, 0.0..=2.0).text("x - frequency"));
                ui.add(egui::Slider::new(red_phase_x, 0.0..=1.0).text("x - phase"));
                ui.add(egui::Slider::new(red_freq_y, 0.0..=2.0).text("y - frequency"));
                ui.add(egui::Slider::new(red_phase_y, 0.0..=1.0).text("y - phase"));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Green").color(Color32::GREEN));
                ui.add(egui::Slider::new(green_a, 0.0..=1.0).text("a"));
                ui.add(egui::Slider::new(green_b, 0.0..=1.0).text("b"));
                ui.add(egui::Slider::new(green_freq_x, 0.0..=2.0).text("x - frequency"));
                ui.add(egui::Slider::new(green_phase_x, 0.0..=1.0).text("x - phase"));
                ui.add(egui::Slider::new(green_freq_y, 0.0..=2.0).text("y - frequency"));
                ui.add(egui::Slider::new(green_phase_y, 0.0..=1.0).text("y - phase"));
                ui.add_space(20.0);
                ui.label(egui::RichText::new("Blue").color(Color32::LIGHT_BLUE));
                ui.add(egui::Slider::new(blue_a, 0.0..=1.0).text("a"));
                ui.add(egui::Slider::new(blue_b, 0.0..=1.0).text("b"));
                ui.add(egui::Slider::new(blue_freq_x, 0.0..=2.0).text("x - frequency"));
                ui.add(egui::Slider::new(blue_phase_x, 0.0..=1.0).text("x - phase"));
                ui.add(egui::Slider::new(blue_freq_y, 0.0..=2.0).text("y - frequency"));
                ui.add(egui::Slider::new(blue_phase_y, 0.0..=1.0).text("y - phase"));
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if ui.button("Reset").clicked() {
                        *red_a = 0.5;
                        *red_b = 0.5;
                        *red_freq_x = 1.0;
                        *red_phase_x = 0.0;
                        *red_freq_y = 1.0;
                        *red_phase_y = 0.0;
                        *green_a = 0.5;
                        *green_b = 0.5;
                        *green_freq_x = 1.0;
                        *green_phase_x = 0.1;
                        *green_freq_y = 1.0;
                        *green_phase_y = 0.1;
                        *blue_a = 0.5;
                        *blue_b = 0.5;
                        *blue_freq_x = 1.0;
                        *blue_phase_x = 0.2;
                        *blue_freq_y = 1.0;
                        *blue_phase_y = 0.2;
                    }
                    ui.add_space(20.0);
                    if ui.button("Random").clicked() {
                        let mut rng = rand::thread_rng();
                        let vals: Gradient2dApp = rng.gen();
                        *red_a = vals.red_a;
                        *red_b = vals.red_b;
                        *red_freq_x = vals.red_freq_x;
                        *red_phase_x = vals.red_phase_x;
                        *red_freq_y = vals.red_freq_y;
                        *red_phase_y = vals.red_phase_y;
                        *green_a = vals.green_a;
                        *green_b = vals.green_b;
                        *green_freq_x = vals.green_freq_x;
                        *green_phase_x = vals.green_phase_x;
                        *green_freq_y = vals.green_freq_y;
                        *green_phase_y = vals.green_phase_y;
                        *blue_a = vals.blue_a;
                        *blue_b = vals.blue_b;
                        *blue_freq_x = vals.blue_freq_x;
                        *blue_phase_x = vals.blue_phase_x;
                        *blue_freq_y = vals.blue_freq_y;
                        *blue_phase_y = vals.blue_phase_y;
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.add_space(10.0);
            ui.heading("Color Palette");
            ui.add_space(40.0);
            egui::warn_if_debug_build(ui);

            let mut opt_texture: Option<egui::TextureHandle> = None;
            let texture: &egui::TextureHandle =
                opt_texture.get_or_insert_with(|| ui.ctx().load_texture("wave", wave(self)));
            let img_size = texture.size_vec2();
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.image(texture, img_size)
            });
        });
    }
}
