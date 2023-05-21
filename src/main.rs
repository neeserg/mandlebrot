use std::ops;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;


static  MAX_ITER: u32 = 100;
const   SIZE: usize = 10000;
static ZOOM: f64 = 2.0;
static RGBA: usize = 4;
static NUM_TRIALS:u8 = 5;
struct Complex{
    x: f64,
    y:f64
}

impl Complex {

    fn square(&self) -> f64 {
        self.x*self.x +self. y*self.y
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
                let magnitude: f64 = new_z.square();
                
                if  magnitude >4.0 {
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


fn color_map(iteration:i16) -> (u8,u8,u8,u8){
    //Custom color map for mandlebrot.
    let mut r:u8 = 255;
    let mut g:u8 = 255;
    let mut b:u8 = 255;
    let mut a:u8 = 255;
    if iteration == -1{
        // this means its part of the set.
        return  (r,g,b,a);
    }
    else if iteration >MAX_ITER as i16{
        return  (r,g,b,a);
    }
    else {
        let iteration_normailsed:f32 = (iteration as f32)/(MAX_ITER as f32);
        let iteration:u8 = (255.0*iteration_normailsed) as u8;
        (r,g,b,a) =(iteration,iteration,iteration,255);
        return  (r,g,b,a);
    }
}





fn get_mandlebrot_point(i:usize,j:usize) -> MandleBrotPoint{
    // Gets a Complex number wrapped in MandleBrotEnum
    let mut x = i as f64;
    let mut y = j as f64;
    let size_convert = SIZE as f64;
    x = (x/size_convert)*(ZOOM*2.0)-ZOOM;
    y = ZOOM-(y/size_convert)*(ZOOM*2.0);
    return  MandleBrotPoint::Point { z: Complex { x: 0.0, y: 0.0 }
        , iterations: 0, c: Complex { x: x, y: y }, coord: (i,j) };
}

fn save_image(pixels :Vec<u8>){
    // saves the image to a file.
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

fn mandlebrot_algrithm(){
    let mut pixels: Vec<u8> = vec![0; SIZE*SIZE*RGBA];
    for i in 0..SIZE{
        for j in 0..SIZE{
            let mut mandle_brot = get_mandlebrot_point(i,j);
            loop {
                match mandle_brot{
                    MandleBrotPoint::Point {..}=>{
                        mandle_brot = mandle_brot.next_point();
                    }
                    MandleBrotPoint::Color { iterations, 
                        coord }=>{
                        let (r,g,b,a) = color_map(iterations);
                        let (x,y) = coord;
                        pixels[y*(RGBA*SIZE) + RGBA*x ] = r ;
                        pixels[y*(RGBA*SIZE) + RGBA*x +1] = g ;
                        pixels[y*(RGBA*SIZE) + RGBA*x +2] = b ;
                        pixels[y*(RGBA*SIZE) + RGBA*x +3] = a ;
                        break;
                    }
                }

            }

        }
    }
    save_image(pixels);

}
fn main(){
    let now = Instant::now();

    for i in 0..NUM_TRIALS{
        let trial_before = Instant::now();
        mandlebrot_algrithm();
        let trial_duration:f32 = trial_before.elapsed().as_secs_f32();
        println!("Trial number {} finishes in: {} minutes {} seconds",i,(trial_duration as u32)/60,trial_duration%60.0);
    }
    
    let elapsed: f32 = now.elapsed().as_secs_f32();
    let elapsed_average: f32 = now.elapsed().as_secs_f32()/(NUM_TRIALS as f32);
    println!("Total time elapsed is: {} minutes {} seconds",(elapsed as u32)/60,elapsed%60.0);
    println!("Average seconds is: is: {} minutes {} seconds",(elapsed_average as u32)/60,elapsed_average%60.0);




    

    }


