pub mod point;
pub mod utils;
pub mod ecdh;

fn main() {
    let alice_private_key = ecdh::gen_priv_key(256);
    let bob_private_key = ecdh::gen_priv_key(256);   

    let temp1 = alice_private_key.clone();
    let temp2 = bob_private_key.clone();
    let alice_public_key = ecdh::gen_pub_key(alice_private_key);
    let bob_public_key = ecdh::gen_pub_key(bob_private_key); 

    

    let alice_shared_key = ecdh::gen_shared_key(bob_public_key, temp1);
    let bob_shared_key = ecdh::gen_shared_key(alice_public_key, temp2);

    assert_eq!(alice_shared_key, bob_shared_key);

}


