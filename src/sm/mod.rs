use libsm::sm2::signature::{SigCtx, Signature};
use num_bigint::BigUint;

pub struct Sm {}

impl Sm {
    /// 生成公私钥
    pub fn new_master_key() -> (String, String) {
        let ctx = SigCtx::new();
        let (pk, sk) = ctx.new_keypair();
        let pk_v = ctx.serialize_pubkey(&pk, true);
        return (hex::encode(pk_v), sk.to_str_radix(16));
    }

    /// 私钥转公钥
    pub fn pri_to_pub(pri_key: &[u8]) -> String {
        let big_uint = BigUint::parse_bytes(pri_key, 16);
        let pri = big_uint.unwrap();
        let ctx = SigCtx::new();
        let pk_point = ctx.pk_from_sk(&pri);
        let pk_v = ctx.serialize_pubkey(&pk_point, true);
        hex::encode(pk_v)
    }

    /// 私钥签名
    pub fn sign(pri_key: &[u8], msg: &[u8]) -> String {
        let big_uint = BigUint::parse_bytes(pri_key, 16);
        let sk = big_uint.unwrap();

        let ctx = SigCtx::new();
        let pk = ctx.pk_from_sk(&sk);

        let signature = ctx.sign(msg, &sk, &pk);
        let signature_der = signature.der_encode();
        return hex::encode(signature_der);
    }

    /// 私钥签名
    pub fn verify(msg: &[u8], pub_key: &[u8], sig_der: &[u8]) -> bool {
        let ctx = SigCtx::new();
        let signature = Signature::der_decode(sig_der).unwrap();
        let pk = ctx.load_pubkey(pub_key).unwrap();

        let signature_der = signature.der_encode();
        println!("{}", hex::encode(signature_der));

        return ctx.verify(msg, &pk, &signature);
    }
}