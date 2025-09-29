mod args;
mod filestruct;
mod jellyfish;
mod longread;
mod sentence;
mod simfilter;
mod similarity;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::longread::kmer_fasta;
use crate::sentence::profilesseq;
use crate::simfilter::simfilterarg;
use crate::similarity::profilesimilarity;
use clap::Parser;
use figlet_rs::FIGfont;
use jellyfish::kmer_jellyfish;

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("sequenceprofiler");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::Sequence {
            sequencepath,
            sequencekmer,
        } => {
            let command = profilesimilarity(sequencepath, sequencekmer);
            println!(
                "The sequence similarity and the clustering of the sequences based on the kmer means have been written: {:?}",
                command
            );
        }
        Commands::Filter {
            sequence,
            kmer,
            threshold,
        } => {
            let command = simfilterarg(sequence, kmer, threshold).unwrap();
            println!("The filtered files have been written: {:?}", command);
        }
        Commands::SequenceSeq {
            sequencepath,
            sequencekmer,
        } => {
            let command = profilesseq(sequencepath, sequencekmer).unwrap();
            println!("The sequence similarity has been profiled:{:?}", command);
        }
        Commands::Jellyfish { fastqfile, kmer } => {
            let command = kmer_jellyfish(fastqfile, *kmer).unwrap();
            println!(
                "The jellyfish count has been completed the and the file has been written:{:?}",
                command
            );
        }
        Commands::OriginKmer { fastafile, kmer } => {
            let command = kmer_fasta(fastafile.to_string(), *kmer).unwrap();
            println!(
                "The kmer file from the given input has been written:{:?}",
                command
            );
        }
    }
}
