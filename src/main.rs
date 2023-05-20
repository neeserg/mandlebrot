use std::ops;
use std::collections::LinkedList;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;


static  MAX_ITER: u32 = 100;
const   SIZE: usize = 10000;
static ZOOM: f64 = 2.0;
static RGBA: usize = 4;

struct Complex{
    x: f64,
    y:f64
}

impl Complex {

    fn magnitude(&self) -> f64 {
        f64::sqrt(self.x*self.x +self. y*self.y) 
    }

}

impl ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex { x: self.x + other.x, y: self.y + other.y }
    }
}


impl ops::Add<&Complex> for Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex { x: self.x + other.x, y: self.y + other.y }
    }
}

impl ops::Add for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex { x: self.x + other.x, y: self.y + other.y }
    }
}
impl ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex { x: self.x*other.x - self.y*other.y, y: 2.0*(self.y*self.x )}
    }
}
impl ops::Mul<&Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: &Complex) -> Complex {
        Complex { x: self.x*other.x - self.y*other.y, y: 2.0*(self.y*self.x )}
    }
}
impl ops::Mul for &Complex {
    type Output = Complex;

    fn mul(self, other: &Complex) -> Complex {
        Complex { x: self.x*other.x - self.y*other.y, y: 2.0*(self.y*self.x )}
    }
}





enum MandleBrotPoint {
    Point{ z: Complex,
        iterations:i16,
        c:Complex,
        coord: (usize,usize)},
    Color{
        iterations:i16,
        coord: (usize,usize)}
}

impl  MandleBrotPoint
 {
    fn next_point(self) -> MandleBrotPoint{
        match self {
            MandleBrotPoint::Point{z,
                iterations,c,coord} =>{
                
                
                let new_z: Complex = &z*&z + &c;
                let iterations: i16 = iterations+1;
                let magnitude: f64 = new_z.magnitude();
                
                if  magnitude >2.0 {
                    return  MandleBrotPoint::Color { iterations: iterations,
                        coord:coord };
                }
                else if iterations >= MAX_ITER as i16 {
                    return  MandleBrotPoint::Color { iterations: -1,
                        coord:coord};
                }
                else {
                    return MandleBrotPoint::Point { z: new_z, 
                        iterations: iterations, 
                        c: c,coord:coord };
                }
            }
            MandleBrotPoint::Color{..}=>{
                return  self;
            }
        }
    }
}


fn color_map(iteration:i16) -> (u8,u8,u8){
    let mut r:u8 = 0;
    let mut g:u8 = 0;
    let mut b:u8 = 0;
    if iteration == -1{
        return  (r,g,b);
    }
    else {
        let iteration:u8 = iteration as u8;
        if iteration >= MAX_ITER as u8{
            (r,g,b) = (0,0,0);
        }
        else if iteration < MAX_ITER as u8 && iteration > 60{
            // bluish tint.
            (r,g,b) = (34-iteration/3,255-iteration,221+iteration/3)
        }
        else if iteration <= 60 && iteration > 30{
            // greenish tint
            (r,g,b) = (120-iteration,194+iteration,160-iteration)
        }
        else if iteration <= 30 && iteration > 9 {
            // redish tint
            (r,g,b) = (224+iteration,198-2*iteration,160-2*iteration)
        }
        else {
            (r,g,b) = (255-iteration,2*iteration,2*iteration)
        }
        return  (r,g,b);
    }
}





fn get_mandlebrot_point(i:usize,j:usize) -> MandleBrotPoint{
    let mut x = i as f64;
    let mut y = j as f64;
    let size_convert = SIZE as f64;
    x = (x/size_convert)*(ZOOM*2.0)-ZOOM;
    y = ZOOM-(y/size_convert)*(ZOOM*2.0);
    return  MandleBrotPoint::Point { z: Complex { x: 0.0, y: 0.0 }
        , iterations: 0, c: Complex { x: x, y: y }, coord: (i,j) };
}

fn main(){
    println!("Start");
    let mut pixels: Vec<u8> = vec![0; SIZE*SIZE*RGBA];
    let mut mandle_brot_set:LinkedList<MandleBrotPoint> = LinkedList::new();
    let mut mandle_brot_set_second:LinkedList<MandleBrotPoint> = LinkedList::new();
    println!("it gets here!");
    for i in 0..SIZE{
        for j in 0..SIZE{
            mandle_brot_set.push_front(get_mandlebrot_point(i,j));
        }
    }

    for i in 0..MAX_ITER+1{
        println!("{i}");
        while !mandle_brot_set.is_empty() {
            match mandle_brot_set.pop_back(){
                Some(val)=>{
                    match val {
                        MandleBrotPoint::Point {..}=>{
                                
                            mandle_brot_set_second.push_back(val.next_point());

                            }
                        MandleBrotPoint::Color { iterations,coord }=>{
                            let (r,g,b) = color_map(iterations);
                            let (x,y) = coord;
                            pixels[y*(RGBA*SIZE) + RGBA*x ] = r ;
                            pixels[y*(RGBA*SIZE) + RGBA*x +1] = g ;
                            pixels[y*(RGBA*SIZE) + RGBA*x +2] = b ;
                            pixels[y*(RGBA*SIZE) + RGBA*x +3] = 255 ;
                        }
                        }
                    }
                None=>{
                    break;
                }
            
            }

        }

        (mandle_brot_set_second,mandle_brot_set) = (mandle_brot_set,mandle_brot_set_second);
    }



    let path = Path::new(r"mandlebrot.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, SIZE as u32, SIZE as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap(); // Save



    }


