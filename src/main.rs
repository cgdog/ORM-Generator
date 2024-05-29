#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]
use eframe::egui;
use egui::Ui as UI;
use image::io::Reader as ImageReader;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ORM Map Generator",
        options,
        Box::new(|_cc| {
            // egui_extras::install_image_loaders(&_cc.egui_ctx);
            Box::<ORMGeneratorApp>::default()
        }),
    )
}

struct ORMGeneratorApp {
    occlusion_path: String,
    roughness_path: String,
    metallic_path: String,
    generated_orm_path: String,
}

impl Default for ORMGeneratorApp {
    fn default() -> Self {
        Self {
            occlusion_path: "".to_owned(),
            roughness_path: "".to_owned(),
            metallic_path: "".to_owned(),
            generated_orm_path: "".to_owned(),
        }
    }
}

fn pick_texture_file(ui: &mut UI, texture_type: &'static str, texture_path: &mut String) {
    ui.horizontal(|ui| {
        if ui.button(format!("Choose {} file...", texture_type)).clicked() {
            if let Some(path) = rfd::FileDialog::new().add_filter("Image Files", &["png", "jpg", "jpeg"]).pick_file() {
                *texture_path = path.display().to_string();
            }
        }

        if texture_path != "" {
            ui.label(format!("{} map: {}", texture_type, texture_path));
        }
    });
}

impl ORMGeneratorApp {
    fn try_generate_ormmap(&mut self, ui: &mut UI) {
        if self.occlusion_path != "" && self.roughness_path != "" && self.metallic_path != "" {
            if ui.button("Generate ORM Map").clicked() {
                let occlusion_img = ImageReader::open( self.occlusion_path.to_owned()).unwrap().decode().unwrap();
                let width = occlusion_img.width();
                let height = occlusion_img.height();
                let occlusion_img = occlusion_img.to_rgb8();
                let roughness_img = ImageReader::open( self.roughness_path.to_owned()).unwrap().decode().unwrap();
                let roughness_img = roughness_img.to_rgb8();

                let metallic_img = ImageReader::open( self.metallic_path.to_owned()).unwrap().decode().unwrap();
                let metallic_img = metallic_img.to_rgb8();

                if width != roughness_img.width() || height != roughness_img.height() || width != metallic_img.width() || height != metallic_img.height() {
                    rfd::MessageDialog::new().set_title("Error!")
                    .set_level(rfd::MessageLevel::Error)
                    .set_description(format!("Input textures do not have same size, occlusion: {}x{}, roughness: {}x{}, metallic: {}x{}", width, height, 
                        roughness_img.width(), roughness_img.height(), metallic_img.width(), metallic_img.height()))
                    .set_buttons(rfd::MessageButtons::Ok).show();
                
                    return;
                }

                let mut imgbuf = image::ImageBuffer::new(width, height);
                for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                    let r = occlusion_img.get_pixel(x, y)[0];
                    let g = roughness_img.get_pixel(x, y)[1];
                    let b = metallic_img.get_pixel(x, y)[2];
                    *pixel = image::Rgb([r, g, b]);
                }

                let orm_path = rfd::FileDialog::new().add_filter("Image Files", &["png", "jpg", "jpeg"]).save_file().unwrap().display().to_string();
                imgbuf.save(orm_path.to_owned()).unwrap();
                
                rfd::MessageDialog::new().set_title("Done.")
                    .set_description(format!("The generated ORM Map: {}", orm_path))
                    .set_buttons(rfd::MessageButtons::Ok).show();

                self.generated_orm_path = orm_path;
            }
        }
    }
}

impl eframe::App for ORMGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            pick_texture_file(ui, "occlusion", &mut self.occlusion_path);
            pick_texture_file(ui, "roughness", &mut self.roughness_path);
            pick_texture_file(ui, "metallic", &mut self.metallic_path);            

            self.try_generate_ormmap(ui);

            if self.generated_orm_path != "" {
                ui.label(format!("Done. The generated ORM map: {}", self.generated_orm_path));
            }
        });
    }
}
