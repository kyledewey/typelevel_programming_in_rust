use crate::nat::{Zero, Succ, Nat};

#[derive(PartialEq, Eq, Debug)]
pub struct HNil;

#[derive(PartialEq, Eq, Debug)]
pub struct HCons<Head, Tail: HList>(Head, Tail);

pub trait HList {}

impl HList for HNil {}
impl<Head, Tail: HList> HList for HCons<Head, Tail> {}

pub trait Append<Other: HList> {
    type Result: HList;
    fn append(self, other: Other) -> Self::Result;
}

impl<Other: HList> Append<Other> for HNil {
    type Result = Other;
    fn append(self, other: Other) -> Self::Result { other }
}

impl<Head, Tail: HList + Append<Other>, Other: HList> Append<Other> for HCons<Head, Tail> {
    type Result = HCons<Head, <Tail as Append<Other>>::Result>;
    fn append(self, other: Other) -> Self::Result {
        HCons(self.0, self.1.append(other))
    }
}

pub trait Len {
    type Result: Nat;
    fn len(&self) -> Self::Result;
}

impl Len for HNil {
    type Result = Zero;
    fn len(&self) -> Self::Result { Zero }
}

impl<Head, Tail: HList + Len>  Len for HCons<Head, Tail> {
    type Result = Succ<<Tail as Len>::Result>;
    fn len(&self) -> Self::Result {
        Succ(self.1.len())
    }
}

pub trait Nth<N: Nat> {
    type Result;
    fn nth(&self, n: &N) -> &Self::Result;
}

impl<Head, Tail: HList> Nth<Zero> for HCons<Head, Tail> {
    type Result = Head;
    fn nth(&self, _n: &Zero) -> &Self::Result {
        &self.0
    }
}

impl<Nested: Nat, Head, Tail: HList + Nth<Nested>>  Nth<Succ<Nested>> for HCons<Head, Tail> {
    type Result = <Tail as Nth<Nested>>::Result;
    fn nth(&self, n: &Succ<Nested>) -> &Self::Result {
        self.1.nth(&n.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_nil_nil() {
        assert_eq!(HNil.append(HNil),
                   HNil);
    }

    #[test]
    fn append_nil_cons() {
        assert_eq!(HNil.append(HCons("foo", HNil)),
                   HCons("foo", HNil));
    }

    #[test]
    fn append_cons_cons() {
        assert_eq!(HCons("foo", HNil).append(HCons("bar", HNil)),
                   HCons("foo", HCons("bar", HNil)));
    }

    #[test]
    fn append_multi_types() {
        let list1 = HCons("foo", HCons(2, HCons(true, HNil)));
        let list2 = HCons(3, HCons("bar", HCons((), HNil)));
        let result = HCons("foo", HCons(2, HCons(true, HCons(3, HCons("bar", HCons((), HNil))))));
        assert_eq!(list1.append(list2), result);
    }

    #[test]
    fn len_nil() {
        assert_eq!(HNil.len().to_int(), 0);
    }

    #[test]
    fn len_cons_nil() {
        assert_eq!(HCons("foo", HNil).len().to_int(), 1);
    }

    #[test]
    fn len_three() {
        assert_eq!(HCons("foo", HCons(1, HCons(true, HNil))).len().to_int(), 3);
    }

    #[test]
    fn nth_tests() {
        let list = HCons("foo", HCons(2, HCons(true, HCons("bar", HNil))));
        assert_eq!(list.nth(&Zero), &"foo");
        assert_eq!(list.nth(&Succ(Zero)), &2);
        assert_eq!(list.nth(&Succ(Succ(Zero))), &true);
        assert_eq!(list.nth(&Succ(Succ(Succ(Zero)))), &"bar");
    }
}
