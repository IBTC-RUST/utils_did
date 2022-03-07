use bip39::{Mnemonic, Language};

pub struct Mc {}

impl Mc {
    /// 生成助记词
    pub fn new_mnemonic() {
        let mut rng = rand::thread_rng();
        let m = Mnemonic::generate_in_with(&mut rng, Language::English, 12).unwrap();
        println!("{}", m);
    }
}