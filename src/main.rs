use num_bigint::BigInt;

#[derive(Debug)]
struct ProjectivePoint {
    x: BigInt,
    y: BigInt, 
    z: BigInt
}

fn point_double(p: &ProjectivePoint) -> ProjectivePoint {

    /*

    Given a point P = (X1, Y1, Z1)

    T = 3X1^2 + aZ1^2
    U = 2Y1Z1
    V = 2UX1*Y1
    W = T^2 - 2*V
    X3 = W*U^2
    Y3 = T*(V - W) - 2*(U*Y1)^2
    Z3 = U^3
    So the doubled point is (X3, Y3, Z3)

    value of a = 0 for secp256k1
    
     */

    let t = (3 * &p.x * &p.x) * &p.z * &p.z; 
    let u = 2 * &p.y * &p.z;
    let v = 2 * &u * &p.x * &p.y;
    let w = &t * &t - 2 * &v;
    
    let x3 = &w * &u * &u;
    let y3 = &t * (&v - &w) - 2 * (&u * &p.y) * (&u * &p.y);
    let z3 = &u * &u * &u;

    ProjectivePoint{x: x3, y: y3, z: z3}
}


fn point_add(p: &ProjectivePoint, q: &ProjectivePoint) -> ProjectivePoint {

    /*
    Given two points P = (X1, Y1, Z1) and Q = (X2, Y2, Z2)

    T0 = Y1*Z2
    T1 = Y2*Z1
    T = T0 - T1
    U0 = X1*Z2
    U1 = X2*Z1
    U = U0 - U1
    U2 = U^2
    V = Z1*Z2
    W = T^2V - U2(U0 + U1)
    X3 = W*U^2
    Y3 = T*(U0U2 - W) - T0U^3
    Z3 = U*U2
    So the added point is (X3, Y3, Z3)
    
    */
    
    let t0 = &p.y * &q.z;
    let t1 = &q.y * &p.z;
    let t = &t0 - &t1;
    
    let u0 = &p.x * &q.z; 
    let u1 = &q.x * &p.z;
    let u = &u0 - &u1;
    
    let u_squared = &u * &u;
    let v = &p.z * &q.z;
    
    let w = &t * &t * &v - &u_squared * (&u0 + &u1); 
    
    let x3 = &w * &u_squared;
    let y3 = &t * (&u0 * &u_squared - &w) - &t0 * &u * &u * &u;
    let z3 = &u * &u_squared;

    ProjectivePoint{x: x3, y: y3, z: z3}   
}


fn to_affine(p: &ProjectivePoint) -> (BigInt, BigInt) {
    let x = &p.x / &p.z;
    let y = &p.y / &p.z;
  
    (x, y)  
}


fn main() {
    
    let point1 = ProjectivePoint {
        x: BigInt::from(1), 
        y: BigInt::from(2),
        z: BigInt::from(1),
    };

    let point2 = ProjectivePoint {
        x: BigInt::from(4), 
        y: BigInt::from(5),
        z: BigInt::from(1),
    };


    // Test point doubling.
    let doubled_point = point_double(&point1);
    println!("Doubled Point: x={}, y={}, z={}", doubled_point.x, doubled_point.y, doubled_point.z);

    let affine_point = to_affine(&doubled_point);
    println!("Affine Point: x={}, y={}", affine_point.0, affine_point.1);

    // Test point addition.
    let added_point = point_add(&point1, &point2);
    println!("Added Point: x={}, y={}, z={}", added_point.x, added_point.y, added_point.z);

    let affine_point = to_affine(&added_point);
    println!("Affine Point: x={}, y={}", affine_point.0, affine_point.1);

}









