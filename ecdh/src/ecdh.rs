use num_bigint::BigInt;
use crate::point::{ProjectivePoint, scalar_mult};
use crate::utils::generate_random_bigint;

pub fn gen_priv_key(bits: usize) -> BigInt{
    generate_random_bigint(bits)
}

pub fn gen_pub_key(priv_key: BigInt) -> BigInt {
    let x = BigInt::from(9);
    let y = BigInt::parse_bytes(b"14781619447589544791020593568409986887264606134616475288964881837755586237401", 10).unwrap();
    let z = BigInt::from(1);
    let base = ProjectivePoint {x, y, z};
    let pub_projective_point = scalar_mult(base, priv_key);
    let pub_key = ProjectivePoint::to_affine(&pub_projective_point).unwrap();
    pub_key.x
}

pub fn gen_shared_key(pub_key: BigInt, priv_key: BigInt) -> BigInt {
    let g = BigInt::from(9);
    let temp = priv_key*pub_key;
    g*temp
}


