use std::{
    fmt::Error,
    iter,
    ops::{Add, Mul},
};

use crypto_bigint::{rand_core::CryptoRngCore, Random};

/// Polynomial of some degree $n$
///
/// Polynomial has a form: $f(x) = a_0 + a_1 x^1 + \dots{} + a_{n-1} x^{n-1} + a_n x^n$
///
/// Coefficients $a_i$ and indeterminate $x$ are within a ring,
/// and this type is generic for any concrete type that implements ring arithmetic operations.
pub struct Polynomial<T>
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    coefficients: Vec<T>,
}

impl<T> Polynomial<T>
where
    T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
{
    /// Constructs polynomial $f(x)$ from list of coefficients $a_0, \dots, a_n$ in the ring
    ///
    /// ## Order
    ///
    /// $a_i$ should corresponds to polynomial $i^{\text{th}}$ coefficient $f(x) = \dots{} + a_i x^i
    /// + \dots$
    ///
    /// ## Polynomial degree
    ///
    /// Note that it's not guaranteed that constructed polynomial degree equals to
    /// `coefficients.len()-1` as it's allowed to end with zero coefficients. Actual polynomial
    /// degree equals to index of last non-zero coefficient or zero if all the coefficients are
    /// zero.
    pub fn from_coefficients(coefficients: Vec<T>) -> Self {
        Self { coefficients }
    }

    /// Sample a random polynomial of given `degree`
    pub fn sample(degree: u16, rng: &mut impl CryptoRngCore) -> Self
    where
        T: Random,
    {
        Self::from_coefficients(
            iter::repeat_with(|| T::random(rng))
                .take(usize::from(degree + 1))
                .collect(),
        )
    }

    /// Samples random polynomial of degree $n$ with fixed constant term (ie. $a_0 =
    /// \text{constant\\_term}$)
    pub fn sample_with_free_term(degree: u16, free_term: T, rng: &mut impl CryptoRngCore) -> Self
    where
        T: Random,
    {
        let mut coefficients = Self::sample(degree, rng).coefficients;
        coefficients[0] = free_term;

        Self::from_coefficients(coefficients)
    }

    /// Takes scalar $x$ and evaluates $f(x)$
    pub fn evaluate(&self, x: &T) -> Result<T, Error> {
        if self.coefficients.is_empty() {
            return Err(Error); // TODO: proper error, thiserror
        }

        // Iterate through the coefficients, tail to head, and iteratively evaluate the polynomial
        // by multiplying by `x` and adding the coefficient Beginning with the last
        // coefficient, every such iteration increases the power of all previously evaluated parts,
        // until we finish with the free term which isn't multiplied by `x`.
        let mut reversed_coefficients = self.coefficients.iter().rev();
        let last_coefficient = reversed_coefficients.next().unwrap();

        Ok(reversed_coefficients.fold(
            *last_coefficient,
            |partially_evaluated_polynomial, coefficient| {
                partially_evaluated_polynomial * (*x) + (*coefficient)
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crypto_bigint::{Wrapping, U64};

    use super::*;

    #[test]
    fn evaluates() {
        let polynomial = Polynomial::from_coefficients(vec![
            Wrapping(U64::from(1u8)),
            Wrapping(U64::from(2u8)),
            Wrapping(U64::from(3u8)),
        ]);

        assert_eq!(
            polynomial.evaluate(&Wrapping(U64::from(0u8))).unwrap(),
            Wrapping(U64::from(1u8))
        );

        assert_eq!(
            polynomial.evaluate(&Wrapping(U64::from(5u8))).unwrap(),
            Wrapping(U64::from(86u8))
        );
    }
}
