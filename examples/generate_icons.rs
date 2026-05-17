use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let svg_data = fs::read("assets/logo.svg")?;
    
    let opt = resvg::usvg::Options::default();
    let rtree = resvg::usvg::Tree::from_data(&svg_data, &opt)?;
    
    let sizes = [64, 128, 256, 512];
    
    for size in sizes {
        let mut pixmap = tiny_skia::Pixmap::new(size, size).unwrap();
        let transform = tiny_skia::Transform::from_scale(
            size as f32 / 512.0,
            size as f32 / 512.0,
        );
        
        resvg::render(&rtree, transform, &mut pixmap.as_mut());
        
        let path = format!("assets/logo_{}.png", size);
        pixmap.save_png(&path)?;
        println!("Generated {}", path);
    }
    
    // Also save a standard logo.png for documentation
    fs::copy("assets/logo_512.png", "assets/logo.png")?;
    println!("Generated assets/logo.png");

    Ok(())
}
