use core::ops::Mul;

pub trait Multiply<T> {
    fn mul(self, other: Self) -> Self;
}

impl<T: Mul<Output=T>> Multiply<T> for (T, ) {
    fn mul(self, other: Self) -> Self {
        (self.0 * other.0, )
    }
}

impl<T: Mul<Output=T>> Multiply<T> for (T, T) {
    fn mul(self, other: Self) -> Self {
        (self.0 * other.0, self.1 * other.1)
    }
}

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T) {
    fn mul(self, other: Self) -> Self {
        (self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T) {
    fn mul(self, other: Self) -> Self {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
        )
    }
}

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
        (
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
            self.4 * other.4,
        )
    }
}

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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

impl<T: Mul<Output=T>> Multiply<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn mul(self, other: Self) -> Self {
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
