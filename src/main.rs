#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]
use eframe::egui;
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
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    occlusion_path: String,
    roughness_path: String,
    metallic_path: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            occlusion_path: "".to_owned(),
            roughness_path: "".to_owned(),
            metallic_path: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            // pick ao map
            ui.horizontal(|ui| {
                if ui.button("Choose occlusion file...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.occlusion_path = path.display().to_string();
                    }
                }

                if self.occlusion_path != "" {
                    ui.label(format!("Occlusion Map: {}", self.occlusion_path));
                }
            });

            ui.horizontal(|ui| {
                if ui.button("Choose roughness file...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.roughness_path = path.display().to_string();
                    }
                }

                if self.roughness_path != "" {
                    ui.label(format!("Roughness Map: {}", self.roughness_path));
                }
            });
            
            ui.horizontal(|ui| {
                if ui.button("Choose metallic file...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.metallic_path = path.display().to_string();
                    }
                }

                if self.metallic_path != "" {
                    ui.label(format!("Metallic Map: {}", self.metallic_path));
                }
            });
            

            if self.occlusion_path != "" && self.roughness_path != "" && self.metallic_path != "" {
                if ui.button("Generate ORM Map").clicked() {
                    let occlusion_img = ImageReader::open( self.occlusion_path.to_owned()).unwrap().decode().unwrap();
                    let width = occlusion_img.width();
                    let height = occlusion_img.height();
                    let occlusion_img = occlusion_img.to_rgb8();
                    // let pixel = occlusion_img.get_pixel(0, 0);
                    let roughness_img = ImageReader::open( self.roughness_path.to_owned()).unwrap().decode().unwrap();
                    let roughness_img = roughness_img.to_rgb8();

                    let metallic_img = ImageReader::open( self.metallic_path.to_owned()).unwrap().decode().unwrap();
                    let metallic_img = metallic_img.to_rgb8();


                    let mut imgbuf = image::ImageBuffer::new(width, height);
                    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                        let r = occlusion_img.get_pixel(x, y)[0];
                        let g = roughness_img.get_pixel(x, y)[1];
                        let b = metallic_img.get_pixel(x, y)[2];
                        *pixel = image::Rgb([r, g, b]);
                    }

                    let orm_path = rfd::FileDialog::new().save_file().unwrap().display().to_string();
                    imgbuf.save(orm_path).unwrap();
                }
            }
        });
    }
}
