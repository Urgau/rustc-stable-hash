//! Weaker version of `-Coverflow-checks`.

/// Addition, but only overflow checked when `cfg(debug_assertions)` is set
/// instead of respecting `-Coverflow-checks`.
///
/// This exists for performance reasons, as we ship rustc with overflow checks.
/// While overflow checks are perf neutral in almost all of the compiler, there
/// are a few particularly hot areas where we don't want overflow checks in our
/// dist builds. Overflow is still a bug there, so we want overflow check for
/// builds with debug assertions.
///
/// That's a long way to say that this should be used in areas where overflow
/// is a bug but overflow checking is too slow.
#[inline]
#[allow(clippy::arithmetic_side_effects)]
pub(crate) fn debug_strict_add(lhs: usize, rhs: usize) -> usize {
    if cfg!(debug_assertions) {
        lhs + rhs
    } else {
        usize::wrapping_add(lhs, rhs)
    }
}

/// Subtraction, but only overflow checked when `cfg(debug_assertions)` is set
/// instead of respecting `-Coverflow-checks`.
///
/// See [`debug_strict_add`] for more details.
#[inline]
#[allow(clippy::arithmetic_side_effects)]
pub(crate) fn debug_strict_sub(lhs: usize, rhs: usize) -> usize {
    if cfg!(debug_assertions) {
        lhs - rhs
    } else {
        usize::wrapping_sub(lhs, rhs)
    }
}
