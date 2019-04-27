/// Generate an enum with mappings to and from the underlying string
/// value
#[macro_export]
macro_rules! string_enum {
    ( $enum_name:ident, { $( $name:ident = $val:expr ),+ } ) => {
        #[derive(Eq, PartialEq, Clone, Debug)]
        pub enum $enum_name {
            $( $name, )+
        }

        impl std::str::FromStr for $enum_name {
            type Err = ();

            fn from_str(from_val: &str) -> Result<Self, Self::Err> {
                match from_val {
                    $( $val => Ok($enum_name::$name), )+
                    _ => Err(())
                }
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                f.write_str(match self {
                    $( $enum_name::$name => $val, )+
                })
            }
        }
    };
}
