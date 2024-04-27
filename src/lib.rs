#![doc = include_str!("../readme.md")]

use litrs::StringLit;
use proc_macro::TokenStream;

use std::{fs, io, path::Path};
use quote::quote;

use digest::{generic_array::GenericArray, Digest, DynDigest, OutputSizeUser};
use walkdir::WalkDir;

fn hash_dir(path: impl AsRef<Path>, mut hasher: Box<dyn DynDigest>) -> io::Result<Box<[u8]>> {

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if entry.file_type().is_file() {
            let bytes = fs::read(entry.path())?;
            hasher.update(&bytes);
        }
    }

    let hash = hasher.finalize();
    Ok(hash)
}


///Generate a checksum from all files in a directory. When relevant, the 256 variant of hashers are
///used.
///```rust
///use checksum_dir::checksum;
///const CHECKSUM: [u8; 32] = checksum!("./src");
///````
#[proc_macro]
pub fn checksum(input: TokenStream) -> TokenStream {

    let input = input.into_iter().collect::<Vec<_>>();

    if input.len() != 1 {
        let msg = format!("expected exactly one input token, got {}", input.len());
        return quote! { compile_error!(#msg) }.into();
    }

    let string_lit = match StringLit::try_from(unsafe { input.get_unchecked(0) }) {
        Err(e) => return e.to_compile_error(),
        Ok(lit) => lit,
    };

    let path = string_lit.value().to_owned();
    let mut hasher = get_hasher();

    let hash = hash_dir(path, hasher).unwrap();

    quote! { [ #(#hash),* ] }.into()
}

macro_rules! init_hashers {
    ($($hasher: ty; $lit: literal),*) => {
        $(
            #[cfg(feature = $lit)] 
            return Box::new(<$hasher>::new());
         )*
    };
}

fn get_hasher() -> Box<dyn DynDigest> {
    init_hashers! {
    blake2::Blake2s256; "blake2",
    ascon_hash::AsconHash; "ascon-hash",
    belt_hash::BeltHash; "belt-hash",
    fsb::Fsb256; "fsb",
    gost94::Gost94CryptoPro; "gost94",
    groestl::Groestl256; "groestl",
    jh::Jh256; "jh",
    md2::Md2; "md2",
    md4::Md4; "md4",
    md5::Md5; "md5",
    ripemd::Ripemd256; "ripemd",
    skein::Skein256<skein::consts::U32>; "skein",
    sm3::Sm3; "sm3",
    streebog::Streebog256; "streebog",
    tiger::Tiger; "tiger",
    whirlpool::Whirlpool; "whirlpool",
    sha1::Sha1; "sha1",
    sha1_checked::Sha1; "sha1-checked",
    sha2::Sha256; "sha2",
    sha3::Sha3_256; "sha3"
    }

    unreachable!("no hash algorithm has been selected")
}

