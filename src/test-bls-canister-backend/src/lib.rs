use ic_bls12_381::{
    hash_to_curve::{ExpandMsgXmd, HashToCurve},
    multi_miller_loop, G1Affine, G1Projective, G2Affine, G2Prepared, G2Projective, Gt, Scalar,
};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
