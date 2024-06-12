use rust_gpu_tools::{Device, Program, Buffer};
use crate::zkp::math::FieldElement;
use num_bigint::BigUint;

pub struct Fri {
    pub domain_size: usize,
    pub modulus: BigUint,
}

impl Fri {
    pub fn new(domain_size: usize, modulus: BigUint) -> Self {
        Fri { domain_size, modulus }
    }

    pub fn fft(&self, input: &[FieldElement]) -> Vec<FieldElement> {
        let devices = Device::all();
        let device = devices.first().expect("No GPU device found");

        let program = Program::from_opencl(device, include_str!("fft.cl")).expect("Failed to create program");

        let input_buffer = Buffer::builder()
            .queue(device.queue())
            .flags(rust_gpu_tools::buffer::MemFlags::new().read_write())
            .len(input.len())
            .build()
            .expect("Failed to create input buffer");

        let output_buffer = Buffer::builder()
            .queue(device.queue())
            .flags(rust_gpu_tools::buffer::MemFlags::new().read_write())
            .len(input.len())
            .build()
            .expect("Failed to create output buffer");

        input_buffer.write(&input).enq().expect("Failed to write to input buffer");

        let kernel = program.create_kernel("fft")
            .arg_buf(&input_buffer)
            .arg_buf(&output_buffer)
            .arg_scl(self.domain_size as u32)
            .arg_scl(&self.modulus.to_bytes_le())
            .build()
            .expect("Failed to create kernel");

        unsafe {
            kernel.enq().expect("Failed to enqueue kernel");
        }

        let mut output = vec![FieldElement::zero(&self.modulus); input.len()];
        output_buffer.read(&mut output).enq().expect("Failed to read from output buffer");

        output
    }
}
