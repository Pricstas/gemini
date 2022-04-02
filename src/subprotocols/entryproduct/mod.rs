//! The entryproduct argument.
//!
//! The entry product argument (also called _grand product argument_ in
//! [plookup](https://eprint.iacr.org/2020/315.pdf),
//! [spartan](https://github.com/microsoft/Spartan))
//! proves that the product of all entries in a vector equal some $t \in \FF$.
//! The [`EntryProduct`] argument reduces claim of the form
//! $$
//! \prod \vec f = t
//! $$
//! for a vector $f \in \FF^N$
//! to sumcheck claim of the form:
//! $$
//!  \langle \vec g \circ \vec y,  \vec f + x^N\rangle = \psi \vec g(\psi) + t - \psi^N
//! $$
//! where:
//!
//! $$
//! \begin{aligned}
//! \vec g &\defeq (\prod_{i \geq 0} f_i,~ \prod_{i \geq 1} f_i,~ \cdots,~ f_{N-2}f_{N-1},~car f_{N-1}) \\\\
//! \vec y &\defeq (1, \psi, \dots, \psi^N).
//! \end{aligned}
//! $$
//!
use ark_ec::PairingEngine;
use ark_ff::PrimeField;
use ark_std::vec::Vec;
use ark_std::Zero;

use crate::kzg::Commitment;
use crate::subprotocols::sumcheck::Prover;

mod elastic_prover;
// XXX. this is temporarily available until accumulated_product is no more needed
// in the preprocessing snark.
pub(crate) mod time_prover;

pub mod streams;

#[cfg(test)]
mod tests;
/// The message sent by the prover during the protocol execution.
///
/// # Note
///
/// Sometimes the verifier already knows the entry product result.
/// For this reason, the product $t$ is never sent or added to the transcript.
/// It is expected that the developer takes care of it in the upper protocol layer.
#[derive(Debug, PartialEq, Eq)]
pub struct ProverMsgs<E: PairingEngine> {
    pub acc_v_commitments: Vec<Commitment<E>>,
    pub claimed_sumchecks: Vec<E::Fr>,
}

impl<E: PairingEngine> ProverMsgs<E> {
    pub(crate) fn size_in_bytes(&self) -> usize {
        let size_of_fe_in_bytes = E::Fr::zero().into_bigint().as_ref().len() * 8;
        let size_of_gp_in_bytes = self.acc_v_commitments[0].size_in_bytes();
        self.acc_v_commitments.len() * size_of_gp_in_bytes
            + self.claimed_sumchecks.len() * size_of_fe_in_bytes
    }
}

/// The entryproduct transcript and subclaims.
pub struct EntryProduct<E: PairingEngine, P: Prover<E::Fr>> {
    /// The messages sent by the prover.
    pub msgs: ProverMsgs<E>,
    /// The challenge sent by the verifier.
    pub chal: E::Fr,
    /// The sumcheck subclaims, parametrized in a time (or elastic) [`Prover`](crate::subprotocols::sumcheck::Prover).
    pub provers: Vec<P>,
}
