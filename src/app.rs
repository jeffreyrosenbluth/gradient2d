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
use wassily::prelude::palette::FromColor;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state


// fn sample_multi(x: f32, rs: Vec<(f32, f32)>) -> f32 {
//     let n = rs.len();
//     let y = (x - 0.0001) * n as f32;
//     let range = rs[y as usize]; 
//     let z = y % n as f32;
//     map_range(z, 0.0, 1.0, range.0, range.1)
// }

fn draw(rorschach: RorschachApp, canvas: &mut Canvas) {
    let nf_x = Perlin::new(0);
    let nf_y = Perlin::new(1);
    let nf_z = Perlin::new(2);
    let opts_x = NoiseOpts::with_wh(canvas.width(), canvas.height())
        .scales(rorschach.x_scale)
        .factor(rorschach.x_factor);
    let opts_y = NoiseOpts::with_wh(canvas.width(), canvas.height())
        .scales(rorschach.y_scale)
        .factor(rorschach.y_factor);
    let opts_z = NoiseOpts::with_wh(canvas.width(), canvas.height())
        .scales(rorschach.z_scale)
        .factor(rorschach.z_factor);
    for i in 0..canvas.width() {
        for j in 0..canvas.height() {
            let k = if rorschach.dim2 { j as f32 } else { rorschach.y_value };
            let x = noise2d_01(&nf_x, &opts_x, i as f32, k).clamp(0.0, 1.0);
            let y = noise2d_01(&nf_y, &opts_y, i as f32, k).clamp(0.0, 1.0);
            let z = noise2d_01(&nf_z, &opts_z, i as f32, k).clamp(0.0, 1.0);
            let k = palette::Xyz::new(x, y, z);
            let k = palette::Srgba::from_color(k);
            let rgba = k.into_components();
            let c = Color::from_rgba(rgba.0, rgba.1, rgba.2, 1.0).unwrap().rotate_hue(rorschach.hue_angle);
            canvas.dot(i as f32, j as f32, c);
        }
    }
}

fn print(rorschach: RorschachApp) {
    let mut canvas = Canvas::new(1080, 1080);
    draw(rorschach, &mut canvas);
    canvas.save_png("./output/rorschach.png");
}

fn generate(rorschach: RorschachApp) -> ColorImage {
    let mut canvas = Canvas::new(360, 360);
    draw(rorschach, &mut canvas);
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

#[derive(Clone, Copy)]
pub struct RorschachApp {
    hue_angle: f32,
    x_scale: f32,
    x_factor: f32,
    y_scale: f32,
    y_factor: f32,
    z_scale: f32,
    z_factor: f32,
    dim2: bool,
    y_value: f32,
}

impl Default for RorschachApp {
    fn default() -> Self {
        Self {
            hue_angle: 0.0,
            x_scale: 4.0,
            x_factor: 1.0,
            y_scale: 4.0,
            y_factor: 1.0,
            z_scale: 4.0,
            z_factor: 1.0,
            dim2: true,
            y_value: 0.0,
        }
    }
}

impl Distribution<RorschachApp> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> RorschachApp {
        let x_scale = rng.gen_range(0.5..=10.0);
        let x_factor = rng.gen_range(0.1..=5.0);
        let y_scale = rng.gen_range(0.5..=10.0);
        let y_factor = rng.gen_range(0.1..=5.0);
        let z_scale = rng.gen_range(0.5..10.0);
        let z_factor = rng.gen_range(0.1..=5.0);
        let hue_angle: f32 = rng.gen_range(0.0..360.0);
        let dim2 = rng.gen_bool(0.5);
        let y_value: f32 = rng.gen_range(0.0..360.0);
        RorschachApp {
            hue_angle,
            x_scale,
            x_factor,
            y_scale,
            y_factor,
            z_scale,
            z_factor,
            dim2,
            y_value,
        }
    }
}

impl epi::App for RorschachApp {
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
        let rorschach = self.clone();
        let Self {
            hue_angle,
            x_scale,
            x_factor,
            y_scale,
            y_factor,
            z_scale,
            z_factor,
            dim2,
            y_value,
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
                ui.add(
                    egui::Slider::new(hue_angle, 0.0..=360.0)
                        .text("hue angle")
                        .fixed_decimals(2),
                );
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("X")
                        .size(18.0),
                );
                ui.separator();
                ui.add(
                    egui::Slider::new(x_scale, 0.5..=25.0)
                        .text("scale")
                        .fixed_decimals(2),
                );
                ui.add(
                    egui::Slider::new(x_factor, 0.1..=10.0)
                        .text("factor")
                        .fixed_decimals(2),
                );
                ui.separator();
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("Y")
                        .size(18.0),
                );
                ui.separator();
                ui.add(
                    egui::Slider::new(y_scale, 0.5..=25.0)
                        .text("scale")
                        .fixed_decimals(2),
                );
                ui.add(
                    egui::Slider::new(y_factor, 0.1..=10.0)
                        .text("factor")
                        .fixed_decimals(2),
                );
                ui.separator();
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("Z")
                        .size(18.0),
                );
                ui.separator();
                ui.add(
                    egui::Slider::new(z_scale, 0.5..=25.0)
                        .text("scale")
                        .fixed_decimals(2),
                );
                ui.add(
                    egui::Slider::new(z_factor, 0.1..=10.0)
                        .text("factor")
                        .fixed_decimals(2),
                );
                ui.separator();
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.add_space(60.0);
                    if ui.button("Reset").clicked() {
                        *x_scale = 4.0;
                        *x_factor = 1.0;
                        *y_scale = 4.0;
                        *y_factor = 1.0;
                        *z_scale = 4.0;
                        *z_factor = 1.0;
                    }
                    ui.add_space(20.0);
                    if ui.button("Random").clicked() {
                        let mut rng = rand::thread_rng();
                        let vals: RorschachApp = rng.gen();
                        *hue_angle = vals.hue_angle;
                        *x_scale = vals.x_scale;
                        *x_factor = vals.x_factor;
                        *y_scale = vals.y_scale;
                        *y_factor = vals.y_factor;
                        *z_scale = vals.z_scale;
                        *z_factor = vals.z_factor;
                    }
                });
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.radio_value(dim2, true, "2 Dimensions");
                    ui.radio_value(dim2, false, "1 Dimension");
                });
                if !*dim2 {
                    ui.add_space(10.0);
                    ui.add(egui::Slider::new(y_value, 0.0..=360.0).text("y value").fixed_decimals(0));
                }
                ui.add_space(20.0);
                if ui.button("Save").clicked() {
                    print(rorschach);
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.add_space(10.0);
            ui.heading("Color Palette");
            ui.add_space(40.0);
            egui::warn_if_debug_build(ui);

            let mut opt_texture: Option<egui::TextureHandle> = None;
            let texture: &egui::TextureHandle =
                opt_texture.get_or_insert_with(|| ui.ctx().load_texture("wave", generate(rorschach)));
            let img_size = 2.0 * texture.size_vec2();
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.image(texture, img_size)
            });
        });
    }
}
