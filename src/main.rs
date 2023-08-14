use std::error::Error;
use stringart::process_image;
fn main() -> Result<(), Box<dyn Error>> {
    let nombre = "pikachu";
    let extension = "png";

    let input = format!("{}.{}",nombre,extension);
    let output0 = format!("{}0.{}",nombre,extension);
    let output = format!("{}1.{}",nombre,extension);

    let img = image::open(&input)?;
    let imgbn = process_image(img,50000,360,0.0);
    imgbn.save(&output0)?;
    println!("listo!");
    let img = image::open(&input)?;
    let imgbn = process_image(img,50000,360,10.0);
    imgbn.save(&output)?;


//no se que falta me estoy durmiendo
//nada
    Ok(())
}

