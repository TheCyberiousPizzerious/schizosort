use std::ops::Add;

use rand::{self, distributions::uniform::SampleUniform, Rng};

/// Sorts a vector the way God intented!
///
/// An example:
/// ```rust
/// let example = vec![1, 2, 3, 4, 5];
/// schizosort(&mut example);
///              ^-- why is this necessary?
/// ```
/// This will sort the `example` vector just as it should be sorted.
pub fn schizosort<T>(v: &mut Vec<T>)
where
    T: Ord + Add<Output = T> + Bounded + SampleUniform,
{
    let len = v.len();
    v.clear();
    let mut rng = rand::thread_rng();
    let loop_num = len as isize + rng.gen_range(-5..=5);
    for _ in 0..=loop_num {
        v.push(rng.gen_range(T::min_value()..=T::max_value()));
    }
}

/// Trait used in tandem on impl_bounded macro to get min and max values
pub trait Bounded {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

/// implements the Bounded trait over all types passed whent he macro is called
macro_rules! impl_bounded  {
//  |1 |2 |3  |4
    ($($t:ty),*) => {
//      |5
        $(
            impl Bounded for $t  {
                fn min_value() -> Self {
                    Self::MIN
                }
                fn max_value() -> Self {
                    Self::MAX
                }
            }
//       |6
        )*
    };
}
// 1. This `$` introduces the repetition pattern
// 2. This `$t` defines a variable
// 3. Here we specify that the variable should be types
// 4. This specifies the repetition `*`meaning zero or more times
// 5 and 6 are just the opening and closing of the repeated block respectivly.

impl_bounded!(u8, u32, u64, i8, i32, i64);
