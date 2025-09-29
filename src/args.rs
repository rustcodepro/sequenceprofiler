use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "sequenceprofiler",
    version = "1.0",
    about = "sequenceprofiler
    ************************************************
    Author Gaurav Sablok,
    Email: codeprog@icloud.com
    ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// identity kmer similarity index
    Sequence {
        /// provide the path to sequence file
        sequencepath: String,
        /// provide the kmer to be profiled for the sequence similarity
        sequencekmer: String,
    },
    /// identity kmer filter
    Filter {
        /// provide the path to the sequence file
        sequence: String,
        /// sequence kmer for the identity kmer
        kmer: String,
        /// provide the threshold
        threshold: String,
    },
    /// compare seq to other seq 1-1 iteration
    SequenceSeq {
        /// provide the path to sequence file
        sequencepath: String,
        /// provide the kmer to be profiled for the sequence similarity
        sequencekmer: String,
    },
    /// jellyfish counter for the long reads
    Jellyfish {
        /// please provide the path to be searched for the strings containing the kmer
        fastqfile: String,
        /// please provide the kmer to be searched for the origin
        kmer: usize,
    },
    /// finding the origin of kmers
    OriginKmer {
        /// please provide the path to be searched for the strings containing the kmer
        fastafile: String,
        /// please provide the kmer to be searched for the origin
        kmer: usize,
    },
}
