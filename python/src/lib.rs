use numpy::{IntoPyArray, PyArray, PyArray1, PyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use ad_ess::ad_ess::AdEss as Rust_AdEss;

use rug::Integer;

/// Encoder/decoder capable of arbitrary distributions
///
/// - `threshold`: Maximum weight level in the trellis
/// - `n_max`: Number of symbols/amplitudes
/// - `weights`: Array of weights, `weights[0]` is the weight for $a=1$, `weights[1]` for
/// $a=3$, ...
#[pyclass]
pub struct AdEss {
    adess: Rust_AdEss,
}

#[pymethods]
impl AdEss {
    /// Encoder/decoder capable of arbitrary distributions
    ///
    /// - `threshold`: Maximum weight level in the trellis
    /// - `n_max`: Number of symbols/amplitudes
    /// - `weights`: Array of weights, `weights[0]` is the weight for $a=1$, `weights[1]` for
    /// $a=3$, ...
    #[new]
    pub fn new(threshold: usize, n_max: usize, weights: Vec<usize>) -> PyResult<Self> {
        let adess = Rust_AdEss::new(threshold, n_max, &weights);
        Ok(AdEss { adess })
    }

    /// Returns a new instance for a given distribution and threshold
    ///
    /// - `threshold`: Maximum weight level in the trellis
    /// - `n_max`: Number of symbols/amplitudes
    /// - `distribution`: Array of probabilities $[P(a=1), P(a=3), P(a=5), ...]$
    /// - The `res_factor` controls a trade off between trellis size and distribution quantisation.
    /// High `res_factor` leads to fine quantisation but a potentially large trellis.
    #[staticmethod]
    pub fn new_for_distribution_threshold(
        threshold: usize,
        n_max: usize,
        distribution: Vec<f32>,
        res_factor: f32,
    ) -> PyResult<AdEss> {
        let adess =
            Rust_AdEss::new_for_distribution_threshold(threshold, n_max, &distribution, res_factor);
        match adess {
            Ok((adess, _)) => Ok(AdEss { adess }),
            Err(_) => Err(PyValueError::new_err(
                "AdEss could not be created with given configuration",
            )),
        }
    }

    /// Returns a new instance for a given distribution and minimum number of encoded bits
    ///
    /// - `num_bits`: Number of data bits that can be encoded
    /// - `n_max`: Number of symbols/amplitudes
    /// - `distribution`: Array of probabilities $[P(a=1), P(a=3), P(a=5), ...]$
    /// - The `res_factor` controls a trade off between trellis size and distribution quantisation.
    /// High `res_factor` leads to fine quantisation but a potentially large trellis.
    #[staticmethod]
    pub fn new_for_distribution_num_bits(
        num_bits: usize,
        n_max: usize,
        distribution: Vec<f32>,
        res_factor: f32,
    ) -> PyResult<AdEss> {
        let adess =
            Rust_AdEss::new_for_distribution_num_bits(num_bits, n_max, &distribution, res_factor);
        match adess {
            Ok((adess, _)) => Ok(AdEss { adess }),
            Err(_) => Err(PyValueError::new_err(
                "AdEss could not be created with given configuration",
            )),
        }
    }

    /// Returns a new instance for a given distribution using the optimal threshold
    ///
    /// According to formulas (13) and (14) in <https://doi.org/10.1109/LWC.2018.2890595>
    ///
    /// - `n_max`: Number of symbols/amplitudes
    /// - `distribution`: Array of probabilities $[P(a=1), P(a=3), P(a=5), ...]$
    /// - The `res_factor` controls a trade off between trellis size and distribution quantisation.
    /// High `res_factor` leads to fine quantisation but a potentially large trellis.
    /// - `search_width`:
    ///     Number of weight levels to check below and above the initial estimated optimal threshold
    /// - `rev_trellis_calculation_fraction`:
    ///     The fraction of the reverse trellis that should be calculated. If the calculated
    ///     fraction is to small, the optimal threshold can not be found.
    #[staticmethod]
    pub fn new_for_distribution_optimal_threshold(
        n_max: usize,
        distribution: Vec<f32>,
        res_factor: f32,
        search_width: usize,
        rev_trellis_calculation_fraction: f32,
    ) -> PyResult<AdEss> {
        let adess = Rust_AdEss::new_for_distribution_optimal_threshold(
            n_max,
            &distribution,
            res_factor,
            search_width,
            rev_trellis_calculation_fraction,
        );
        match adess {
            Ok((adess, _)) => Ok(AdEss { adess }),
            Err(_) => Err(PyValueError::new_err(
                "AdEss could not be created with given configuration",
            )),
        }
    }

    /// Calculates the trellis weights for a given distribution
    ///
    /// - `distribution`: Array of probabilities $[P(a=1), P(a=3), P(a=5), ...]$
    /// - The `res_factor` controls a trade off between trellis size and distribution quantisation.
    /// High `res_factor` leads to fine quantisation but a potentially large trellis.
    #[staticmethod]
    pub fn weights_from_distribution(
        distribution: Vec<f32>,
        res_factor: f32,
    ) -> PyResult<Vec<usize>> {
        let weights_result = Rust_AdEss::calc_weights(&distribution, res_factor);
        match weights_result {
            Ok(weights) => Ok(weights),
            Err(msg) => Err(PyValueError::new_err(msg)),
        }
    }

    /// Returns the amplitude sequence for the given bits as a numpy array
    ///
    /// The values in `index_bits` should be either `1` or `0`. (Currently other values
    /// are possible but this may change.)
    ///
    /// This function raises an exception if `index_bits` is invalid.
    ///
    /// - `index_bits` - numpy array or list of length `num_data_bits()`
    pub fn encode<'py>(
        &self,
        py: Python<'py>,
        index_bits: Vec<u8>,
    ) -> PyResult<&'py PyArray1<usize>> {
        // convert vec of index bits to Integer
        let index = index_bits
            .into_iter()
            .fold(Integer::new(), |integer, bit| (integer << 1) + bit);

        let sequence = self.adess.sequence_for_index(&index);
        Ok(sequence.into_pyarray(py))
    }

    /// Returns the amplitude sequences for multiple given bit strings as a 2D numpy array
    ///
    /// The values in `multi_index_bits` should be either `1` or `0`.
    ///
    /// This function raises an exception if one of the index bit strings in `multi_index_bits` is invalid.
    ///
    /// - `index_bits` - 2D numpy array of dimension [arbitrary, `num_data_bits()`]
    pub fn multi_encode<'py>(
        &self,
        py: Python<'py>,
        multi_index_bits: Vec<Vec<u32>>,
    ) -> PyResult<&'py PyArray2<usize>> {
        let mut sequences: Vec<Vec<usize>> = Vec::with_capacity(multi_index_bits.len());
        for index_bits in multi_index_bits {
            // convert vec of index bits to Integer
            let index = index_bits
                .into_iter()
                .fold(Integer::new(), |integer, bit| (integer << 1) + bit);

            let sequence = self.adess.sequence_for_index(&index);
            sequences.push(sequence)
        }
        let arr = PyArray::from_vec2(py, &sequences).expect("Should be valid ndarray");
        Ok(arr)
    }

    /// Returns the index corresponding to the provided amplitude sequence as a numpy
    /// array of `1`s and `0`s
    ///
    /// Raises an exception if `sequence` is invalid.
    ///
    /// - `sequence` - numpy array or list of length `n_max` (as passed to constructor)
    pub fn decode<'py>(
        &self,
        py: Python<'py>,
        sequence: Vec<usize>,
    ) -> PyResult<&'py PyArray1<u32>> {
        // decodes a sequence of amplitudes to a bit array with same length specified by `get_num_bits`

        let index = self.adess.index_for_sequence(&sequence);

        // convert index to numpy array
        let len = self.adess.num_bits() as usize;
        let mut bits_vec = vec![0; len];
        let mut mask = Integer::from(1);
        let zero = Integer::from(0);
        for i in (0..len).rev() {
            let masked_index: Integer = (&mask & &index).into();
            if masked_index != zero {
                bits_vec[i] = 1;
            }
            mask <<= 1;
        }
        Ok(bits_vec.into_pyarray(py))
    }

    /// Returns the indexes corresponding to the provided amplitude sequence as a 2D numpy
    /// array of `1`s and `0`s
    ///
    /// Raises an exception if any amplitude sequence in `sequences` is invalid.
    ///
    /// - `sequences` - 2D numpy array of dimension [arbitrary, `n_max` (as passed to constructor)]
    pub fn multi_decode<'py>(
        &self,
        py: Python<'py>,
        sequences: Vec<Vec<usize>>,
    ) -> PyResult<&'py PyArray2<u32>> {
        let mut bit_vectors = Vec::with_capacity(sequences.len());

        for sequence in sequences {
            let index = self.adess.index_for_sequence(&sequence);

            // convert index to numpy array
            let len = self.adess.num_bits() as usize;
            let mut bits_vec = vec![0; len];
            let mut mask = Integer::from(1);
            let zero = Integer::from(0);
            for i in (0..len).rev() {
                let masked_index: Integer = (&mask & &index).into();
                if masked_index != zero {
                    bits_vec[i] = 1;
                }
                mask <<= 1;
            }
            bit_vectors.push(bits_vec);
        }
        Ok(PyArray::from_vec2(py, &bit_vectors).unwrap())
    }

    /// Returns the number of bits encoded per amplitude sequence
    pub fn num_data_bits(&self) -> PyResult<u32> {
        Ok(self.adess.num_bits())
    }
    /// Returns the weights used by the internal trellis
    pub fn get_weights<'py>(&self, py: Python<'py>) -> &'py PyArray1<usize> {
        self.adess.get_weights().into_pyarray(py)
    }
    /// Returns the distribution `AdEss` is optimizing for
    pub fn get_distribution<'py>(&self, py: Python<'py>, res_factor: f32) -> &'py PyArray1<f32> {
        self.adess.get_distribution(res_factor).into_pyarray(py)
    }
    /// Returns the probabilities of the amplitude values
    ///
    /// The probabilities are returned as an array with the lowest index corresponding to the
    /// lowest amplitude.
    pub fn amplitude_distribution<'py>(&self, py: Python<'py>) -> PyResult<&'py PyArray1<f32>> {
        Ok(self.adess.amplitude_distribution().into_pyarray(py))
    }
    /// Returns the average energy of amplitude sequences
    pub fn average_energy(&self) -> PyResult<f32> {
        Ok(self.adess.average_energy())
    }
    /// Returns the maximum number of possible amplitude sequences as a string
    ///
    /// WARNING: Effect of limiting the used indexes to a power of two is not regarded!!!
    pub fn num_sequences_possible(&self) -> PyResult<String> {
        Ok(self.adess.num_sequences().to_string_radix(10))
    }
}

/// Python distribution matcher module implemented in Rust.
///
/// This module matches strings of bits to strings of amplitudes.
/// While the marginal distributions of the bits are assumed to be uniform, the distribution of the
/// amplitudes can be adapted to a channel.
///
/// Amplitudes are denoted $a \in \\{1, 3, 5, 7, \dots\\}$ in the following.
///
/// For a usage example see the `example.py` script in the repo (https://github.com/kit-cel/ad-ess)
#[pymodule]
fn pyadess(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AdEss>()?;
    Ok(())
}
