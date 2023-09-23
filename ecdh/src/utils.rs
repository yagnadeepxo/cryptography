use num_bigint::{BigInt, BigUint, Sign, RandBigInt};
use num_traits::One;
use num_traits::Zero;
use rand::thread_rng;

pub fn mod_inv(a: &BigUint, module: &BigUint) -> Option<BigUint> {
    let zero = BigInt::zero();
    let one = BigInt::one();
    let mut mn = (
        BigInt::from_biguint(Sign::Plus, module.clone()),
        BigInt::from_biguint(Sign::Plus, a.clone()),
    );
    let mut xy = (zero.clone(), one.clone());

    while mn.1 != zero {
        xy = (xy.1.clone(), xy.0.clone() - (&mn.0 / &mn.1) * xy.1.clone());
        mn = (mn.1.clone(), mn.0 % mn.1.clone());
    }

    if mn.0 > one {
        return None;
    }

    let res = (xy.0 + BigInt::from(module.clone())) % BigInt::from(module.clone());
    Some(res.to_biguint().unwrap())
}

pub fn generate_random_bigint(bits: usize) -> BigInt {
    let mut rng = thread_rng();
    rng.gen_bigint(bits as u64)
}