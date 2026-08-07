use handbook_engine::grounding::{
    DeltaSignalSummary, FlowPacketGrounding, GroundingEvidence, GroundingOmission,
    GroundingOutcome, GroundingProvenance, GroundingRefusal,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GroundedPacketStatus {
    Ready,
    Omitted,
    Refused,
}

#[derive(Clone, Debug)]
pub struct GroundedPacket {
    grounding: FlowPacketGrounding,
    delta_signal_summary: DeltaSignalSummary,
    provenance: GroundingProvenance,
    omissions: Vec<GroundingOmission>,
    evidence: GroundingEvidence,
}

impl GroundedPacket {
    pub fn grounding(&self) -> &FlowPacketGrounding {
        &self.grounding
    }

    pub fn delta_signal_summary(&self) -> &DeltaSignalSummary {
        &self.delta_signal_summary
    }

    pub fn provenance(&self) -> &GroundingProvenance {
        &self.provenance
    }

    pub fn omissions(&self) -> &[GroundingOmission] {
        &self.omissions
    }

    pub fn evidence(&self) -> &GroundingEvidence {
        &self.evidence
    }
}

#[derive(Clone, Debug)]
pub struct RefusedGroundedPacket {
    refusal: GroundingRefusal,
}

impl RefusedGroundedPacket {
    pub fn refusal(&self) -> &GroundingRefusal {
        &self.refusal
    }
}

#[derive(Clone, Debug)]
pub enum GroundedPacketOutcome {
    Ready(GroundedPacket),
    Omitted(GroundedPacket),
    Refused(RefusedGroundedPacket),
}

impl GroundedPacketOutcome {
    pub fn status(&self) -> GroundedPacketStatus {
        match self {
            Self::Ready(_) => GroundedPacketStatus::Ready,
            Self::Omitted(_) => GroundedPacketStatus::Omitted,
            Self::Refused(_) => GroundedPacketStatus::Refused,
        }
    }
}

pub fn adopt_grounding_outcome(outcome: GroundingOutcome) -> GroundedPacketOutcome {
    match outcome {
        GroundingOutcome::Grounded(resolution) => {
            let packet = GroundedPacket {
                grounding: resolution.flow_packet_grounding().clone(),
                delta_signal_summary: resolution.delta_signal_summary().clone(),
                provenance: resolution.provenance().clone(),
                omissions: resolution.omissions().to_vec(),
                evidence: resolution.evidence().clone(),
            };
            if packet.omissions.is_empty() {
                GroundedPacketOutcome::Ready(packet)
            } else {
                GroundedPacketOutcome::Omitted(packet)
            }
        }
        GroundingOutcome::Refused(refusal) => {
            GroundedPacketOutcome::Refused(RefusedGroundedPacket { refusal })
        }
    }
}
