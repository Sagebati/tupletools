#![cfg_attr(feature = "no_std", no_std)]

mod fst;
mod ops;
mod snd;
mod trd;
mod cast;

pub use fst::Fst;
pub use ops::add::Addition;
pub use ops::div::Division;
pub use ops::mul::Multiply;
pub use ops::sub::Subtraction;
pub use cast::CastOneToOne;
pub use snd::Snd;
pub use trd::Trd;

#[cfg(feature = "use_std")]
#[cfg(test)]
mod tests {
    use crate::{Addition, Division, Fst, Multiply, Snd, Subtraction, CastOneToOne};

    #[test]
    fn fst() {
        let x = vec![(2, 3), (3, 4), (4, 5), (5, 6)];

        let expected = vec![2, 3, 4, 5];
        assert_eq!(
            x.iter().map(Fst::fst).cloned().collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            x.into_iter().map(Fst::into_fst).collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn snd() {
        let x = vec![(2, 3), (3, 4), (4, 5), (5, 6)];

        let expected = vec![3, 4, 5, 6];
        assert_eq!(
            x.iter().map(Snd::snd).cloned().collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            x.into_iter().map(Snd::into_snd).collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn add() {
        let x = vec![(1, 2, 1), (1, 1, 1)];
        let y = (1, 1, 1);
        let z = (1, 1, 1);

        assert_eq!(y.add(z), (2, 2, 2));

        assert_eq!(x.into_iter().fold((0, 0, 0), |a, b| a.add(b)), (2, 3, 2));
    }

    #[test]
    fn mul() {
        let x = vec![(1, 2, 1), (1, 1, 1)];
        let y = (1, 1, 1);
        let z = (1, 1, 1);

        assert_eq!(y.mul(z), (1, 1, 1));

        assert_eq!(x.into_iter().fold((0, 0, 0), |a, b| a.mul(b)), (0, 0, 0));
    }

    #[test]
    fn sub() {
        let x = vec![(1, 2, 1), (1, 1, 1)];
        let y = (1, 1, 1);
        let z = (1, 1, 1);

        assert_eq!(y.sub(z), (0, 0, 0));

        assert_eq!(x.into_iter().fold((0, 0, 0), |a, b| a.sub(b)), (-2, -3, -2));
    }

    #[test]
    fn div() {
        let x = vec![(1, 2, 1), (1, 1, 1)];
        let y = (1, 1, 1);
        let z = (1, 1, 1);

        assert_eq!(y.div(z), (1, 1, 1));

        assert_eq!(
            x.into_iter().fold((1, 1, 1), |a, b| a.div(b)),
            (1, 1 / 2, 1)
        );
    }

    #[test]
    fn cast_one_to_one() {
        let x: (u32, u32, u32, u32, u32, u32) = (3, 4, 5, 6, 7, 8);
        let _z: (u64, u64, u64, u64, u64, u64) = x.cast();
    }
}
