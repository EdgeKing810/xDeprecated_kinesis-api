//! Easily hash and verify passwords using bcrypt
#![forbid(unsafe_code)]

extern crate alloc;

use blowfish::Blowfish;

#[cfg(feature = "std")]
use std::error;
#[cfg(feature = "std")]
use std::io;

use std::iter::repeat_with;

/// Library generic result type.
pub type BcryptResult<T> = Result<T, BcryptError>;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use core::{fmt, str::FromStr};
#[cfg(any(feature = "alloc", feature = "std"))]
use {core::convert::AsRef};

// Cost constants
const MIN_COST: u32 = 4;
const MAX_COST: u32 = 31;
pub const DEFAULT_COST: u32 = 12;

#[derive(Debug, PartialEq)]
/// A bcrypt hash result before concatenating
pub struct HashParts {
    cost: u32,
    salt: String,
    hash: String,
}

/// BCrypt hash version
/// https://en.wikipedia.org/wiki/Bcrypt#Versioning_history
pub enum Version {
    TwoA,
    TwoX,
    TwoY,
    TwoB,
}

impl HashParts {
    /// Creates the bcrypt hash string from all its parts
    fn format(self) -> String {
        self.format_for_version(Version::TwoB)
    }

    /// Get the bcrypt hash cost
    pub fn get_cost(&self) -> u32 {
        self.cost
    }

    /// Get the bcrypt hash salt
    pub fn get_salt(&self) -> String {
        self.salt.clone()
    }

    /// Creates the bcrypt hash string from all its part, allowing to customize the version.
    pub fn format_for_version(&self, version: Version) -> String {
        // Cost need to have a length of 2 so padding with a 0 if cost < 10
        alloc::format!("${}${:02}${}{}", version, self.cost, self.salt, self.hash)
    }
}

impl FromStr for HashParts {
    type Err = BcryptError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        split_hash(s)
    }
}

impl ToString for HashParts {
    fn to_string(&self) -> String {
        self.format_for_version(Version::TwoY)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Version::TwoA => "2a",
            Version::TwoB => "2b",
            Version::TwoX => "2x",
            Version::TwoY => "2y",
        };
        write!(f, "{}", str)
    }
}

fn setup(cost: u32, salt: &[u8], key: &[u8]) -> Blowfish {
    assert!(cost < 32);
    let mut state = Blowfish::bc_init_state();

    state.salted_expand_key(salt, key);
    for _ in 0..1u32 << cost {
        state.bc_expand_key(key);
        state.bc_expand_key(salt);
    }

    state
}

pub fn bcrypt(cost: u32, salt: &[u8], password: &[u8], output: &mut [u8]) {
    assert!(salt.len() == 16);
    assert!(!password.is_empty() && password.len() <= 72);
    assert!(output.len() == 24);

    let state = setup(cost, salt, password);
    // OrpheanBeholderScryDoubt
    #[allow(clippy::unreadable_literal)]
    let mut ctext = [
        0x4f727068, 0x65616e42, 0x65686f6c, 0x64657253, 0x63727944, 0x6f756274,
    ];
    for i in 0..3 {
        let i: usize = i * 2;
        for _ in 0..64 {
            let (l, r) = state.bc_encrypt(ctext[i], ctext[i + 1]);
            ctext[i] = l;
            ctext[i + 1] = r;
        }

        let buf = ctext[i].to_be_bytes();
        output[i * 4..][..4].copy_from_slice(&buf);
        let buf = ctext[i + 1].to_be_bytes();
        output[(i + 1) * 4..][..4].copy_from_slice(&buf);
    }
}

/// The main meat: actually does the hashing and does some verification with
/// the cost to ensure it's a correct one
fn _hash_password(password: &[u8], cost: u32, salt: &[u8]) -> BcryptResult<HashParts> {
    if !(MIN_COST..=MAX_COST).contains(&cost) {
        return Err(BcryptError::CostNotAllowed(cost));
    }
    if password.contains(&0u8) {
        return Err(BcryptError::InvalidPassword);
    }

    // Output is 24
    let mut output = [0u8; 24];
    // Passwords need to be null terminated
    let mut vec = Vec::with_capacity(password.len() + 1);
    vec.extend_from_slice(password);
    vec.push(0);
    // We only consider the first 72 chars; truncate if necessary.
    // `bcrypt` below will panic if len > 72
    let truncated = if vec.len() > 72 { &vec[..72] } else { &vec };

    bcrypt(cost, salt, truncated, &mut output);

    Ok(HashParts {
        cost,
        salt: base64::encode_config(salt, base64::BCRYPT),
        hash: base64::encode_config(&output[..23], base64::BCRYPT), // remember to remove the last byte
    })
}

/// Takes a full hash and split it into 3 parts:
/// cost, salt and hash
fn split_hash(hash: &str) -> BcryptResult<HashParts> {
    let mut parts = HashParts {
        cost: 0,
        salt: "".to_string(),
        hash: "".to_string(),
    };

    // Should be [prefix, cost, hash]
    let raw_parts: Vec<_> = hash.split('$').filter(|s| !s.is_empty()).collect();

    if raw_parts.len() != 3 {
        return Err(BcryptError::InvalidHash(hash.to_string()));
    }

    if raw_parts[0] != "2y" && raw_parts[0] != "2b" && raw_parts[0] != "2a" && raw_parts[0] != "2x"
    {
        return Err(BcryptError::InvalidPrefix(raw_parts[0].to_string()));
    }

    if let Ok(c) = raw_parts[1].parse::<u32>() {
        parts.cost = c;
    } else {
        return Err(BcryptError::InvalidCost(raw_parts[1].to_string()));
    }

    if raw_parts[2].len() == 53 && raw_parts[2].is_char_boundary(22) {
        parts.salt = raw_parts[2][..22].chars().collect();
        parts.hash = raw_parts[2][22..].chars().collect();
    } else {
        return Err(BcryptError::InvalidHash(hash.to_string()));
    }

    Ok(parts)
}

/// Generates a password hash using the cost given.
/// The salt is generated randomly using the OS randomness
pub fn hash<P: AsRef<[u8]>>(password: P, cost: u32) -> BcryptResult<String> {
    hash_with_result(password, cost).map(|r| r.format())
}

/// Generates a password hash using the cost given.
/// The salt is generated randomly using the OS randomness.
/// The function returns a result structure and allows to format the hash in different versions.
pub fn hash_with_result<P: AsRef<[u8]>>(password: P, cost: u32) -> BcryptResult<HashParts> {
    let salt: Vec<u8> = repeat_with(|| fastrand::u8(..)).take(16).collect();
    
    // let salt = {
    //     let mut s = [0u8; 16];
    //     getrandom(&mut s).map(|_| s)
    // }?;

    _hash_password(password.as_ref(), cost, salt.as_ref())
}

/// Verify that a password is equivalent to the hash provided
pub fn verify<P: AsRef<[u8]>>(password: P, hash: &str) -> BcryptResult<bool> {
    let parts = split_hash(hash)?;
    let salt = base64::decode_config(&parts.salt, base64::BCRYPT)?;
    let generated = _hash_password(password.as_ref(), parts.cost, &salt)?;
    let source_decoded = base64::decode_config(&parts.hash, base64::BCRYPT)?;
    let generated_decoded = base64::decode_config(&generated.hash, base64::BCRYPT)?;
    if source_decoded.len() != generated_decoded.len() {
        return Ok(false);
    }

    let mut diff = 0;
    for (a, b) in source_decoded.into_iter().zip(generated_decoded) {
        diff |= a ^ b;
    }

    Ok(diff == 0)
}

#[derive(Debug)]
/// All the errors we can encounter while hashing/verifying
/// passwords
pub enum BcryptError {
    #[cfg(feature = "std")]
    Io(io::Error),
    CostNotAllowed(u32),
    InvalidPassword,
    InvalidCost(String),
    InvalidPrefix(String),
    InvalidHash(String),
    InvalidBase64(base64::DecodeError),
}

macro_rules! impl_from_error {
    ($f: ty, $e: expr) => {
        impl From<$f> for BcryptError {
            fn from(f: $f) -> BcryptError {
                $e(f)
            }
        }
    };
}

impl_from_error!(base64::DecodeError, BcryptError::InvalidBase64);
#[cfg(feature = "std")]
impl_from_error!(io::Error, BcryptError::Io);

impl fmt::Display for BcryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            #[cfg(feature = "std")]
            BcryptError::Io(ref err) => write!(f, "IO error: {}", err),
            BcryptError::InvalidCost(ref cost) => write!(f, "Invalid Cost: {}", cost),
            BcryptError::CostNotAllowed(ref cost) => write!(
                f,
                "Cost needs to be between {} and {}, got {}",
                MIN_COST,
                MAX_COST,
                cost
            ),
            BcryptError::InvalidPassword => write!(f, "Invalid password: contains NULL byte"),
            BcryptError::InvalidPrefix(ref prefix) => write!(f, "Invalid Prefix: {}", prefix),
            BcryptError::InvalidHash(ref hash) => write!(f, "Invalid hash: {}", hash),
            BcryptError::InvalidBase64(ref err) => write!(f, "Base64 error: {}", err),
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for BcryptError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            BcryptError::Io(ref err) => Some(err),
            BcryptError::InvalidCost(_)
            | BcryptError::CostNotAllowed(_)
            | BcryptError::InvalidPassword
            | BcryptError::InvalidPrefix(_)
            | BcryptError::InvalidHash(_) => None,
            BcryptError::InvalidBase64(ref err) => Some(err),
            BcryptError::Rand(ref err) => Some(err),
        }
    }
}
