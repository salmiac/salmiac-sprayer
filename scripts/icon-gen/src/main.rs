use std::fs;
use image::{RgbaImage, imageops::FilterType};
use resvg::usvg::{Options, Tree};

fn main() {
    let svg_data = fs::read("../../assets/logo.svg").unwrap();
    let opt = Options::default();
    let rtree = Tree::from_data(&svg_data, &opt).unwrap();
    
    let mut pixmap = resvg::tiny_skia::Pixmap::new(512, 512).unwrap();
    
    let rtree = resvg::Tree::from_usvg(&rtree);
    rtree.render(resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
    
    let img = RgbaImage::from_raw(512, 512, pixmap.data().to_vec()).unwrap();
    
    let sizes = [48, 64, 72, 96, 128, 144, 192, 256, 512];
    for &size in &sizes {
        let resized = image::imageops::resize(&img, size, size, FilterType::Lanczos3);
        resized.save(format!("../../assets/logo_{}.png", size)).unwrap();
        if size == 512 {
            resized.save("../../assets/logo.png").unwrap();
        }
    }
    
    let mut ico_file = fs::File::create("../../assets/icon.ico").unwrap();
    let resized_256 = image::imageops::resize(&img, 256, 256, FilterType::Lanczos3);
    use image::ImageEncoder;
    let ico_encoder = image::codecs::ico::IcoEncoder::new(&mut ico_file);
    ico_encoder.write_image(&resized_256, 256, 256, image::ColorType::Rgba8).unwrap();
    
    println!("Icons generated successfully!");
}
