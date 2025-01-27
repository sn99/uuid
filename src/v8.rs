use crate::{Builder, Uuid, Variant, Version};

impl Uuid {
    /// Creates a custom UUID comprised almost entirely of user-supplied bytes
    ///
    /// This will inject the UUID Version at 4 bits starting at the 48th bit
    /// and the Variant into 2 bits 64th bit.
    /// So if there are bits are supplied in the input buffer, they will not be
    /// visible in the result
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::{Uuid, Version};
    /// let buf: [u8; 16] = *b"abcdefghijklmnop";
    /// let uuid = Uuid::new_v8(buf);
    ///
    /// assert_eq!(Some(Version::Custom), uuid.get_version());
    /// ```
    pub fn new_v8(buf: [u8; 16]) -> Uuid {
        Builder::from_bytes(buf)
            .with_variant(Variant::RFC4122)
            .with_version(Version::Custom)
            .into_uuid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_new() {
        let buf: [u8; 16] = [
            0xf, 0xe, 0xd, 0xc, 0xb, 0xa, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3,
            0x2, 0x1, 0x0,
        ];
        let uuid = Uuid::new_v8(buf);
        assert_eq!(uuid.get_version(), Some(Version::Custom));
        assert_eq!(uuid.get_variant(), Variant::RFC4122);
        assert_eq!(uuid.get_version_num(), 8);
        assert_eq!(
            uuid.hyphenated().to_string(),
            "0f0e0d0c-0b0a-8908-8706-050403020100"
        );
    }
}
