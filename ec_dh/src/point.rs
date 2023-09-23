use num_bigint::{BigInt, BigUint};
use num_traits::Num;
use num_bigint::Sign::Plus;
use crate::utils;

#[derive(Debug,Clone)]
pub struct ProjectivePoint {
    pub x: BigInt,
    pub y: BigInt,
    pub z: BigInt 
}


pub struct AffinePoint {
    pub x: BigInt,
    y: BigInt
}

impl ProjectivePoint {
    pub fn new(x: BigInt, y: BigInt, z: BigInt) -> Self {
        ProjectivePoint {x : x, y: y, z: z}
    }


    pub fn point_add(p: ProjectivePoint, q: ProjectivePoint) -> ProjectivePoint {
      // p = (x1, y1, z1), q = (x2, y2, z2)
  
      let x1 = p.x.clone();
      let y1 = p.y.clone();
      let z1 = p.z.clone();
  
      let x2 = q.x.clone();
      let y2 = q.y.clone();
      let z2 = q.z.clone();
  
      if z1 == BigInt::from(0) {
          return q; // Return q as the result
      } else if z2 == BigInt::from(0) {
          return p; // Return p as the result
      }
  
      let z1z2 = &z1 * &z2;
  
      let x1y2 = &x1 * &y2;
      let x2y1 = &x2 * &y1;
      let y1y2 = &y1 * &y2;
      let x1x2 = &x1 * &x2;
  
      let z1z2_squared = &z1z2 * &z1z2;
  
      let x3 = &z1z2_squared * (&x2y1 - &x1y2);
  
      let z3 = &z1z2_squared * (&x1y2 - &x2y1);
      let z1z3 = &z1 * &z3;
      let z1z3_square = &z1z3 * &z1z3;
      let y3 = &z1z3_square * &z1z3 * (&y1y2 - &x1x2);
  
      ProjectivePoint { x: x3, y: y3, z: z3 }
  }
  

    pub fn point_double(p: ProjectivePoint) -> ProjectivePoint {
        // let p = (x1, y1, z1)
        let a = BigInt::from(486662);
        let two = BigInt::from(2);
        let three = BigInt::from(3);

  
        let x1 = p.x.clone();
        let y1 = p.y.clone();
        let z1 = p.z.clone();

        if z1 == BigInt::from(0) {
          return p;  // Return the original point as the result
      }

        let y1z1 = &y1*&z1;
        let y1z1_squared = &y1z1*&y1z1;
        let y1z1_cube = &y1z1_squared*&y1z1;
        let x1_squared = &x1*&x1;
        let z1_squared = &z1*&z1;
        let z1_quad = &z1_squared*&z1_squared;
        let y1_squared = &y1*&y1;

        let x3 = &y1z1_squared*(&three*&x1_squared + &a*&z1_quad);
        let y3 = &y1z1_cube*(&three*&x1_squared - &y1_squared);
        let z3 = &two*&y1z1*(&x1*&y1);

        ProjectivePoint { x: x3, y: y3, z: z3 }

    }

    pub fn to_affine(&self) -> Option<AffinePoint> {

        let p_hex = "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED";
        let modulus = BigUint::from_str_radix(p_hex, 16).unwrap();
    
        if self.z == BigInt::from(0) {
          return Some(AffinePoint {
            x: BigInt::from(0),
            y: BigInt::from(0),
        });
        }

        let a = &self.z.to_biguint().unwrap();

        let temp = utils::mod_inv(a, &modulus).unwrap();

        let z_inv = BigInt::from_biguint(Plus, temp.clone());
    
        let x = &self.x*&z_inv;
        let y = &self.y*&z_inv;
    
        Some(AffinePoint {
          x,
          y 
        })
      }


}

fn is_odd(val: &BigInt) -> bool {
    val & BigInt::from(1) == BigInt::from(1)  
}

pub fn scalar_mult(base: ProjectivePoint, scalar: BigInt) -> ProjectivePoint {

    let mut res = ProjectivePoint::new(BigInt::from(0), BigInt::from(1), BigInt::from(0));
  
    let mut remaining = scalar.clone();
    let mut addend = base.clone();
  
    while remaining > BigInt::from(0) {
  
      if is_odd(&remaining) {
        let addend_copy = addend.clone();
        res = ProjectivePoint::point_add(res, addend_copy);
      }
  
      addend = ProjectivePoint::point_double(addend);
  
      remaining = remaining >> 1; 
    }
    res
}


