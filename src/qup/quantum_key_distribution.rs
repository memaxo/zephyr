use crate::qup::crypto::{QUPKeyPair, QUPPublicKey, QUPSecretKey};
use hmac::{Hmac, Mac, NewMac};
use rand::Rng;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct QuantumKeyDistribution {
    // Implementation details
}

impl QuantumKeyDistribution {
    pub fn new() -> Self {
        QuantumKeyDistribution {
            // Initialize the necessary fields
        }
    }

    fn authenticate_message(&self, message: &[u8], key: &[u8]) -> Vec<u8> {
        let mut mac = HmacSha256::new_varkey(key).expect("HMAC can take key of any size");
        mac.update(message);
        mac.finalize().into_bytes().to_vec()
    }

    fn verify_message(&self, message: &[u8], key: &[u8], mac: &[u8]) -> bool {
        let mut expected_mac = HmacSha256::new_varkey(key).expect("HMAC can take key of any size");
        expected_mac.update(message);
        expected_mac.verify(mac).is_ok()
    }

    pub fn bb84_protocol(
        &self,
        key_size: usize,
        security_parameter: usize,
    ) -> (QUPPublicKey, QUPSecretKey) {
        // Generate authentication keys for Alice and Bob
        let alice_auth_key = self.generate_authentication_key();
        let bob_auth_key = self.generate_authentication_key();

        // 1. Alice generates a random sequence of qubits
        let alice_qubits = self.generate_random_qubits(key_size);

        // 2. Alice chooses a random sequence of bases (+ or x) for each qubit
        let alice_bases = self.generate_random_bases(key_size);

        // 3. Alice chooses a random sequence of intensity levels (signal or decoy) for each qubit
        let alice_intensities = self.generate_random_intensities(key_size);

        // 4. Alice prepares the qubits according to the chosen bases and intensity levels
        let prepared_qubits = self.prepare_qubits(&alice_qubits, &alice_bases, &alice_intensities);

        // 5. Alice sends the prepared qubits to Bob over a quantum channel
        let bob_qubits = self.send_qubits(&prepared_qubits);

        // 6. Bob measures each qubit in a randomly chosen basis (+ or x)
        let bob_bases = self.generate_random_bases(key_size);
        let bob_measurements = self.measure_qubits(&bob_qubits, &bob_bases);

        // 7. Bob publicly announces his measurement bases and the positions of the decoy states
        let decoy_positions = self.announce_decoy_positions(&bob_bases, &alice_intensities);

        // 8. Alice publicly announces her preparation bases and intensity levels for the decoy states
        let alice_decoy_bases = self.extract_decoy_bases(&alice_bases, &decoy_positions);
        let alice_decoy_intensities =
            self.extract_decoy_intensities(&alice_intensities, &decoy_positions);

        // 9. Alice and Bob estimate the quantum bit error rate (QBER) for the decoy states
        let decoy_qber = self.estimate_decoy_qber(
            &alice_decoy_bases,
            &bob_bases,
            &bob_measurements,
            &decoy_positions,
        );

        // 10. Alice and Bob discard the decoy states and the bits where their bases do not match
        let sifted_key = self.sift_key(
            &alice_bases,
            &bob_bases,
            &bob_measurements,
            &decoy_positions,
        );

        // Authenticate the sifted key
        let sifted_key_auth = self.authenticate_message(&sifted_key, &alice_auth_key);

        // Exchange the authenticated sifted key
        let bob_sifted_key_auth = self.exchange_sifted_key(&sifted_key, &sifted_key_auth);

        // Verify the authenticity of the sifted key
        if !self.verify_message(&sifted_key, &bob_auth_key, &bob_sifted_key_auth) {
            panic!("Sifted key authentication failed!");
        }

        // 11. Alice and Bob perform error correction on the sifted key
        let reconciled_key = self.reconcile_key(&sifted_key);

        // Authenticate the reconciled key
        let reconciled_key_auth = self.authenticate_message(&reconciled_key, &alice_auth_key);

        // Exchange the authenticated reconciled key
        let bob_reconciled_key_auth =
            self.exchange_reconciled_key(&reconciled_key, &reconciled_key_auth);

        // Verify the authenticity of the reconciled key
        if !self.verify_message(&reconciled_key, &bob_auth_key, &bob_reconciled_key_auth) {
            panic!("Reconciled key authentication failed!");
        }

        // 12. Alice and Bob estimate the QBER for the signal states
        let signal_qber = self.estimate_signal_qber(&reconciled_key);

        // 13. Alice and Bob perform privacy amplification to obtain the final shared key
        let final_key = self.privacy_amplification(&reconciled_key, security_parameter);

        // Convert the final key into QUPPublicKey and QUPSecretKey
        let public_key = QUPPublicKey::from_bytes(&final_key);
        let secret_key = QUPSecretKey::from_bytes(&final_key);

        (public_key, secret_key)
    }

    fn generate_authentication_key(&self) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let key_size = 32; // 256-bit key
        let mut key = vec![0; key_size];
        rng.fill_bytes(&mut key);
        key
    }

    fn exchange_sifted_key(&self, sifted_key: &[Bit], sifted_key_auth: &[u8]) -> Vec<u8> {
        // Simulate the exchange of the authenticated sifted key with Bob
        // In practice, this would involve communication over an authenticated classical channel
        // For simplicity, assume that Bob's sifted key is the same as Alice's sifted key
        sifted_key_auth.to_vec()
    }

    fn exchange_reconciled_key(
        &self,
        reconciled_key: &[Bit],
        reconciled_key_auth: &[u8],
    ) -> Vec<u8> {
        // Simulate the exchange of the authenticated reconciled key with Bob
        // In practice, this would involve communication over an authenticated classical channel
        // For simplicity, assume that Bob's reconciled key is the same as Alice's reconciled key
        reconciled_key_auth.to_vec()
    }

    fn generate_random_qubits(&self, size: usize) -> Vec<Qubit> {
        let mut rng = rand::thread_rng();
        let mut qubits = Vec::with_capacity(size);

        for _ in 0..size {
            let basis = rng.gen_range(0..=1);
            let value = rng.gen_range(0..=1);

            let qubit = match (basis, value) {
                (0, 0) => Qubit::Zero,
                (0, 1) => Qubit::One,
                (1, 0) => Qubit::Plus,
                (1, 1) => Qubit::Minus,
                _ => unreachable!(),
            };

            qubits.push(qubit);
        }

        qubits
    }

    fn generate_random_bases(&self, size: usize) -> Vec<Basis> {
        let mut rng = rand::thread_rng();
        let mut bases = Vec::with_capacity(size);

        for _ in 0..size {
            let basis = match rng.gen_range(0..=1) {
                0 => Basis::Standard,
                1 => Basis::Hadamard,
                _ => unreachable!(),
            };

            bases.push(basis);
        }

        bases
    }

    fn generate_random_intensities(&self, size: usize) -> Vec<Intensity> {
        let mut rng = rand::thread_rng();
        let mut intensities = Vec::with_capacity(size);

        for _ in 0..size {
            let intensity = match rng.gen_range(0..=1) {
                0 => Intensity::Signal,
                1 => Intensity::Decoy,
                _ => unreachable!(),
            };

            intensities.push(intensity);
        }

        intensities
    }

    fn prepare_qubits(
        &self,
        qubits: &[Qubit],
        bases: &[Basis],
        intensities: &[Intensity],
    ) -> Vec<Qubit> {
        let mut rng = rand::thread_rng();
        let mut prepared_qubits = Vec::with_capacity(qubits.len());

        for (qubit, basis, intensity) in izip!(qubits, bases, intensities) {
            // Simulate imperfect state preparation
            let preparation_error_probability = 0.01; // 1% probability of preparation error
            if rng.gen_bool(preparation_error_probability) {
                // Introduce a random error in the prepared state
                let error_state = match rng.gen_range(0..=1) {
                    0 => Qubit::Zero,
                    1 => Qubit::One,
                    _ => unreachable!(),
                };
                prepared_qubits.push(error_state);
            } else {
                // Prepare the qubit based on the chosen basis and intensity
                let prepared_qubit = match (basis, intensity) {
                    (Basis::Standard, Intensity::Signal) => *qubit,
                    (Basis::Standard, Intensity::Decoy) => {
                        if rng.gen_bool(0.5) {
                            Qubit::Zero
                        } else {
                            Qubit::One
                        }
                    }
                    (Basis::Hadamard, Intensity::Signal) => self.apply_hadamard(qubit),
                    (Basis::Hadamard, Intensity::Decoy) => {
                        if rng.gen_bool(0.5) {
                            Qubit::Plus
                        } else {
                            Qubit::Minus
                        }
                    }
                };
                prepared_qubits.push(prepared_qubit);
            }
        }

        prepared_qubits
    }

    fn send_qubits(&self, qubits: &[Qubit]) -> Vec<Qubit> {
        let mut rng = rand::thread_rng();
        let mut received_qubits = Vec::with_capacity(qubits.len());

        for qubit in qubits {
            // Simulate quantum channel noise
            let noise_probability = 0.05; // 5% probability of noise
            if rng.gen_bool(noise_probability) {
                // Apply a random Pauli error (X, Y, or Z) to the qubit
                let error_type = rng.gen_range(0..=2);
                let noisy_qubit = match error_type {
                    0 => self.apply_pauli_x(qubit),
                    1 => self.apply_pauli_y(qubit),
                    2 => self.apply_pauli_z(qubit),
                    _ => unreachable!(),
                };
                received_qubits.push(noisy_qubit);
            } else {
                received_qubits.push(*qubit);
            }

            // Simulate quantum channel loss
            let loss_probability = 0.1; // 10% probability of loss
            if rng.gen_bool(loss_probability) {
                // Remove the qubit from the received qubits
                received_qubits.pop();
            }
        }

        // Simulate eavesdropping attempt
        let eavesdropping_probability = 0.02; // 2% probability of eavesdropping
        if rng.gen_bool(eavesdropping_probability) {
            // Perform a random measurement on a subset of the qubits
            let num_eavesdropped_qubits = rng.gen_range(1..=received_qubits.len());
            let eavesdropped_indices =
                rand::seq::index::sample(&mut rng, received_qubits.len(), num_eavesdropped_qubits)
                    .into_vec();
            for index in eavesdropped_indices {
                let basis = if rng.gen_bool(0.5) {
                    Basis::Standard
                } else {
                    Basis::Hadamard
                };
                let _ = self.measure_qubit(&received_qubits[index], &basis);
            }
        }

        received_qubits
    }

    fn apply_pauli_x(&self, qubit: &Qubit) -> Qubit {
        match qubit {
            Qubit::Zero => Qubit::One,
            Qubit::One => Qubit::Zero,
            Qubit::Plus => Qubit::Plus,
            Qubit::Minus => Qubit::Minus,
        }
    }

    fn apply_pauli_y(&self, qubit: &Qubit) -> Qubit {
        match qubit {
            Qubit::Zero => Qubit::Minus,
            Qubit::One => Qubit::Plus,
            Qubit::Plus => Qubit::Zero,
            Qubit::Minus => Qubit::One,
        }
    }

    fn apply_pauli_z(&self, qubit: &Qubit) -> Qubit {
        match qubit {
            Qubit::Zero => Qubit::Zero,
            Qubit::One => Qubit::One,
            Qubit::Plus => Qubit::Minus,
            Qubit::Minus => Qubit::Plus,
        }
    }

    fn measure_qubit(&self, qubit: &Qubit, basis: &Basis) -> Bit {
        let mut rng = rand::thread_rng();

        // Simulate measurement errors
        let measurement_error_probability = 0.02; // 2% probability of measurement error
        if rng.gen_bool(measurement_error_probability) {
            // Introduce a random measurement error
            return if rng.gen_bool(0.5) {
                Bit::Zero
            } else {
                Bit::One
            };
        }

        // Perform the measurement based on the chosen basis
        match (qubit, basis) {
            (Qubit::Zero, Basis::Standard) => Bit::Zero,
            (Qubit::One, Basis::Standard) => Bit::One,
            (Qubit::Plus, Basis::Hadamard) => Bit::Zero,
            (Qubit::Minus, Basis::Hadamard) => Bit::One,
            _ => {
                // Simulate the probabilistic nature of quantum measurements
                if rng.gen_bool(0.5) {
                    Bit::Zero
                } else {
                    Bit::One
                }
            }
        }
    }

    fn apply_hadamard(&self, qubit: &Qubit) -> Qubit {
        match qubit {
            Qubit::Zero => Qubit::Plus,
            Qubit::One => Qubit::Minus,
            Qubit::Plus => Qubit::Zero,
            Qubit::Minus => Qubit::One,
        }
    }

    fn announce_decoy_positions(&self, bases: &[Basis], intensities: &[Intensity]) -> Vec<usize> {
        // Announce the positions of the decoy states
        bases
            .iter()
            .zip(intensities)
            .enumerate()
            .filter_map(|(i, (_, &intensity))| {
                if intensity == Intensity::Decoy {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn extract_decoy_bases(&self, bases: &[Basis], decoy_positions: &[usize]) -> Vec<Basis> {
        // Extract the preparation bases for the decoy states
        decoy_positions.iter().map(|&i| bases[i].clone()).collect()
    }

    fn extract_decoy_intensities(
        &self,
        intensities: &[Intensity],
        decoy_positions: &[usize],
    ) -> Vec<Intensity> {
        // Extract the intensity levels for the decoy states
        decoy_positions
            .iter()
            .map(|&i| intensities[i].clone())
            .collect()
    }

    fn estimate_decoy_qber(
        &self,
        alice_bases: &[Basis],
        bob_bases: &[Basis],
        bob_measurements: &[Bit],
        decoy_positions: &[usize],
    ) -> f64 {
        let num_decoy_bits = decoy_positions.len();
        let num_errors = decoy_positions
            .iter()
            .zip(alice_bases)
            .zip(bob_bases)
            .zip(bob_measurements)
            .filter(|(((_, alice_basis), bob_basis), bob_measurement)| {
                alice_basis == bob_basis
                    && *bob_measurement
                        != self.measure_qubit(&self.prepare_qubit(alice_basis), bob_basis)
            })
            .count();

        let qber = (num_errors as f64) / (num_decoy_bits as f64);

        // Apply finite-key correction to the QBER estimate
        let confidence_level = 0.99;
        let error_rate_deviation =
            (1.0 - confidence_level).sqrt() / (2.0 * (num_decoy_bits as f64).sqrt());
        let upper_bound_qber = qber + error_rate_deviation;

        upper_bound_qber
    }

    fn sift_key(
        &self,
        alice_bases: &[Basis],
        bob_bases: &[Basis],
        bob_measurements: &[Bit],
        decoy_positions: &[usize],
    ) -> Vec<Bit> {
        // Sift the key by discarding the decoy states and the bits where bases do not match
        alice_bases
            .iter()
            .zip(bob_bases)
            .zip(bob_measurements)
            .enumerate()
            .filter_map(|(i, ((alice_basis, bob_basis), &bit))| {
                if !decoy_positions.contains(&i) && alice_basis == bob_basis {
                    Some(bit)
                } else {
                    None
                }
            })
            .collect()
    }

    fn reconcile_key(&self, sifted_key: &[Bit]) -> Vec<Bit> {
        let num_rounds = 4;
        let mut corrected_key = sifted_key.to_vec();

        for round in 0..num_rounds {
            let block_size = 2_usize.pow(round as u32);
            let num_blocks = sifted_key.len() / block_size;

            for block_index in 0..num_blocks {
                let start_index = block_index * block_size;
                let end_index = start_index + block_size;
                let block = &corrected_key[start_index..end_index];

                let parity = block
                    .iter()
                    .fold(false, |acc, &bit| acc ^ (bit == Bit::One));

                // Exchange parity information with Bob
                let bob_parity = self.exchange_parity(block_index, parity);

                if parity != bob_parity {
                    // Perform binary search to locate and correct the error
                    let error_index = self.binary_search_error(block, bob_parity);
                    corrected_key[start_index + error_index] =
                        !corrected_key[start_index + error_index];
                }
            }
        }

        corrected_key
    }

    fn exchange_parity(&self, block_index: usize, parity: bool) -> bool {
        // Simulate the exchange of parity information with Bob
        // In practice, this would involve communication over an authenticated classical channel
        // For simplicity, assume that Bob's parity is always the same as Alice's parity
        parity
    }

    fn binary_search_error(&self, block: &[Bit], target_parity: bool) -> usize {
        let mut left = 0;
        let mut right = block.len() - 1;

        while left < right {
            let mid = (left + right) / 2;
            let parity = block[..=mid]
                .iter()
                .fold(false, |acc, &bit| acc ^ (bit == Bit::One));

            if parity == target_parity {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }

    fn estimate_signal_qber(&self, reconciled_key: &[Bit]) -> f64 {
        let num_bits = reconciled_key.len();
        let num_errors = reconciled_key
            .iter()
            .filter(|&&bit| bit != Bit::Zero)
            .count();

        let qber = (num_errors as f64) / (num_bits as f64);

        // Apply finite-key correction to the QBER estimate
        let confidence_level = 0.99;
        let error_rate_deviation =
            (1.0 - confidence_level).sqrt() / (2.0 * (num_bits as f64).sqrt());
        let upper_bound_qber = qber + error_rate_deviation;

        upper_bound_qber
    }

    fn privacy_amplification(&self, reconciled_key: &[Bit], security_parameter: usize) -> Vec<u8> {
        let key_size = reconciled_key.len();
        let output_size = key_size - security_parameter;

        // Convert the reconciled key to a binary vector
        let key_bits: Vec<bool> = reconciled_key.iter().map(|&bit| bit == Bit::One).collect();

        // Generate a random Toeplitz matrix
        let toeplitz_matrix = self.generate_toeplitz_matrix(key_size, output_size);

        // Perform matrix multiplication (Toeplitz hashing)
        let output_bits = self.multiply_toeplitz_matrix(&toeplitz_matrix, &key_bits);

        // Convert the output bits to bytes
        output_bits
            .chunks(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, &bit)| acc | ((bit as u8) << i))
            })
            .collect()
    }

    fn generate_toeplitz_matrix(&self, input_size: usize, output_size: usize) -> Vec<Vec<bool>> {
        let mut rng = rand::thread_rng();
        let mut matrix = vec![vec![false; input_size]; output_size];

        // Generate the first row and column of the Toeplitz matrix
        for i in 0..input_size {
            matrix[0][i] = rng.gen();
        }
        for i in 1..output_size {
            matrix[i][0] = rng.gen();
        }

        // Fill the rest of the matrix using the Toeplitz property
        for i in 1..output_size {
            for j in 1..input_size {
                matrix[i][j] = matrix[i - 1][j - 1];
            }
        }

        matrix
    }

    fn multiply_toeplitz_matrix(&self, matrix: &[Vec<bool>], input: &[bool]) -> Vec<bool> {
        let output_size = matrix.len();
        let mut output = vec![false; output_size];

        for i in 0..output_size {
            output[i] = matrix[i]
                .iter()
                .zip(input)
                .fold(false, |acc, (&a, &b)| acc ^ (a & b));
        }

        output
    }

    fn measure_qubit(&self, qubit: &Qubit, basis: &Basis) -> Bit {
        // Measure a qubit in the given basis
        match (qubit, basis) {
            (Qubit::Zero, Basis::Standard) => Bit::Zero,
            (Qubit::One, Basis::Standard) => Bit::One,
            (Qubit::Plus, Basis::Hadamard) => Bit::Zero,
            (Qubit::Minus, Basis::Hadamard) => Bit::One,
            _ => {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.5) {
                    Bit::Zero
                } else {
                    Bit::One
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Qubit {
    Zero,
    One,
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
enum Basis {
    Standard,
    Hadamard,
}

#[derive(Debug, Clone, PartialEq)]
enum Bit {
    Zero,
    One,
}

#[derive(Debug, Clone, PartialEq)]
enum Intensity {
    Signal,
    Decoy,
}
