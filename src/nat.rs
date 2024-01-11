pub struct Zero;
pub struct Succ<T: Nat>(pub T);

pub trait Nat {
    fn to_int(&self) -> u32;
}

impl Nat for Zero {
    fn to_int(&self) -> u32 { 0 }
}
impl<T: Nat> Nat for Succ<T> {
    fn to_int(&self) -> u32 { self.0.to_int() + 1 }
}

pub trait Add<Other: Nat> {
    type Result: Nat;
    fn add(self, other: Other) -> Self::Result;
}

impl<Other: Nat> Add<Other> for Zero {
    type Result = Other;
    fn add(self, other: Other) -> Self::Result { other }
}

impl<Nested: Nat + Add<Other>, Other: Nat> Add<Other> for Succ<Nested> {
    type Result = Succ<<Nested as Add<Other>>::Result>;
    fn add(self, other: Other) -> Self::Result {
        Succ(self.0.add(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_plus_zero() {
        assert_eq!(Zero.add(Zero).to_int(), 0);
    }

    #[test]
    fn zero_plus_one() {
        assert_eq!(Zero.add(Succ(Zero)).to_int(), 1);
    }

    #[test]
    fn one_plus_zero() {
        assert_eq!(Succ(Zero).add(Zero).to_int(), 1);
    }

    #[test]
    fn one_plus_one() {
        assert_eq!(Succ(Zero).add(Succ(Zero)).to_int(), 2);
    }

    #[test]
    fn three_plus_four() {
        let three = Succ(Succ(Succ(Zero)));
        let four = Succ(Succ(Succ(Succ(Zero))));
        assert_eq!(three.add(four).to_int(), 7);
    }
}

