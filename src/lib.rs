use std::{f32::consts::PI, fs::OpenOptions, io::{BufWriter, self}, fmt::{Display, Debug}};
use image::{DynamicImage, GenericImageView, Luma};
use std::io::prelude::*;


pub fn process_image(input_image: DynamicImage,hilos:u32,clavos:u32,menos:f32,id:u32) -> DynamicImage {
    let (width, height) = input_image.dimensions();
    let radio;
    if width > height{
        radio = height/2;
    }else{
        radio = width/2;
    }

    let centro_x = width / 2;
    let centro_y = height / 2;

    let mut imgbn = input_image.to_luma8();
    let grosor_borde = 2; 

    draw_circle_border(&mut imgbn, centro_x, centro_y, radio, grosor_borde);//sacable

  
    imgbn.save("zzz-gris.png").unwrap();
/////////////////////////
    let mut dibujo = image::GrayImage::from_pixel(imgbn.width(), imgbn.height(),Luma([255]));
    let radio = (radio-5) as f32;
    let mut v = vec![];

    for i in 1 ..= clavos{//la pos de cada clavo en un circulo imaginario BIEN | sencos((2 pi * i / 128)+pi/2) 
        let calc = ((2.0 * i as f32  /clavos as f32)) * PI + PI/2.0;
        let x = f32::floor((imgbn.width()/2) as f32 + radio * f32::cos(calc)) as u32;
        let y = f32::floor((imgbn.height()/2) as f32 + radio *f32::sin(calc)) as u32;
        let p = Punto::new(i,x,y);
        v.push(p);
        dibujo.put_pixel(x, y, Luma([0]));
    }
    
    let mut ultimo_lugar = v[0].clone();//ultimo lugar vector
    let mut pasos = Pasos::new();
    for _  in 0..hilos{//cantidad de lineas
        
        let start = ultimo_lugar;
        let pixelss: Vec<Punto>;//vector de sumas de negro (cada linea)
        let mut max_encontrado:Punto = Punto::new(1000,0,0);//la suma maxima encontrada

        for punto_del_circulo in &v{//punto del circulo
            let pixels = pixel_linea(start,*punto_del_circulo);
            let mut temp_suma: f32 = 0.0;
        
            for pixel in &pixels{//punto de la linea..... cambiar la forma de sumar, talvez que saque por cada coso que toca
                let a = imgbn.get_pixel(pixel.x, pixel.y);
                temp_suma += (u8::MAX-a[0]) as f32;
            }
            if temp_suma > menos*pixels.len() as f32{
               // temp_suma -=menos*pixels.len() as f32// no se que es
            }else {
                temp_suma=0.0;
            }

           
            if temp_suma as u32 >= max_encontrado.get_intensidad() as u32{
                max_encontrado.id = punto_del_circulo.id;
                max_encontrado.x = punto_del_circulo.x;
                max_encontrado.y = punto_del_circulo.y;
                max_encontrado.set_intensidad(temp_suma as u32);         
            }  
        }

        if max_encontrado.intensida < 900{
            println!("se consumio en el hilo: {}",pasos.count+1);
            break;
        }
        pixelss = pixel_linea(start, max_encontrado);

        for p in pixelss{//dibuja
  
            let mut pixel_img = imgbn.get_pixel(p.x, p.y).clone();
            if pixel_img[0] <= 235{
                pixel_img[0] +=20;
            }
            imgbn.put_pixel(p.x, p.y, pixel_img);

            let mut pixel_dib = dibujo.get_pixel(p.x, p.y).clone();
            if pixel_dib[0] >= 20{
                pixel_dib[0]-=20;
            }
            dibujo.put_pixel(p.x, p.y, pixel_dib);
        }
        //calculo el movimiento
        //p1 = start p2 = max_encontrado c = clavos
        //im a genious o como se escriba
        let p1 = start.id;
        let p2 = max_encontrado.id;
        let mut paso:Paso;
        if p1 < p2{
            if p2 - p1 < clavos - p2 + p1{
                paso = Paso::new_p(p2-p1);
            }else{
                paso = Paso::new_n(clavos - p2 + p1);
            }
        }else if p1 > p2{
            if p1-p2 < clavos - p1 + p2{
                paso = Paso::new_n(p1-p2);
            }else{
                paso = Paso::new_p(clavos - p1 + p2);
            }
        }else{
            panic!("no se");
        }

        ultimo_lugar = max_encontrado;
        //lo cambio a la cantidad de pasos para el motor
        
        match paso {
            Paso::Neg(b) => {
                paso = Paso::new_n(b);//2048 / 256 / 4 // no se porque habia un *2
            },
            Paso::Pos(b) => {
                paso = Paso::new_p(b);
            },
        }
        pasos.add(paso);

    } 
    let file_path = format!("z{}zz-output.txt",id);
    let file = OpenOptions::new().create(true).append(true).open(&file_path).unwrap();
    let file = BufWriter::new(file);
    let mut output = io::LineWriter::new(file);
    writeln!(output, "const PROGMEM int[{}] pasos = ",pasos.count).unwrap();
    writeln!(output, "{}",pasos).unwrap();
    imgbn.save("zzz-gastado.png").unwrap();
    DynamicImage::ImageLuma8(dibujo)
    
}

fn draw_circle_border(image: &mut image::GrayImage, centro_x: u32, centro_y: u32, radio: u32, grosor_borde: u32) {

    for y in 0..image.height() {
        for x in 0..image.width() {

            let dx = x as f32 - centro_x as f32;//longitud entre pixel y centro
            let dy = y as f32 - centro_y as f32;
            let distancia_al_centro: f32 = f32::sqrt(dx * dx + dy * dy);
            let distancia_max = (radio + grosor_borde) as f32;
            let distancia_min = radio as f32;
            
            if  distancia_al_centro > distancia_min as f32 && distancia_al_centro < distancia_max {
                image.put_pixel(x, y, Luma([0]));
                
            }else if distancia_al_centro > distancia_min{
                image.put_pixel(x, y, Luma([255]));
            }
        }
    }
}

fn pixel_linea(point1: Punto, point2: Punto) -> Vec<Punto> {
    let (x1, y1) = point1.condenadas();
    let (x2, y2) = point2.condenadas();

    let mut pixels_on_line: Vec<Punto> = Vec::new();

    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let mut x = x1 as i32;
    let mut y = y1 as i32;
    let x_inc = if x2 >= x1 { 1 } else { -1 };
    let y_inc = if y2 >= y1 { 1 } else { -1 };
    let mut error = if dx > dy { dx } else { -dy } / 2;
    let mut prev_x = x;
    let mut prev_y = y;

    let mut id = 0;
    loop {
        
        pixels_on_line.push(Punto::new(id,x as u32, y as u32));
        
        if x == x2 as i32 && y == y2 as i32 {
            break;
        }

        let error2 = error * 2;

        if error2 > -dx {
            error -= dy;
            x += x_inc;
        }
        if error2 < dy {
            error += dx;
            y += y_inc;
        }

        // Si hay cambio en la dirección vertical o horizontal, almacenar el píxel anterior
        if x != prev_x || y != prev_y {
            pixels_on_line.push(Punto::new(id,prev_x as u32, prev_y as u32));
            prev_x = x;
            prev_y = y;
        }
        id+=1;
    }

    pixels_on_line
}


#[derive(Clone, Copy,Debug)]
struct Punto {
    id:u32,
    x:u32,
    y:u32,
    intensida:u32
}
impl PartialEq for Punto {
    fn eq(&self, other: &Self) -> bool {
       self.x == other.x && self.y == other.y
    }
}
impl Punto {
    pub fn new(id:u32,x:u32,y:u32)-> Punto{
        Punto { id, x, y, intensida:0 }
    }
    pub fn condenadas(&self)->(u32,u32){
        (self.x,self.y)
    }

    pub fn get_intensidad(& self)->u32{
        self.intensida
    }
    pub fn set_intensidad(&mut self,n:u32)->u32{
        self.intensida = n;
        self.intensida
    }
}
enum Paso{
    Neg(u32),
    Pos(u32)
}
impl Paso {
    pub fn new_p(n:u32)-> Paso{
        Self::Pos(n)
    }
    pub fn new_n(n:u32)-> Paso{
        Self::Neg(n)
    }
}
impl Debug for Paso {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neg(arg0) => write!(f,"-{}",arg0),
            Self::Pos(arg0) => write!(f,"{}",arg0),
        }
    }
}
impl Display for Paso {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Paso::Pos(n) => write!(f, "{n}"  ),
            Paso::Neg(n) => write!(f, "-{n}" ),
        }
    }
}

struct Pasos{
    pasos:Vec<Paso>,
    count:u16
}
impl Pasos {
    pub fn new()->Pasos{
        Pasos { pasos: vec![],count:0 }
    }
    pub fn add(&mut self,n:Paso){
        self.pasos.push(n);
        self.count +=1;
    }
}
impl Display for Pasos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut a = String::new();
        for i in &self.pasos{
            a.push_str(&i.to_string());
            a.push(',');
            
        }
        a.push('0');
        write!(f,"{{ {} }}",a)
    }
}