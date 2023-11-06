use std::error::Error;
use stringart::process_image;
fn main() -> Result<(), Box<dyn Error>> {
    let nombre = "pikachu2";
    let extension = "jpg";

    let input = format!("{}.{}",nombre,extension);
    let output0 = format!("zz1z-{}.{}",nombre,extension);
    let output1 = format!("zz2z-{}.{}",nombre,extension);
    //let output2 = format!("zz3z-{}.{}",nombre,extension);

    let img = image::open(&input)?;
    let imgbn = process_image(img,1500,256,21.0,1);
    imgbn.save(&output0)?;
    println!("listo!");
    let img = image::open(&input)?;
    let imgbn = process_image(img,1500,256,20.0,2);
    imgbn.save(&output1)?;
    /*println!("listo!");
    let img = image::open(&input)?;
    let imgbn = process_image(img,4000,240,17.5,3);
    imgbn.save(&output2)?;*/

//no se que falta me estoy durmiendo
//nada
    Ok(())
}

