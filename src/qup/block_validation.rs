use crate::qup::block::QUPBlock;
use crate::qup::validator::QUPValidator;
use crate::qup::error::Error;

pub fn validate_block(block: &QUPBlock, validator: &QUPValidator) -> Result<(), Error> {
    // Verify the block
    if validator.verify_block(block) {
        // Update reputation for successful block validation
        validator.update_reputation(ReputationAction::SuccessfulBlockValidation);
        Ok(())
    } else {
        // Handle invalid block
        validator.update_reputation(ReputationAction::FailedBlockValidation);
        Err(Error::InvalidBlock)
    }
}
