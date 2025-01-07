#[cfg(not(target_arch = "wasm32"))]

use std::fmt::Debug;
use starknet_ff::FieldElement as FieldElement252;

use serde::{Deserialize, Serialize};
use crate::core::air::accumulation::AccumulationOps;
use crate::core::channel::{Blake2sChannel, Poseidon252Channel};
use crate::core::circle::{CirclePoint, Coset};
use super::{Backend, BackendForChannel, Col, ColumnOps, FieldOps};
use crate::core::fields::m31::BaseField;
use crate::core::fields::qm31::SecureField;
use crate::core::fields::secure_column::SecureColumnByCoords;
use crate::core::fri::FriOps;
use crate::core::lookups::gkr_prover::{GkrMultivariatePolyOracle, GkrOps, Layer};
use crate::core::lookups::mle::{Mle, MleOps};
use crate::core::lookups::utils::UnivariatePoly;
use crate::core::pcs::quotients::{ColumnSampleBatch, QuotientOps};
use crate::core::poly::BitReversedOrder;
use crate::core::poly::circle::{CanonicCoset, CircleDomain, CircleEvaluation, CirclePoly, PolyOps, SecureEvaluation};
use crate::core::poly::line::LineEvaluation;
use crate::core::poly::twiddles::TwiddleTree;
use crate::core::proof_of_work::GrindOps;
use crate::core::utils::bit_reverse_index;
use crate::core::vcs::blake2_hash::Blake2sHash;
use crate::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use crate::core::vcs::ops::MerkleOps;
use crate::core::vcs::poseidon252_merkle::{Poseidon252MerkleChannel, Poseidon252MerkleHasher};
use metal::*;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct MetalBackend;

impl PolyOps for MetalBackend {
    type Twiddles = ();

    fn new_canonical_ordered(_coset: CanonicCoset, _values: Col<Self, BaseField>) -> CircleEvaluation<Self, BaseField, BitReversedOrder> {
        todo!()
    }

    fn interpolate(_eval: CircleEvaluation<Self, BaseField, BitReversedOrder>, _itwiddles: &TwiddleTree<Self>) -> CirclePoly<Self> {
        todo!()
    }

    fn eval_at_point(_poly: &CirclePoly<Self>, _point: CirclePoint<SecureField>) -> SecureField {
        todo!()
    }

    fn extend(_poly: &CirclePoly<Self>, _log_size: u32) -> CirclePoly<Self> {
        todo!()
    }

    fn evaluate(_poly: &CirclePoly<Self>, _domain: CircleDomain, _twiddles: &TwiddleTree<Self>) -> CircleEvaluation<Self, BaseField, BitReversedOrder> {
        todo!()
    }

    fn precompute_twiddles(_coset: Coset) -> TwiddleTree<Self> {
        todo!()
    }
}

impl QuotientOps for MetalBackend {
    fn accumulate_quotients(_domain: CircleDomain, _columns: &[&CircleEvaluation<Self, BaseField, BitReversedOrder>], _random_coeff: SecureField, _sample_batches: &[ColumnSampleBatch], _log_blowup_factor: u32) -> SecureEvaluation<Self, BitReversedOrder> {
        todo!()
    }
}

impl FriOps for MetalBackend {
    fn fold_line(_eval: &LineEvaluation<Self>, _alpha: SecureField, _twiddles: &TwiddleTree<Self>) -> LineEvaluation<Self> {
        todo!()
    }

    fn fold_circle_into_line(_dst: &mut LineEvaluation<Self>, _src: &SecureEvaluation<Self, BitReversedOrder>, _alpha: SecureField, _twiddles: &TwiddleTree<Self>) {
        todo!()
    }

    fn decompose(_eval: &SecureEvaluation<Self, BitReversedOrder>) -> (SecureEvaluation<Self, BitReversedOrder>, SecureField) {
        todo!()
    }
}

impl AccumulationOps for MetalBackend {
    fn accumulate(_column: &mut SecureColumnByCoords<Self>, _other: &SecureColumnByCoords<Self>) {
        todo!()
    }

    fn generate_secure_powers(_felt: SecureField, _n_powers: usize) -> Vec<SecureField> {
        todo!()
    }
}

impl GkrOps for MetalBackend {
    fn gen_eq_evals(_y: &[SecureField], _v: SecureField) -> Mle<Self, SecureField> {
        todo!()
    }

    fn next_layer(_layer: &Layer<Self>) -> Layer<Self> {
        todo!()
    }

    fn sum_as_poly_in_first_variable(_h: &GkrMultivariatePolyOracle<'_, Self>, _claim: SecureField) -> UnivariatePoly<SecureField> {
        todo!()
    }
}

impl MleOps<BaseField> for MetalBackend {
    fn fix_first_variable(_mle: Mle<Self, BaseField>, _assignment: SecureField) -> Mle<Self, SecureField>
    where
        Self: MleOps<SecureField>
    {
        todo!()
    }
}

impl MleOps<SecureField> for MetalBackend {
    fn fix_first_variable(_mle: Mle<Self, SecureField>, _assignment: SecureField) -> Mle<Self, SecureField>
    where
        Self: MleOps<SecureField>
    {
        todo!()
    }
}

impl Backend for MetalBackend {}

impl MerkleOps<Blake2sMerkleHasher> for MetalBackend {
    fn commit_on_layer(_log_size: u32, _prev_layer: Option<&Vec<Blake2sHash>>, _columns: &[&Col<Self, BaseField>]) -> Col<Self, Blake2sHash> {
        todo!()
    }
}

impl GrindOps<Blake2sChannel> for MetalBackend {
    fn grind(_channel: &Blake2sChannel, _pow_bits: u32) -> u64 {
        todo!()
    }
}

impl BackendForChannel<Blake2sMerkleChannel> for MetalBackend {}

impl MerkleOps<Poseidon252MerkleHasher> for MetalBackend {
    fn commit_on_layer(_log_size: u32, _prev_layer: Option<&Vec<FieldElement252>>, _columns: &[&Col<Self, BaseField>]) -> Col<Self, FieldElement252> {
        todo!()
    }
}

impl GrindOps<Poseidon252Channel> for MetalBackend {
    fn grind(_channel: &Poseidon252Channel, _pow_bits: u32) -> u64 {
        todo!()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl BackendForChannel<Poseidon252MerkleChannel> for MetalBackend {}

/// Performs a naive bit-reversal permutation inplace.
///
/// # Panics
///
/// Panics if the length of the slice is not a power of two.
pub fn bit_reverse<T>(v: &mut [T]) {
    let n = v.len();
    assert!(n.is_power_of_two());
    let log_n = n.ilog2();
    for i in 0..n {
        let j = bit_reverse_index(i, log_n);
        if j > i {
            v.swap(i, j);
        }
    }
}

impl<T: Debug + Clone + Default> ColumnOps<T> for MetalBackend {
    type Column = Vec<T>;

    fn bit_reverse_column(column: &mut Self::Column) {
        bit_reverse(column)
    }
}

impl FieldOps<BaseField> for MetalBackend {
    /// Batch inversion using Montgomery's trick.
    fn batch_inverse(column: &Self::Column, dst: &mut Self::Column) {
        let size = column.len();
        let elements_per_threadgroup: u32 = 512;
        let number_of_manual_inversions = 32;
        let shared_element_tree_size = 2 * elements_per_threadgroup - number_of_manual_inversions;

        let device = Device::system_default().expect("No Metal device found");
        let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
            "src/core/backend/metal/batch_inverse.metallib"
        );
        let library = device.new_library_with_file(library_path).unwrap();
        let function = library.get_function("base_field_batch_inverse", None).unwrap();

        let pipeline_state = device
            .new_compute_pipeline_state_with_function(&function)
            .unwrap();
        let command_queue = device.new_command_queue();

        let buffer_in1 = device.new_buffer_with_data(
            column.as_ptr() as *const _,
            (size * size_of::<f32>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );
        let buffer_result = device.new_buffer(
            (size * size_of::<f32>()) as u64,
            MTLResourceOptions::StorageModeShared,
        );
        let buffer_size = device.new_buffer_with_data(
            &(size as u32) as *const u32 as *const _,
            size_of::<u32>() as u64,
            MTLResourceOptions::StorageModeShared,  // Check
        );
        let buffer_log_size = device.new_buffer_with_data(
            &(size.ilog2()) as *const u32 as *const _,
            size_of::<u32>() as u64,
            MTLResourceOptions::StorageModeShared,  // Check
        );
        let buffer_shared_element_tree_size = device.new_buffer_with_data(
            &shared_element_tree_size as *const u32 as *const _,
            size_of::<u32>() as u64,
            MTLResourceOptions::StorageModeShared,  // Check
        );
        let buffer_log_shared_element_tree_size = device.new_buffer_with_data(
            &(shared_element_tree_size.ilog2()) as *const u32 as *const _,
            size_of::<u32>() as u64,
            MTLResourceOptions::StorageModeShared,  // Check
        );

        let command_buffer = command_queue.new_command_buffer();
        let encoder = command_buffer.new_compute_command_encoder();

        encoder.set_compute_pipeline_state(&pipeline_state);
        encoder.set_buffer(0, Some(&buffer_in1), 0);
        encoder.set_buffer(1, Some(&buffer_result), 0);
        encoder.set_buffer(2, Some(&buffer_size), 0);
        encoder.set_buffer(3, Some(&buffer_log_size), 0);
        encoder.set_buffer(4, Some(&buffer_shared_element_tree_size), 0);
        encoder.set_buffer(5, Some(&buffer_log_shared_element_tree_size), 0);
        encoder.set_threadgroup_memory_length(0, shared_element_tree_size as NSUInteger);

        let block_size = elements_per_threadgroup as NSUInteger >> 1;
        let threadgroup_size = MTLSize::new(block_size, 1, 1);
        let number_of_blocks = ((size >> 2) as NSUInteger + block_size - 1) / block_size;
        let threadgroup_count = MTLSize::new(number_of_blocks, 1, 1);
        encoder.dispatch_threads(threadgroup_size, threadgroup_count);

        encoder.end_encoding();
        command_buffer.commit();
        command_buffer.wait_until_completed();

        let result_ptr = buffer_result.contents() as *const BaseField;
        unsafe{ std::ptr::copy_nonoverlapping(result_ptr, dst.as_mut_ptr(), size); }
    }
}

impl FieldOps<SecureField> for MetalBackend {
    fn batch_inverse(_column: &Self::Column, _dst: &mut Self::Column) {
        todo!()
    }
}

pub type MetalCirclePoly = CirclePoly<MetalBackend>;
pub type MetalCircleEvaluation<F, EvalOrder> = CircleEvaluation<MetalBackend, F, EvalOrder>;
pub type MetalMle<F> = Mle<MetalBackend, F>;

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::prelude::*;
    use rand::rngs::SmallRng;

    use crate::core::backend::metal::bit_reverse;
    use crate::core::backend::{Column, metal::MetalBackend, FieldOps};
    use crate::core::fields::m31::M31;

    #[test]
    fn bit_reverse_works() {
        let mut data = [0, 1, 2, 3, 4, 5, 6, 7];
        bit_reverse(&mut data);
        assert_eq!(data, [0, 4, 2, 6, 1, 5, 3, 7]);
    }

    #[test]
    #[should_panic]
    fn bit_reverse_non_power_of_two_size_fails() {
        let mut data = [0, 1, 2, 3, 4, 5];
        bit_reverse(&mut data);
    }

    #[test]
    fn batch_inverse_base_field_test() {
        let mut rng = SmallRng::seed_from_u64(0);
        let column = rng.gen::<[M31; 16]>().to_vec();
        let expected = column.iter().map(|e| e.inverse()).collect_vec();
        let mut dst = Column::zeros(column.len());

        MetalBackend::batch_inverse(&column, &mut dst);

        assert_eq!(expected, dst);
    }
}
