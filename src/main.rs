use std::io::{Error, ErrorKind};

fn get_tile(x: u8, y: u8, z: u8) -> Result<image::DynamicImage, Error> {
    let url = format!("https://s3.amazonaws.com/elevation-tiles-prod/terrarium/{z}/{x}/{y}.png", x=x, y=y, z=z);
    let response = match reqwest::blocking::get(&url) {
        Ok(r) => r,
        Err(_) => return Err(Error::new(ErrorKind::Other, "tile http request failed")),
    };

    let img_bytes = match response.bytes() {
        Ok(b) => b,
        Err(_) => return Err(Error::new(ErrorKind::Other, "failed to convert downloaded image to bytes")),
    };

    let image_in_mem = match image::load_from_memory(&img_bytes) {
        Ok(im) => im,
        Err(_) => return Err(Error::new(ErrorKind::Other, "failed to load image from memory")),
    };

    Ok(image_in_mem)

}  

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = 1;
    let y = 1;
    let z = 10;

    let img = get_tile(x, y, z)?; 
    let filename = format!("{x}x_{y}y_{z}z.png", x=x, y=y, z=z);

    img.save(&filename)?;

    Ok(())
}
