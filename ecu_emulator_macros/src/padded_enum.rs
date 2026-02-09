/// Generates a padded, `zerocopy`-safe version of an enum.
///
/// # Arguments
/// * `(size = N)` - A compile-time assertion that the generated enum is exactly N bytes.
/// * `#[pad(N)]` - An attribute placed on each variant to add N bytes of explicit zero-padding.
///
/// # Example
/// ```rust
/// use zerocopy::{IntoBytes, FromBytes, Immutable, KnownLayout};
/// use ecu_emulator_macros::padded_enum;
///
/// padded_enum! {
///     (size = 5) // Mandatory size check. All enum variants must take 5 bytes.
///     #[derive(Debug, Clone, Copy, PartialEq)]
///     pub enum Command {
///         // Variant 1: Tag(1) + u32(4) + Pad(0) = 5 bytes
///         #[pad(0)]
///         Move{val: u32},
///
///         // Variant 2: Tag(1) + Pad(4) = 5 bytes
///         #[pad(4)]
///         Stop
///     }
/// }
///
/// // Usage
/// let cmd = Command::Move { val: 42 };
/// let mut buffer = [0u8; 5];
/// let bytes = cmd.to_bytes(&mut buffer);
/// let cmd = Command::from_bytes(bytes).unwrap();
/// ```
#[macro_export]
macro_rules! padded_enum {
    (
        (size = $size:expr)
        $(#[$meta:meta])*
        $vis:vis enum $Original:ident {
            $(
                #[pad($pad:expr)]
                $Variant:ident $( { $( $field_name:ident : $field_type:ty ),* $(,)?} )? $( = $disc:expr )?
            ),* $(,)?
        }
    ) => {
        // ---------------------------------------------------------
        // 1. The Original Ergonomic Enum
        // ---------------------------------------------------------
        $(#[$meta])*
        $vis enum $Original {
            $(
                $Variant $( { $($field_name: $field_type),* } )?  $( = $disc )?,
            )*
        }

        // We use the `paste` crate to concatenate names (Original + Padded)
        $crate::paste! {

            // ---------------------------------------------------------
            // 2. Internal Packed Structs
            // ---------------------------------------------------------
            // These wrap the data to force alignment to 1, preventing
            // the compiler from inserting uninitialized padding bytes
            // between the Enum Tag and the Variant Data.
            $(
                #[repr(C, packed)]
                #[derive(
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::FromBytes,
                    zerocopy_derive::Immutable,
                    zerocopy_derive::KnownLayout
                )]
                #[allow(non_camel_case_types)]
                $vis struct [<$Original Padded _ $Variant _Body>] {
                    $($( pub $field_name: $field_type, )*)?
                    pub _pad: [u8; $pad],
                }
            )*

            // ---------------------------------------------------------
            // 3. The Padded (Wire-Format) Enum
            // ---------------------------------------------------------
            #[repr(u8)]
            #[derive(
                zerocopy_derive::IntoBytes,
                zerocopy_derive::TryFromBytes,
                zerocopy_derive::Immutable,
                zerocopy_derive::KnownLayout
            )]
            $vis enum [<$Original Padded>] {
                $(
                    $Variant( [<$Original Padded _ $Variant _Body>] ) $( = $disc )?,
                )*
            }

            // ---------------------------------------------------------
            // 4. Direct conversions between Original and bytes
            // ---------------------------------------------------------
            impl $Original {
                /// Serializes the enum to a vector of bytes, omitting the padding.
                #[allow(unused)]
                pub fn to_bytes<'a>(&self, buf: &'a mut [u8; $size]) -> &'a [u8] {
                    match self {
                        $(
                            $Original::$Variant $( { $($field_name),* } )? => {
                                // Construct the padded body struct
                                let padded_body = [<$Original Padded _ $Variant _Body>] {
                                    $($( $field_name: $field_name.clone(), )*)?
                                    _pad: [0u8; $pad],
                                };
                                let padded_enum = [<$Original Padded>]::$Variant(padded_body);
                                let bytes = ::zerocopy::IntoBytes::as_bytes(&padded_enum);
                                buf.copy_from_slice(bytes);
                                &buf[0..(bytes.len() - $pad)]
                            }
                        )*
                    }
                }

                /// Deserializes from a byte slice, padding with zeros if necessary.
                #[allow(unused)]
                pub fn from_bytes(bytes: &[u8]) -> Result<Self, ::std::string::String> {
                    let mut buf = [0u8; $size];
                    let len = ::std::cmp::min(bytes.len(), $size);
                    buf[0..len].copy_from_slice(&bytes[0..len]);

                    let padded = <[<$Original Padded>] as ::zerocopy::TryFromBytes>::try_read_from_bytes(&buf)
                        .map_err(|e| ::std::format!("{:?}", e))?;
                    Ok(padded.into())
                }
            }

            // ---------------------------------------------------------
            // 5. Conversion: Original -> Padded
            // ---------------------------------------------------------
            impl From<$Original> for [<$Original Padded>] {
                fn from(orig: $Original) -> Self {
                    match orig {
                        $(
                            $Original::$Variant $( { $($field_name),* } )? => {
                                [<$Original Padded>]::$Variant(
                                    [<$Original Padded _ $Variant _Body>] {
                                        $($( $field_name, )*)?
                                        _pad: [0u8; $pad]
                                    }
                                )
                            }
                        )*
                    }
                }
            }

            // ---------------------------------------------------------
            // 6. Conversion: Padded -> Original
            // ---------------------------------------------------------
            impl From<[<$Original Padded>]> for $Original {
                fn from(padded: [<$Original Padded>]) -> Self {
                    match padded {
                        $(
                            #[allow(unused_variables)]
                            [<$Original Padded>]::$Variant(body) => {
                                $Original::$Variant $( { $( $field_name: body.$field_name ),* } )?
                            }
                        )*
                    }
                }
            }

            // ---------------------------------------------------------
            // 7. Size Check (Type Mismatch Trick)
            // ---------------------------------------------------------
            // If the size doesn't match, this triggers a compiler error:
            // "Expected array of size X, found array of size Y"
            const _: [(); $size] = [(); std::mem::size_of::<[<$Original Padded>]>()];
        }
    };
}

// ---------------------------------------------------------
// Unit Tests
// ---------------------------------------------------------
#[cfg(test)]
mod tests {
    use zerocopy::{IntoBytes, TryFromBytes};

    // Define a test enum using the macro
    padded_enum! {
        (size = 5) // Tag(1) + MaxPayload(4)

        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u8)]
        pub enum MyProto {
            // 1 + 4 + 0 = 5
            #[pad(0)]
            Move{dist: u32,},

            // 1 + 1 + 3 = 5
            #[pad(3)]
            Jump{height: u8},

            // 1 + 0 + 4 = 5
            #[pad(4)]
            Stop
        }
    }

    #[test]
    fn test_layout_size() {
        assert_eq!(std::mem::size_of::<MyProtoPadded>(), 5);
    }

    #[test]
    fn test_move_variant() {
        let original = MyProto::Move { dist: 0xAABBCCDD };
        let padded: MyProtoPadded = original.into();

        // Check bytes: Tag (0) + u32 (DD CC BB AA) in little endian
        let bytes = padded.as_bytes();
        // Note: Tag values depend on declaration order. Move=0
        assert_eq!(bytes[0], 0);
        assert_eq!(&bytes[1..5], &[0xDD, 0xCC, 0xBB, 0xAA]);

        let padded_back: MyProtoPadded = MyProtoPadded::try_read_from_bytes(&bytes).unwrap();
        // Round trip
        let back: MyProto = padded_back.into();
        assert_eq!(original, back);
    }

    #[test]
    fn test_jump_variant_padding() {
        let original = MyProto::Jump { height: 0xFF };
        let padded: MyProtoPadded = original.into();

        let bytes = padded.as_bytes();
        // Tag=1, Data=FF, Pad=00 00 00
        assert_eq!(bytes[0], 1);
        assert_eq!(bytes[1], 0xFF);
        assert_eq!(bytes[2], 0x00); // Pad must be zero
        assert_eq!(bytes[3], 0x00);
        assert_eq!(bytes[4], 0x00);

        // Round trip
        let back: MyProto = padded.into();
        assert_eq!(original, back);
    }

    #[test]
    fn test_stop_variant_padding() {
        let original = MyProto::Stop;
        let padded: MyProtoPadded = original.into();

        let bytes = padded.as_bytes();
        // Tag=2, Pad=00 00 00 00
        assert_eq!(bytes[0], 2);
        assert_eq!(bytes[1..5], [0, 0, 0, 0]);

        // Round trip
        let back: MyProto = padded.into();
        assert_eq!(original, back);
    }

    // Test enum with discriminants set via constants
    const CMD_PING: u8 = 10;
    const CMD_PONG: u8 = 20;
    const CMD_DATA: u8 = 30;

    padded_enum! {
        (size = 5)

        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u8)]
        pub enum ConstDiscriminant {
            #[pad(4)]
            Ping = CMD_PING,

            #[pad(4)]
            Pong = CMD_PONG,

            #[pad(0)]
            Data { value: u32 } = CMD_DATA,
        }
    }

    #[test]
    fn test_constant_discriminants() {
        // Verify discriminants are set correctly via constants
        let ping = ConstDiscriminant::Ping;
        let padded: ConstDiscriminantPadded = ping.into();
        let bytes = padded.as_bytes();
        assert_eq!(bytes[0], CMD_PING);

        let pong = ConstDiscriminant::Pong;
        let padded: ConstDiscriminantPadded = pong.into();
        let bytes = padded.as_bytes();
        assert_eq!(bytes[0], CMD_PONG);

        let data = ConstDiscriminant::Data { value: 0x12345678 };
        let padded: ConstDiscriminantPadded = data.into();
        let bytes = padded.as_bytes();
        assert_eq!(bytes[0], CMD_DATA);
        assert_eq!(&bytes[1..5], &[0x78, 0x56, 0x34, 0x12]); // little endian
    }

    #[test]
    fn test_constant_discriminants_round_trip() {
        let original = ConstDiscriminant::Data { value: 0xDEADBEEF };
        let padded: ConstDiscriminantPadded = original.into();
        let bytes = padded.as_bytes();

        let padded_back = ConstDiscriminantPadded::try_read_from_bytes(&bytes).unwrap();
        let back: ConstDiscriminant = padded_back.into();
        assert_eq!(original, back);
    }

    #[test]
    fn test_to_from_bytes_for_original() {
        let mut buffer = [0u8; 5];
        let original = MyProto::Move { dist: 0xA1B2C3D4 };
        let bytes = original.to_bytes(&mut buffer);
        // Tag(0) + dist(4) = 5. padding is 0.
        assert_eq!(bytes.len(), 5);
        assert_eq!(bytes[0], 0);
        assert_eq!(&bytes[1..5], &[0xD4, 0xC3, 0xB2, 0xA1]);

        let recovered = MyProto::from_bytes(&bytes).unwrap();
        assert_eq!(original, recovered);

        // Test padding logic
        let jump = MyProto::Jump { height: 0x77 };
        let bytes_jump = jump.to_bytes(&mut buffer);
        // Tag(1) + height(1) + pad(3) = 5.
        // Stripped bytes = 5 - 3 = 2.
        assert_eq!(bytes_jump.len(), 2);
        assert_eq!(bytes_jump, vec![1, 0x77]);

        let recovered_jump = MyProto::from_bytes(&bytes_jump).unwrap();
        assert_eq!(jump, recovered_jump);

        // Test undersized input
        let incomplete = vec![1, 0x77]; // implicit padding
        let recovered = MyProto::from_bytes(&incomplete).unwrap();
        assert_eq!(jump, recovered);
        
        // Test empty input (should be padded with 0 -> Tag=0 -> Move{val:0})
        let empty: Vec<u8> = vec![];
        let recovered_empty = MyProto::from_bytes(&empty).unwrap();
        match recovered_empty {
             MyProto::Move { dist } => assert_eq!(dist, 0),
             _ => panic!("Expected Move(0)"),
        }
    }
}

