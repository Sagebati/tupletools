#![cfg_attr(not(feature = "use_std"), no_std)]

mod fst;
mod snd;
mod trd;

pub use fst::Fst;
pub use snd::Snd;
pub use trd::Trd;

#[cfg(feature = "use_std")]
#[cfg(test)]
mod tests {
    use crate::{Fst, Snd};

    #[test]
    fn fst() {
        let x = vec![(2, 3), (3, 4), (4, 5), (5, 6)];

        let expected = vec![2, 3, 4, 5];
        assert_eq!(x.iter().map(Fst::fst).cloned().collect::<Vec<_>>(), expected);
        assert_eq!(x.into_iter().map(Fst::into_fst).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn snd() {
        let x = vec![(2, 3), (3, 4), (4, 5), (5, 6)];

        let expected = vec![3, 4, 5, 6];
        assert_eq!(x.iter().map(Snd::snd).cloned().collect::<Vec<_>>(), expected);
        assert_eq!(x.into_iter().map(Snd::into_snd).collect::<Vec<_>>(), expected);
    }
}
