use std::ops;
use std::convert::TryFrom;


static  MAX_ITER: u32 = 100;
const   SIZE: usize = 10000;
static CENTER:(f64,f64) = (0.0,0.0);
static ZOOM: f64 = 2.0;


struct Complex{
    x: f64,
    y:f64
}

impl Complex {

    fn conjugate(&self) ->Complex{
        Complex{x:self.x,y:self.y}
    }
    fn magnitude(&self) -> f64 {
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
        coord: (u32,u32)},
    Color{
        iterations:i16,
        coord: (u32,u32)}
}

impl  MandleBrotPoint
 {
    fn nextPoint(point: MandleBrotPoint) -> MandleBrotPoint{
        match point {
            MandleBrotPoint::Point{z,
                iterations,c,coord} =>{
                
                
                let new_z: Complex = &z*&z + &c;
                let iterations: i16 = iterations+1;
                let magnitude: f64 = new_z.magnitude();
                
                if iterations >= MAX_ITER as i16 || magnitude >2.0 {
                    return  MandleBrotPoint::Color { iterations: iterations,
                        coord:coord };
                }
                else if magnitude < 1.0 {
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
                return  point;
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
        let MAX_ITER: u8 = MAX_ITER as u8;
        if iteration == MAX_ITER{
            (r,g,b) = (255,255,255);
        }
        else if iteration < MAX_ITER && iteration > 60{
            let iter_share: u8 = (iteration-60)/3;
            (r,g,b) = (0+iter_share,255-iter_share,221+iter_share)
        }
        return  (r,g,b);
    }
}

fn main(){
    let mut PIXELS: [u8;SIZE*SIZE*3] = [0; SIZE*SIZE*3];

}


