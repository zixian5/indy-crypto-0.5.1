use bn::{BigNumber, BigNumberContext};
use errors::prelude::*;


/// Generate a pedersen commitment to a given number
///
/// # Arguments
/// * `gen_1` - first generator
/// * `m` - exponent of the first generator
/// * `gen_2` - second generator
/// * `r` - exponent of the second generator
/// * `modulus` - all computations are done this modulo
/// * `ctx` - big number context
///
/// # Result
/// Return the pedersen commitment, i.e `(gen_1^m)*(gen_2^r)`
pub fn get_pedersen_commitment(gen_1: &BigNumber, m: &BigNumber,
                               gen_2: &BigNumber, r: &BigNumber,
                               modulus: &BigNumber, ctx: &mut BigNumberContext) -> IndyCryptoResult<BigNumber> {
    let commitment = gen_1.mod_exp(m, modulus, Some(ctx))?
        .mod_mul(&gen_2.mod_exp(r, modulus, Some(ctx))?,
                 modulus, Some(ctx))?;
    Ok(commitment)
}


/// Generate a pedersen commitment over `n` values
///
/// # Arguments
/// * `to_commit` - a list of 2-tuples where the first element of the tuple is a generator and
/// the second is the value being committed to, like [(g_1, m_1), (g_2, m_2), (g_3, m_3), ... (g_i, m_i)]
/// * `modulus` - all computations are done this modulo
/// * `ctx` - big number context
///
/// # Result
/// Return the pedersen commitment, i.e `(g_1^m_1)*(g_2^m_2)*...(g_i^m_i)*(gen_2^r)`
pub fn get_generalized_pedersen_commitment(to_commit: Vec<(&BigNumber, &BigNumber)>,
                                           modulus: &BigNumber, ctx: &mut BigNumberContext) -> IndyCryptoResult<BigNumber> {
    let accumulated = get_exponentiated_generators(to_commit, modulus, ctx)?;

    Ok(accumulated)
}


/// Exponentiate the given generators to corresponding exponents
///
/// # Arguments
/// * `to_exponentiate` - a list of 2-tuples where the first element of the tuple is a generator and
/// the second is the exponent, like [(g_1, e_1), (g_2, e_2), (g_3, e_3), ... (g_i, e_i)]
/// * `modulus` - all computations are done this modulo
/// * `ctx` - big number context
///
/// # Result
/// Return the exponentiation, i.e `(g_1^e_1)*(g_2^e_2)*...(g_i^e_i)`
pub fn get_exponentiated_generators(to_exponentiate: Vec<(&BigNumber, &BigNumber)>,
                                    modulus: &BigNumber, ctx: &mut BigNumberContext) -> IndyCryptoResult<BigNumber> {
    let accumulated = to_exponentiate.iter()
                                     .fold(BigNumber::from_u32(1),
                                           |acc, &(g, m)| acc?.mod_mul(&g.mod_exp(m, modulus, Some(ctx))?, modulus, Some(ctx)))?;
    Ok(accumulated)
}
