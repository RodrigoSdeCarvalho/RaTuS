use serde::{Deserialize, Serialize};

macro_rules! tuple_types {
    ($(($type:ty, $name:ident,$exact:ident,$any:ident)),+) => {
        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub enum Types {
            $(
                $name($type),
            )*
        }

        impl Types {
            fn satisfy(&self, other: &Types) -> bool {
                match (self, other) {
                    $(
                        (Self::$name(lhs), Self::$name(rhs)) => lhs == rhs,
                    )*
                        _ => false,
                }
            }
        }

        impl std::fmt::Display for Types {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
                match self {
                    $(
                        Self::$name(inner_value) => write!(formatter, "{}", inner_value)?,
                    )*
                };
                Ok(())
            }
        }


        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub enum QueryTypes {
            Any,
            $(
                $exact($type),
                $any,
            )*
        }


        impl QueryTypes {
            fn satisfy(&self, other: &Types) -> bool {
                match(self, other) {
                    (Self::Any, _) => true,
                    $(
                        (Self::$any, Types::$name(_)) => true,
                        (Self::$any, _) => false,
                    )*
                    $(
                        (Self::$exact(lhs), Types::$name(rhs)) => lhs == rhs,
                        (Self::$exact(_), _) => false,
                    )*
                }
            }
        }
    };
}

tuple_types![
    (bool, Boolean, ExactBoolean, AnyBoolean),
    (i64, Integer, ExactInteger, AnyInteger),
    (f64, Float, ExactFloat, AnyFloat),
    (String, String, ExactString, AnyString)
];

impl PartialEq<Types> for QueryTypes {
    fn eq(&self, other: &Types) -> bool {
        self.satisfy(other)
    }
}

impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        self.satisfy(other)
    }
}

#[test]
fn test_compare() {
    let b1 = Types::Boolean(true);
    let b1_copy = Types::Boolean(true);
    let b2 = Types::Boolean(false);

    assert_eq!(b1, b1_copy);
    assert_ne!(b1, b2);

    let i1 = Types::Integer(1);
    let i1_copy = Types::Integer(1);
    let i2 = Types::Integer(2);

    assert_eq!(i1, i1_copy);
    assert_ne!(i1, i2);

    let f1 = Types::Float(1.0);
    let f1_copy = Types::Float(1.0);
    let f2 = Types::Float(2.0);

    assert_eq!(f1, f1_copy);
    assert_ne!(f1, f2);
    assert_ne!(f1, i1);

    let s1 = Types::String(String::from("S1"));
    let s1_copy = Types::String(String::from("S1"));
    let s2 = Types::String(String::from("S2"));

    assert_eq!(s1, s1_copy);
    assert_ne!(s1, s2);
    assert_ne!(s1, f1);
}

#[test]
fn test_query_compare() {
    let boolean = Types::Boolean(true);
    let integer = Types::Integer(1);
    let float = Types::Float(1.0);
    let string = Types::String(String::from("S1"));

    assert_eq!(QueryTypes::Any, boolean);
    assert_eq!(QueryTypes::Any, integer);
    assert_eq!(QueryTypes::Any, float);
    assert_eq!(QueryTypes::Any, string);
    assert_eq!(QueryTypes::AnyBoolean, boolean);
    assert_eq!(QueryTypes::AnyInteger, integer);
    assert_eq!(QueryTypes::AnyFloat, float);
    assert_eq!(QueryTypes::AnyString, string);

    assert_eq!(QueryTypes::ExactString(String::from("S1")), string);
    assert_eq!(QueryTypes::ExactInteger(1), integer);
    assert_eq!(QueryTypes::ExactFloat(1.0), float);
    assert_eq!(QueryTypes::ExactBoolean(true), boolean);
}
