use core::ops::Mul;

pub trait Multiply {
    type Output;
    fn mul(self, other: Self) -> Self::Output;
}

impl<T1: Mul<Output = T1>> Multiply for (T1,) {
    type Output = (T1,);
    fn mul(self, other: Self) -> Self::Output {
        (self.0 * other.0,)
    }
}

impl<T1: Mul<Output = T1>, T2: Mul<Output = T2>> Multiply for (T1, T2) {
    type Output = (T1, T2);
    fn mul(self, other: Self) -> Self::Output {
        (self.0 * other.0, self.1 * other.1)
    }
}

impl<T1: Mul<Output = T1>, T2: Mul<Output = T2>, T3: Mul<Output = T3>> Multiply for (T1, T2, T3) {
    type Output = (T1, T2, T3);
    fn mul(self, other: Self) -> Self::Output {
        (self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl<T1: Mul<Output = T1>, T2: Mul<Output = T2>, T3: Mul<Output = T3>, T4: Mul<Output = T4>>
    Multiply for (T1, T2, T3, T4)
{
    type Output = (T1, T2, T3, T4);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
    > Multiply for (T1, T2, T3, T4, T5)
{
    type Output = (T1, T2, T3, T4, T5);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
    > Multiply for (T1, T2, T3, T4, T5, T6)
{
    type Output = (T1, T2, T3, T4, T5, T6);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
    > Multiply for (T1, T2, T3, T4, T5, T6, T7)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
        T8: Mul<Output = T8>,
    > Multiply for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
            self.7 * other.7,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
        T8: Mul<Output = T8>,
        T9: Mul<Output = T9>,
    > Multiply for (T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
            self.7 * other.7,
            self.8 * other.8,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
        T8: Mul<Output = T8>,
        T9: Mul<Output = T9>,
        T10: Mul<Output = T10>,
    > Multiply for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
            self.7 * other.7,
            self.8 * other.8,
            self.9 * other.9,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
        T8: Mul<Output = T8>,
        T9: Mul<Output = T9>,
        T10: Mul<Output = T10>,
        T11: Mul<Output = T11>,
    > Multiply for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
            self.7 * other.7,
            self.8 * other.8,
            self.9 * other.9,
            self.10 * other.10,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
        T8: Mul<Output = T8>,
        T9: Mul<Output = T9>,
        T10: Mul<Output = T10>,
        T11: Mul<Output = T11>,
        T12: Mul<Output = T12>,
    > Multiply for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
            self.7 * other.7,
            self.8 * other.8,
            self.9 * other.9,
            self.10 * other.10,
            self.11 * other.11,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
        T8: Mul<Output = T8>,
        T9: Mul<Output = T9>,
        T10: Mul<Output = T10>,
        T11: Mul<Output = T11>,
        T12: Mul<Output = T12>,
        T13: Mul<Output = T13>,
    > Multiply for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
            self.7 * other.7,
            self.8 * other.8,
            self.9 * other.9,
            self.10 * other.10,
            self.11 * other.11,
            self.12 * other.12,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
        T8: Mul<Output = T8>,
        T9: Mul<Output = T9>,
        T10: Mul<Output = T10>,
        T11: Mul<Output = T11>,
        T12: Mul<Output = T12>,
        T13: Mul<Output = T13>,
        T14: Mul<Output = T14>,
    > Multiply for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
            self.7 * other.7,
            self.8 * other.8,
            self.9 * other.9,
            self.10 * other.10,
            self.11 * other.11,
            self.12 * other.12,
            self.13 * other.13,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
        T8: Mul<Output = T8>,
        T9: Mul<Output = T9>,
        T10: Mul<Output = T10>,
        T11: Mul<Output = T11>,
        T12: Mul<Output = T12>,
        T13: Mul<Output = T13>,
        T14: Mul<Output = T14>,
        T15: Mul<Output = T15>,
    > Multiply
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
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
            self.7 * other.7,
            self.8 * other.8,
            self.9 * other.9,
            self.10 * other.10,
            self.11 * other.11,
            self.12 * other.12,
            self.13 * other.13,
            self.14 * other.14,
        )
    }
}

impl<
        T1: Mul<Output = T1>,
        T2: Mul<Output = T2>,
        T3: Mul<Output = T3>,
        T4: Mul<Output = T4>,
        T5: Mul<Output = T5>,
        T6: Mul<Output = T6>,
        T7: Mul<Output = T7>,
        T8: Mul<Output = T8>,
        T9: Mul<Output = T9>,
        T10: Mul<Output = T10>,
        T11: Mul<Output = T11>,
        T12: Mul<Output = T12>,
        T13: Mul<Output = T13>,
        T14: Mul<Output = T14>,
        T15: Mul<Output = T15>,
        T16: Mul<Output = T16>,
    > Multiply
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
    fn mul(self, other: Self) -> Self::Output {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
            self.5 * other.5,
            self.6 * other.6,
            self.7 * other.7,
            self.8 * other.8,
            self.9 * other.9,
            self.10 * other.10,
            self.11 * other.11,
            self.12 * other.12,
            self.13 * other.13,
            self.14 * other.14,
            self.15 * other.15,
        )
    }
}
