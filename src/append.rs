pub trait Append<T> {
    type Output;
    fn append(self, other: T) -> Self::Output;
}

impl<T1, T> Append<T> for (T1,) {
    type Output = (T1, T);
    fn append(self, other: T) -> Self::Output {
        (self.0, other)
    }
}

impl<T1, T2, T> Append<T> for (T1, T2) {
    type Output = (T1, T2, T);
    fn append(self, other: T) -> Self::Output {
        (self.0, self.1, other)
    }
}

impl<T1, T2, T3, T> Append<T> for (T1, T2, T3) {
    type Output = (T1, T2, T3, T);
    fn append(self, other: T) -> Self::Output {
        (self.0, self.1, self.2, other)
    }
}

impl<T1, T2, T3, T4, T> Append<T> for (T1, T2, T3, T4) {
    type Output = (T1, T2, T3, T4, T);
    fn append(self, other: T) -> Self::Output {
        (self.0, self.1, self.2, self.3, other)
    }
}

impl<T1, T2, T3, T4, T5, T> Append<T> for (T1, T2, T3, T4, T5) {
    type Output = (T1, T2, T3, T4, T5, T);
    fn append(self, other: T) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, other)
    }
}

impl<T1, T2, T3, T4, T5, T6, T> Append<T> for (T1, T2, T3, T4, T5, T6) {
    type Output = (T1, T2, T3, T4, T5, T6, T);
    fn append(self, other: T) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, other)
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T> Append<T> for (T1, T2, T3, T4, T5, T6, T7) {
    type Output = (T1, T2, T3, T4, T5, T6, T7, T);
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, other,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T> Append<T> for (T1, T2, T3, T4, T5, T6, T7, T8) {
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T);
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, other,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T> Append<T> for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T);
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, other,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T> Append<T>
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T);
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, other,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T> Append<T>
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T);
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, other,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T> Append<T>
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T);
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, self.11, other,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T> Append<T>
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
    type Output = (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T);
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, self.11, self.12, other,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T> Append<T>
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
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
        T,
    );
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, self.11, self.12, self.13, other,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T> Append<T>
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
        T,
    );
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, self.11, self.12, self.13, self.14, other,
        )
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T> Append<T>
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
        T,
    );
    fn append(self, other: T) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, self.11, self.12, self.13, self.14, self.15, other,
        )
    }
}
