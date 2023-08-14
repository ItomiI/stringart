use std::error::Error;
use stringart::process_image;
fn main() -> Result<(), Box<dyn Error>> {
    let nombre = "grudon";
    let extension = "jpg";

    let input = format!("{}.{}",nombre,extension);
    let output0 = format!("{}0.{}",nombre,extension);
    let output = format!("{}1.{}",nombre,extension);
    let output2 = format!("{}2.{}",nombre,extension);
    let output3 = format!("{}3.{}",nombre,extension);

    let img = image::open(&input)?;
    let imgbn = process_image(img,30000,180,0.0);
    imgbn.save(&output0)?;

    let img = image::open(&input)?;
    let imgbn = process_image(img,30000,180,6.0);
    imgbn.save(&output)?;

    let img2 = image::open(&input)?;
    let imgbn2 = process_image(img2,30000,180,10.0);
    imgbn2.save(&output2)?;

    let img3 = image::open(&input)?;
    let imgbn3 = process_image(img3,30000,180,20.0);
    imgbn3.save(&output3)?;

//no se que falta me estoy durmiendo
    Ok(())
}

