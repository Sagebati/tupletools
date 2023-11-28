#![cfg_attr(feature = "no_std", no_std)]

mod append;
mod cast;
mod fst;
mod head_tail;
mod ops;
mod pop_back;
mod pop_front;
mod prepend;
mod snd;
mod trd;

pub use append::Append;
pub use cast::CastOneToOne;
pub use fst::fst;
pub use fst::Fst;
pub use ops::add::Addition;
pub use ops::div::Division;
pub use ops::mul::Multiply;
pub use ops::sub::Subtraction;
pub use pop_back::PopBack;
pub use pop_front::PopFront;
pub use prepend::Prepend;
pub use snd::snd;
pub use snd::Snd;
pub use trd::trd;
pub use trd::Trd;

#[cfg(not(feature = "no_std"))]
#[cfg(test)]
mod tests {
    use crate::*;
    use crate::head_tail::HeadTail;

    #[test]
    fn fst_test() {
        let x = vec![(2, 3), (3, 4), (4, 5), (5, 6)];

        let expected = vec![2, 3, 4, 5];
        assert_eq!(
            x.iter().map(Fst::fst).cloned().collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            x.clone().into_iter().map(Fst::fst).collect::<Vec<_>>(),
            expected
        );
        assert_eq!(x.into_iter().map(fst).collect::<Vec<_>>(), expected)
    }

    #[test]
    fn snd_test() {
        let x = vec![(2, 3), (3, 4), (4, 5), (5, 6)];

        let expected = vec![3, 4, 5, 6];
        assert_eq!(
            x.iter().map(Snd::snd).cloned().collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            x.clone().into_iter().map(Snd::snd).collect::<Vec<_>>(),
            expected
        );
        assert_eq!(x.iter().map(snd).cloned().collect::<Vec<_>>(), expected);
        assert_eq!(x.into_iter().map(snd).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn trd_test() {
        let x = vec![(2, 3, 4), (3, 4, 5), (4, 5, 6), (5, 6, 7)];

        let expected = vec![4, 5, 6, 7];
        assert_eq!(
            x.iter().map(Trd::trd).cloned().collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            x.clone().into_iter().map(Trd::trd).collect::<Vec<_>>(),
            expected
        );

        assert_eq!(x.iter().map(trd).cloned().collect::<Vec<_>>(), expected);
        assert_eq!(x.into_iter().map(trd).collect::<Vec<_>>(), expected);
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

    #[test]
    fn append() {
        let x = (1, 3, 4, 5);
        let y = x.append(5);

        assert_eq!((1, 3, 4, 5, 5), y);
    }

    #[test]
    fn prepend() {
        let x = (1, 3, 4, 5);

        assert_eq!((5, 1, 3, 4, 5), x.prepend(5))
    }

    #[test]
    fn pop_back() {
        let x = (1, 3, 4, 5);

        assert_eq!((1, 3, 4), x.pop_back())
    }

    #[test]
    fn pop_front() {
        let x = (1, 3, 4, 5);

        assert_eq!((3, 4, 5), x.pop_front())
    }

    #[test]
    fn head_tail() {
        let x = (1,3,4,5);

        assert_eq!((1,(3,4,5)),x.head_tail())
    }
}
