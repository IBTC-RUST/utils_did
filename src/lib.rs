extern crate libsm;
extern crate bip39;
extern crate num_bigint;

pub mod mc;
pub mod sm;

#[cfg(test)]
mod tests {
    use crate::mc::Mc;
    use crate::sm::Sm;

    #[test]
    fn mc_test1() {
        Mc::new_mnemonic();
    }

    #[test]
    fn sm_test1() {
        let b = Sm::new_master_key();
        println!("{} {}", b.0, b.1)
    }

    #[test]
    fn sm_test2() {
        let bytes = b"ced586c083cb59fe11b6bde0984a2411d471bcc564e9a8667c4c4c0b02cae9e7";
        let b = Sm::pri_to_pub(bytes);
        println!("{}", b)
    }

    #[test]
    fn sm_test3() {
        let pri_key = b"bf0a4549f58d1d436764b6d758a7aabd243081d0580efa04a6707d6950669d7d";
        let msg = b"123";
        let b = Sm::sign(pri_key, msg);
        println!("{}", b);
    }

    #[test]
    fn sm_test4() {
        let pub_key = b"039cae08b2f6f0d02cf600fd7f36698e07968571e9aa33006db24e5eb70aad6800";
        let msg = b"123";
        let signature = b"30450221008651c47d596cdcfc47875295ba77459f59afcb126efee5e0798f1ea5b8f58201022010cfbe005492eb3ac400c5ff9ec126624fb4826e41d438f28f9a5f5275c0751a";
        let sig = hex::decode(signature).unwrap();
        let _pub = hex::decode(pub_key).unwrap();
        let b = Sm::verify(msg, &_pub, &sig);
        println!("{:?}", b);
    }
}
