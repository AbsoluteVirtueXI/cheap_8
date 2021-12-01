use std::convert::From;
use std::fmt;

macro_rules! define_nibble {
    ($name:ident, $BITS:literal, $internal_type:ident) => {
        #[derive(Debug, PartialEq, PartialOrd, Default, Copy, Clone)]
        pub struct $name(pub $internal_type);
        impl $name {
            pub const BITS: usize = $BITS;
            const MIN_VALUE: $internal_type = std::$internal_type::MIN;
            const MAX_VALUE: $internal_type =
                std::$internal_type::MAX >> ($internal_type::BITS - $BITS);
            pub const MIN: Self = $name($name::MIN_VALUE);
            pub const MAX: Self = $name($name::MAX_VALUE);

            pub fn new(value: $internal_type) -> $name {
                match value {
                    $name::MIN_VALUE..=$name::MAX_VALUE => $name(value),
                    _ => panic!("value is too big"),
                }
            }

            pub fn value(&self) -> $internal_type {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$internal_type> for $name {
            fn from(value: $internal_type) -> Self {
                $name::new(value)
            }
        }
    };
}

define_nibble!(Nibble, 4, u8);
define_nibble!(DoubleNibble, 8, u8);
define_nibble!(TripleNibble, 12, u16);

pub type Register = Nibble;
pub type ImmediateValue = DoubleNibble;
pub type Address = TripleNibble;
