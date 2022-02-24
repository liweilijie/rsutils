trait Trait {
    fn returns_num() -> i32;
    fn return_self() -> Self;
}

struct SomeType;
struct OtherType;

impl Trait for SomeType {
    fn returns_num() -> i32 {
        1
    }

    // Self == SomeType
    fn return_self() -> Self {
        SomeType
    }
}

impl Trait for OtherType {
    fn returns_num() -> i32 {
        2
    }

    // Self == OtherType
    fn return_self() -> Self {
        OtherType
    }
}

trait AssociatedTrait {
    type AssociatedType;
    fn func(arg: Self::AssociatedType);
}

impl AssociatedTrait for SomeType {
    type AssociatedType = i8;
    fn func(arg: Self::AssociatedType) {
        println!("{}", arg);
    }
}

impl AssociatedTrait for OtherType {
    type AssociatedType = u8;
    fn func(arg: Self::AssociatedType) {
        println!("{}", arg);
    }
}

fn main() {
    SomeType::func(-1_i8);
    OtherType::func(1_u8);
}

use std::convert::TryInto;
use std::fmt::Debug;
use std::ops::Rem;

// generic blanket impls
trait Even {
    fn is_even(self) -> bool;
}

impl<T> Even for T
where
    T: Rem<Output = T> + PartialEq<T> + Sized,
    u8: TryInto<T>,
    <u8 as TryInto<T>>::Error: Debug,
{
    fn is_even(self) -> bool {
        // these unwraps will never panic
        self % 2.try_into().unwrap() == 0.try_into().unwrap()
    }
}

use std::hash::Hash;
use std::hash::Hasher;

struct HPoint {
    x: i32,
    y: i32,
}

impl Hash for HPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x);
        state.write_i32(self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blanket_trait() {
        assert!(2_i8.is_even());
        assert!(4_u8.is_even());
        assert!(6_i16.is_even());
    }
}
