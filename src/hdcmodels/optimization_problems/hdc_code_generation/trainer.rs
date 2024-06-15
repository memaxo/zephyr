use crate::hdcmodels::optimization_problems::hdc_code_generation::model::CodeGenerationModel;
use crate::hdcmodels::Dataset;
use crate::hdcmodels::optimization_problems::hdc_code_generation::optimizer::Optimizer;
use std::fs::File;
use std::io::{self, Write};

pub struct Trainer {
    model: CodeGenerationModel,
    optimizer: Optimizer,
    batch_size: usize,
    epochs: usize,
}

impl Trainer {
    pub fn new(model: CodeGenerationModel, optimizer: Optimizer, batch_size: usize, epochs: usize) -> Self {
        Trainer {
            model,
            optimizer,
            batch_size,
            epochs,
        }
    }

    pub fn train(&mut self, dataset: &Dataset) -> io::Result<()> {
        for epoch in 0..self.epochs {
            for batch in dataset.batches(self.batch_size) {
                let loss = self.model.forward(&batch);
                self.optimizer.step(&mut self.model, loss);
            }
            self.save_checkpoint(epoch)?;
        }
        Ok(())
    }

    fn save_checkpoint(&self, epoch: usize) -> io::Result<()> {
        let mut file = File::create(format!("model_checkpoint_epoch_{}.bin", epoch))?;
        let encoded_model = bincode::serialize(&self.model)?;
        file.write_all(&encoded_model)?;
        Ok(())
    }
}
