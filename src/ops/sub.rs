use core::ops::Sub;

pub trait Subtraction {
    type Output;
    fn sub(self, other: Self) -> Self::Output;
}

impl<T1: Sub<Output = T1>> Subtraction for (T1,) {
    type Output = (T1,);
    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0,)
    }
}

impl<T1: Sub<Output = T1>, T2: Sub<Output = T2>> Subtraction for (T1, T2) {
    type Output = (T1, T2);
    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0, self.1 - other.1)
    }
}

impl<T1: Sub<Output = T1>, T2: Sub<Output = T2>, T3: Sub<Output = T3>> Subtraction
    for (T1, T2, T3)
{
    type Output = (T1, T2, T3);
    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl<T1: Sub<Output = T1>, T2: Sub<Output = T2>, T3: Sub<Output = T3>, T4: Sub<Output = T4>>
    Subtraction for (T1, T2, T3, T4)
{
    type Output = (T1, T2, T3, T4);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
    > Subtraction for (T1, T2, T3, T4, T5)
{
    type Output = (T1, T2, T3, T4, T5);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
    > Subtraction for (T1, T2, T3, T4, T5, T6)
{
    type Output = (T1, T2, T3, T4, T5, T6);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
    > Subtraction for (T1, T2, T3, T4, T5, T6, T7)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
        T8: Sub<Output = T8>,
    > Subtraction for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
            self.7 - other.7,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
        T8: Sub<Output = T8>,
        T9: Sub<Output = T9>,
    > Subtraction for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
            self.7 - other.7,
            self.8 - other.8,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
        T8: Sub<Output = T8>,
        T9: Sub<Output = T9>,
        T10: Sub<Output = T10>,
    > Subtraction for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
            self.7 - other.7,
            self.8 - other.8,
            self.9 - other.9,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
        T8: Sub<Output = T8>,
        T9: Sub<Output = T9>,
        T10: Sub<Output = T10>,
        T11: Sub<Output = T11>,
    > Subtraction for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
            self.7 - other.7,
            self.8 - other.8,
            self.9 - other.9,
            self.10 - other.10,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
        T8: Sub<Output = T8>,
        T9: Sub<Output = T9>,
        T10: Sub<Output = T10>,
        T11: Sub<Output = T11>,
        T12: Sub<Output = T12>,
    > Subtraction for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
            self.7 - other.7,
            self.8 - other.8,
            self.9 - other.9,
            self.10 - other.10,
            self.11 - other.11,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
        T8: Sub<Output = T8>,
        T9: Sub<Output = T9>,
        T10: Sub<Output = T10>,
        T11: Sub<Output = T11>,
        T12: Sub<Output = T12>,
        T13: Sub<Output = T13>,
    > Subtraction for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
            self.7 - other.7,
            self.8 - other.8,
            self.9 - other.9,
            self.10 - other.10,
            self.11 - other.11,
            self.12 - other.12,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
        T8: Sub<Output = T8>,
        T9: Sub<Output = T9>,
        T10: Sub<Output = T10>,
        T11: Sub<Output = T11>,
        T12: Sub<Output = T12>,
        T13: Sub<Output = T13>,
        T14: Sub<Output = T14>,
    > Subtraction for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
            self.7 - other.7,
            self.8 - other.8,
            self.9 - other.9,
            self.10 - other.10,
            self.11 - other.11,
            self.12 - other.12,
            self.13 - other.13,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
        T8: Sub<Output = T8>,
        T9: Sub<Output = T9>,
        T10: Sub<Output = T10>,
        T11: Sub<Output = T11>,
        T12: Sub<Output = T12>,
        T13: Sub<Output = T13>,
        T14: Sub<Output = T14>,
        T15: Sub<Output = T15>,
    > Subtraction
    for (
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
    )
{
    type Output = (
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
    );
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
            self.7 - other.7,
            self.8 - other.8,
            self.9 - other.9,
            self.10 - other.10,
            self.11 - other.11,
            self.12 - other.12,
            self.13 - other.13,
            self.14 - other.14,
        )
    }
}

impl<
        T1: Sub<Output = T1>,
        T2: Sub<Output = T2>,
        T3: Sub<Output = T3>,
        T4: Sub<Output = T4>,
        T5: Sub<Output = T5>,
        T6: Sub<Output = T6>,
        T7: Sub<Output = T7>,
        T8: Sub<Output = T8>,
        T9: Sub<Output = T9>,
        T10: Sub<Output = T10>,
        T11: Sub<Output = T11>,
        T12: Sub<Output = T12>,
        T13: Sub<Output = T13>,
        T14: Sub<Output = T14>,
        T15: Sub<Output = T15>,
        T16: Sub<Output = T16>,
    > Subtraction
    for (
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
    )
{
    type Output = (
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
        T10,
        T11,
        T12,
        T13,
        T14,
        T15,
        T16,
    );
    fn sub(self, other: Self) -> Self::Output {
        (
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
            self.4 - other.4,
            self.5 - other.5,
            self.6 - other.6,
            self.7 - other.7,
            self.8 - other.8,
            self.9 - other.9,
            self.10 - other.10,
            self.11 - other.11,
            self.12 - other.12,
            self.13 - other.13,
            self.14 - other.14,
            self.15 - other.15,
        )
    }
}
