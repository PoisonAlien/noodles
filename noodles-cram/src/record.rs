mod builder;
pub mod feature;
mod flags;
mod next_mate_flags;
mod read_group_id;
pub mod resolve;
pub mod tag;

pub use self::{
    builder::Builder, feature::Feature, flags::Flags, next_mate_flags::NextMateFlags,
    read_group_id::ReadGroupId, tag::Tag,
};

use std::{fmt, str};

use noodles_bam as bam;
use noodles_sam as sam;

#[derive(Clone, PartialEq)]
pub struct Record {
    pub id: i64,
    pub bam_bit_flags: sam::record::Flags,
    pub cram_bit_flags: Flags,
    pub reference_sequence_id: Option<bam::record::ReferenceSequenceId>,
    pub read_length: i32,
    pub alignment_start: i32,
    pub read_group: ReadGroupId,
    pub read_name: Vec<u8>,
    pub next_mate_bit_flags: NextMateFlags,
    pub next_fragment_reference_sequence_id: Option<bam::record::ReferenceSequenceId>,
    pub next_mate_alignment_start: i32,
    pub template_size: i32,
    pub distance_to_next_fragment: i32,
    pub tags: Vec<Tag>,
    pub bases: Vec<u8>,
    pub features: Vec<Feature>,
    pub mapping_quality: sam::record::MappingQuality,
    pub quality_scores: Vec<u8>,
}

impl Record {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn bam_flags(&self) -> sam::record::Flags {
        self.bam_bit_flags
    }

    pub fn flags(&self) -> Flags {
        self.cram_bit_flags
    }

    pub fn reference_sequence_id(&self) -> Option<bam::record::ReferenceSequenceId> {
        self.reference_sequence_id
    }

    pub fn read_length(&self) -> i32 {
        self.read_length
    }

    pub fn alignment_start(&self) -> i32 {
        self.alignment_start
    }

    pub fn alignment_end(&self) -> i32 {
        let mut alignment_span = self.read_length();

        // - if does not consume reference; + if does not consume read
        for feature in self.features() {
            match feature {
                Feature::Insertion(_, bases) => {
                    alignment_span -= bases.len() as i32;
                }
                Feature::InsertBase(_, _) => {
                    alignment_span -= 1;
                }
                Feature::Deletion(_, len) => {
                    alignment_span += len;
                }
                Feature::ReferenceSkip(_, len) => {
                    alignment_span += len;
                }
                Feature::SoftClip(_, bases) => {
                    alignment_span -= bases.len() as i32;
                }
                _ => {}
            }
        }

        self.alignment_start() + alignment_span - 1
    }

    pub fn read_group_id(&self) -> ReadGroupId {
        self.read_group
    }

    pub fn read_name(&self) -> &[u8] {
        &self.read_name
    }

    pub fn next_mate_flags(&self) -> NextMateFlags {
        self.next_mate_bit_flags
    }

    pub fn next_fragment_reference_sequence_id(&self) -> Option<bam::record::ReferenceSequenceId> {
        self.next_fragment_reference_sequence_id
    }

    pub fn next_mate_alignment_start(&self) -> i32 {
        self.next_mate_alignment_start
    }

    pub fn template_size(&self) -> i32 {
        self.template_size
    }

    pub fn distance_to_next_fragment(&self) -> i32 {
        self.distance_to_next_fragment
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }

    pub fn bases(&self) -> &[u8] {
        &self.bases
    }

    pub fn features(&self) -> &[Feature] {
        &self.features
    }

    pub fn add_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }

    pub fn mapping_quality(&self) -> sam::record::MappingQuality {
        self.mapping_quality
    }

    pub fn quality_scores(&self) -> &[u8] {
        &self.quality_scores
    }
}

impl Default for Record {
    fn default() -> Self {
        Builder::default().build()
    }
}

impl fmt::Debug for Record {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let read_name = str::from_utf8(self.read_name());

        fmt.debug_struct("Record")
            .field("id", &self.id)
            .field("bam_bit_flags", &self.bam_flags())
            .field("cram_bit_flags", &self.flags())
            .field("reference_id", &self.reference_sequence_id)
            .field("read_length", &self.read_length)
            .field("alignment_start", &self.alignment_start)
            .field("read_group", &self.read_group)
            .field("read_name", &read_name)
            .field("next_mate_bit_flags", &self.next_mate_flags())
            .field(
                "next_fragment_reference_sequence_id",
                &self.next_fragment_reference_sequence_id,
            )
            .field("next_mate_alignment_start", &self.next_mate_alignment_start)
            .field("template_size", &self.template_size)
            .field("distance_to_next_fragment", &self.distance_to_next_fragment)
            .field("tags", &self.tags)
            .field("bases", &self.bases)
            .field("features", &self.features)
            .field("mapping_quality", &self.mapping_quality)
            .field("quality_scores", &self.quality_scores)
            .finish()
    }
}
