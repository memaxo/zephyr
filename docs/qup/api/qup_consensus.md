# QUP Consensus

This section covers the consensus mechanism used in the QUP module.

## Overview

The QUPConsensus struct provides methods for managing the consensus process, including block proposal, voting, quorum, and block validation.

## Methods

- `new() -> Self`
- `propose_block(&self, block: QUPBlock) -> Result<(), ConsensusError>`
- `vote(&self, block: QUPBlock) -> Result<(), ConsensusError>`
- `validate_block(&self, block: QUPBlock) -> Result<(), ConsensusError>`
