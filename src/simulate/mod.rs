//! Simulate reads

/* mod declaration */
pub mod description;
pub mod error;
pub mod fragments;
pub mod quality;

/* standard use */
use std::io::Write;

/* crate use */
use anyhow::{Context, Result};
use rand::Rng;
use rand::SeedableRng;
use rayon::prelude::*;

/* local use */
use crate::cli;
use crate::model;
use crate::references::*;
use description::{Description, Origin, ReadType};
use fragments::Fragments;

#[cfg(not(tarpaulin_include))]
/// main simulate function
pub fn simulate(params: cli::simulate::Command) -> Result<()> {
    let mut main_rng = if let Some(seed) = params.seed {
        rand::rngs::StdRng::seed_from_u64(seed)
    } else {
        rand::rngs::StdRng::seed_from_u64(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .with_context(|| "Get seed for rng")?
                .as_secs(),
        )
    };

    log::info!("Start read reference");
    let references = References::from_stream(
        niffler::get_reader(Box::new(std::io::BufReader::new(
            std::fs::File::open(params.reference_path).with_context(|| "Read reference file")?,
        )))
        .with_context(|| "Read reference file niffler")?
        .0,
    )?;
    log::info!("End read reference");

    log::info!("Start init lenght model");
    let length = model::Length::new(params.length.0 as f64, params.length.1 as f64)
        .with_context(|| "Init length model")?;
    log::info!("End init lenght model");

    log::info!("Start init identity model");
    let identity = model::Identity::new(
        params.identity.0 as f64,
        params.identity.1 as f64,
        params.identity.2 as f64,
    )
    .with_context(|| "Init identity model")?;
    log::info!("End init length model");

    log::info!("Start init adapter model");
    let adapter = model::Adapter::new(
        params.start_adapter_seq.as_bytes().to_vec(),
        params.end_adapter_seq.as_bytes().to_vec(),
        params.start_adapter.0 as f64,
        params.start_adapter.1 as f64,
        params.end_adapter.0 as f64,
        params.end_adapter.1 as f64,
    )
    .with_context(|| "Init adapter model")?;
    log::info!("End init adapter model");

    log::info!("Start init glitches model");
    let glitches = model::Glitch::new(
        params.glitches.0 as f64,
        params.glitches.1 as f64,
        params.glitches.2 as f64,
    )
    .with_context(|| "Init glitches model")?;
    log::info!("End init glitches model");

    log::info!("Start read error model");
    let error = if params.error_model == *"random" {
        log::info!("Use random error model");
        model::Error::random(7)
    } else {
        log::info!("Use file error model");
        let error_path = crate::cli::simulate::found_model(params.error_model, "error".to_string())
            .with_context(|| "Get path of error model")?;
        model::Error::from_stream(
            niffler::get_reader(Box::new(std::io::BufReader::new(
                std::fs::File::open(error_path).with_context(|| "Open error model")?,
            )))
            .with_context(|| "Open error model")?
            .0,
            &mut main_rng,
        )
        .with_context(|| "Init error model")?
    };
    let k = error.k();
    log::info!("End read error model");

    log::info!("Start read quality score model");
    let qscore = if params.qscore_model == *"ideal" {
        log::info!("Use ideal quality score model");
        model::Quality::ideal()
    } else if params.qscore_model == *"random" {
        log::info!("Use random quality score model");
        model::Quality::random()
    } else {
        log::info!("Use file error model");
        let qscore_path =
            crate::cli::simulate::found_model(params.qscore_model, "qscore".to_string())
                .with_context(|| "Get path of qscore model")?;
        model::Quality::from_stream(
            niffler::get_reader(Box::new(std::io::BufReader::new(
                std::fs::File::open(qscore_path).with_context(|| "Open qscore model")?,
            )))
            .with_context(|| "Open qscore model")?
            .0,
        )?
    };
    log::info!("End read quality score model");

    let len_ref = references
        .sequences
        .iter()
        .map(|x| x.seq.len() as u64)
        .sum();
    let total_base = params.quantity.number_of_base(len_ref);
    let base_limit = if let Some(limit) = params.nb_base_store {
        limit.number_of_base(len_ref)
    } else {
        total_base
    };
    let mut base_produce = 0;
    log::info!("Target number of base {}", total_base);

    let mut output: std::io::BufWriter<Box<dyn std::io::Write>> =
        if let Some(output_path) = params.output_path {
            std::io::BufWriter::new(Box::new(
                std::fs::File::create(output_path).with_context(|| "Open output file")?,
            ))
        } else {
            std::io::BufWriter::new(Box::new(std::io::stdout()))
        };

    while base_produce < total_base {
        let base_loop = if base_limit > total_base - base_produce {
            total_base - base_produce
        } else {
            base_limit
        };

        base_produce += base_loop;

        log::info!("Start generate {} bases", base_loop);
        let sequences: Vec<(Description, Seq, Quality)> = Fragments::new(
            base_loop,
            (params.junk, params.random, params.chimera),
            &references,
            &length,
            &identity,
            &mut main_rng,
        )
        .par_bridge()
        .map(|(ref_idx, ref_idx2, description, seed)| {
            generate_read(
                (
                    &references.sequences[ref_idx],
                    &references.sequences[ref_idx2],
                ),
                description,
                &adapter,
                &error,
                &glitches,
                &qscore,
                rand::rngs::StdRng::seed_from_u64(seed),
            )
            .unwrap()
        })
        .collect();
        log::info!("End generate sequences");

        log::info!("Start write {} bases", base_loop);
        for (comment, seq, qual) in sequences {
            if seq.len() <= 14 {
                continue;
            }

            writeln!(
                output,
                "@{} {}\n{}\n+ {}\n{}",
                uuid::Uuid::new_v3(
                    &uuid::Uuid::NAMESPACE_X500,
                    &main_rng.gen::<u128>().to_be_bytes()
                )
                .to_hyphenated(),
                comment,
                std::str::from_utf8(&seq[k..(seq.len() - k)])
                    .with_context(|| "Write read in output file")?, // begin and end of fragment is just random base
                comment,
                std::str::from_utf8(&qual[k..seq.len() - k])
                    .with_context(|| "Write read in output file")?
            )
            .with_context(|| "Write read in output file")?;
        }
        log::info!("End write sequences");
    }
    Ok(())
}

type Seq = Vec<u8>;
type Quality = Vec<u8>;

/// Function realy generate read
fn generate_read(
    references: (&Reference, &Reference),
    mut description: Description,
    adapter_model: &model::Adapter,
    error_model: &model::Error,
    glitch_model: &model::Glitch,
    qscore_model: &model::Quality,
    mut rng: rand::rngs::StdRng,
) -> Result<(Description, Seq, Quality)> {
    let k = error_model.k();

    // Estimate size of final fragment all edit is consider as insertion -> it's overestimation
    let mut estimate_length = 2 * k + adapter_model.max_len() * 2 + description.length;
    estimate_length +=
        error::number_of_edit(description.identity, estimate_length).round() as usize;

    // Generate fragment
    let mut raw_fragment = Vec::with_capacity(estimate_length);
    raw_fragment.extend(crate::random_seq(k, &mut rng));

    let start_adapter = adapter_model.get_start(&mut rng);
    raw_fragment.extend(&start_adapter);

    add_fragment(
        &mut raw_fragment,
        &description.origin,
        &references.0,
        &mut rng,
    );

    // Add chimeric part
    if let Some(ref chimera) = description.chimera {
        if rng.gen_bool(crate::CHIMERA_END_ADAPTER_CHANCE) {
            raw_fragment.extend(adapter_model.get_end(&mut rng));
        }
        if rng.gen_bool(crate::CHIMERA_START_ADAPTER_CHANCE) {
            raw_fragment.extend(adapter_model.get_start(&mut rng));
        }

        add_fragment(&mut raw_fragment, &chimera, &references.1, &mut rng);
    }

    let end_adapter = adapter_model.get_end(&mut rng);
    raw_fragment.extend(&end_adapter);

    raw_fragment.extend(crate::random_seq(k, &mut rng));

    // Add error in fragment and produce quality
    let (err_fragment, cigar, real_id) = error::sequence(
        description.identity,
        &raw_fragment,
        error_model,
        glitch_model,
        &mut rng,
    );

    let mut quality = quality::generate_quality(&cigar, qscore_model, &mut rng)?;

    if quality.len() != err_fragment.len() {
        log::warn!("read and quality string have different length, if you use seed please send all run information to author.");
        quality.resize(err_fragment.len(), b'!');
    }

    description.identity = real_id * 100.0;

    Ok((description, err_fragment, quality))
}

fn add_fragment(
    raw_fragment: &mut Vec<u8>,
    origin: &Origin,
    reference: &Reference,
    rng: &mut rand::rngs::StdRng,
) {
    match origin.read_type {
        ReadType::Junk => add_junk(raw_fragment, origin.end, rng),
        ReadType::Random => add_random(raw_fragment, origin.end, rng),
        ReadType::Real => add_real_fragment(raw_fragment, origin, reference),
    }
}

fn add_junk(raw_fragment: &mut Vec<u8>, length: usize, rng: &mut rand::rngs::StdRng) {
    let small_seq = crate::random_seq(rng.gen_range(1..=5), rng);

    raw_fragment.extend(small_seq.repeat(length / small_seq.len()))
}

fn add_random(raw_fragment: &mut Vec<u8>, length: usize, rng: &mut rand::rngs::StdRng) {
    raw_fragment.extend(crate::random_seq(length, rng))
}

fn add_real_fragment(raw_fragment: &mut Vec<u8>, origin: &Origin, reference: &Reference) {
    let local_ref = if origin.strand == '+' {
        &reference.seq
    } else {
        &reference.revcomp
    };

    if origin.start < origin.end {
        raw_fragment.extend(&local_ref[origin.start..origin.end]);
    } else if reference.circular {
        raw_fragment.extend(&local_ref[origin.start..]);
        raw_fragment.extend(&local_ref[..origin.end]);
    }
}

#[cfg(test)]
mod t {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn junk_seq() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut seq = Vec::new();

        add_junk(&mut seq, 100, &mut rng);
        assert_eq!(b"AGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGATAGAT".to_vec(), seq);
    }

    #[test]
    fn random_seq() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut seq = Vec::new();

        add_random(&mut seq, 100, &mut rng);

        assert_eq!(b"TTAGATTATAGTACGGTATAGTGGTTACTATGTAGCCTAAGTGGCGCCCGTTGTAGAGGAATCCACTTATATAACACAGGTATAATCCGGACGGCATGCG".to_vec(), seq);
    }

    static FASTA: &'static [u8] = b">random_seq_0
TCCTAACGTG
>random_seq_1 depth=1.5
TCACGATTAC
>random_seq_2 circular=true
CCTATCCGAT
>random_seq_3
TGCAAGATCA
>random_seq_4
TAGCCGTGGT
>random_seq_5
CGCTTTGTGA
>random_seq_6 circular=true depth=1
CACATGGGCG
>random_seq_7 depth=2.0 circular=false
ATCTAATGCG
>random_seq_8 depth=0.5 circular=true
CGGAACTCAG
>random_seq_9
TCCCGCTGTC
>random_seq_10
TCCTAACGTGTCACGATTACCCTATCCGATTGCAAGATCATAGCCGTGGTCGCTTTGTGACACATGGGCGATCTAATGCGCGGAACTCAGTCCCGCTGTC
";

    #[test]
    fn read_reference() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let refs = References::from_stream(std::io::Cursor::new(FASTA)).unwrap();
        let length = model::Length::new(8.0, 2.0).unwrap();
        let identity = model::Identity::new(85.0, 95.0, 5.0).unwrap();

        let seqs: Vec<Vec<u8>> =
            Fragments::new(10_000, (0.0, 0.0, 0.0), &refs, &length, &identity, &mut rng)
                .map(|(ref_idx, _, description, _)| {
                    let mut seq = Vec::new();

                    add_real_fragment(&mut seq, &description.origin, &refs.sequences[ref_idx]);

                    seq
                })
                .take(10)
                .collect();

        assert_eq!(
            vec![
                b"GG".to_vec(),
                b"ACGT".to_vec(),
                b"GGGCGCA".to_vec(),
                b"CGGCT".to_vec(),
                b"A".to_vec(),
                b"TGTG".to_vec(),
                b"AAAGC".to_vec(),
                b"GTGCGCC".to_vec(),
                b"T".to_vec(),
                b"ATGGGCGC".to_vec()
            ],
            seqs
        );
    }

    #[test]
    fn produce_read() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let refs = References::from_stream(std::io::Cursor::new(FASTA)).unwrap();
        let length = model::Length::new(8.0, 2.0).unwrap();
        let identity = model::Identity::new(85.0, 95.0, 5.0).unwrap();
        let adapter = model::Adapter::new(
            "TACGTATTGCT".as_bytes().to_vec(),
            "TACGTATTGCT".as_bytes().to_vec(),
            90.0,
            60.0,
            50.0,
            20.0,
        )
        .unwrap();
        let error = model::Error::random(7);
        let qscore = model::Quality::random();
        let glitches = model::Glitch::new(50.0, 5.0, 5.0).unwrap();

        let seqs: Vec<(description::Description, Vec<u8>, Vec<u8>)> = Fragments::new(
            10_000,
            (10.0, 10.0, 25.0),
            &refs,
            &length,
            &identity,
            &mut rng,
        )
        .map(|(ref_idx, ref_idx2, description, seed)| {
            generate_read(
                (&refs.sequences[ref_idx], &refs.sequences[ref_idx2]),
                description,
                &adapter,
                &error,
                &glitches,
                &qscore,
                rand::rngs::StdRng::seed_from_u64(seed),
            )
            .unwrap()
        })
        .take(20)
        .collect();

        assert_eq!(
            vec![
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_4".to_string(),
                            strand: '+',
                            start: 7,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 3,
                        identity: 90.0
                    },
                    vec![65, 84, 84, 84, 71, 65, 84, 65, 67, 71, 71, 71, 84, 84, 65, 67, 67, 67],
                    vec![35, 44, 46, 47, 34, 51, 46, 49, 48, 51, 36, 43, 43, 39, 45, 38, 36, 34]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_0".to_string(),
                            strand: '+',
                            start: 5,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: Some(Origin {
                            ref_id: "random_seq_3".to_string(),
                            strand: '+',
                            start: 4,
                            end: 9,
                            read_type: ReadType::Real
                        }),
                        length: 11,
                        identity: 92.5925925925926
                    },
                    vec![
                        67, 67, 84, 65, 67, 65, 65, 65, 65, 67, 67, 71, 84, 84, 65, 67, 65, 71, 65,
                        84, 67, 65, 84, 65, 65, 71, 65, 65
                    ],
                    vec![
                        52, 41, 50, 42, 42, 51, 49, 47, 39, 35, 36, 45, 53, 38, 40, 35, 42, 46, 51,
                        41, 41, 47, 40, 44, 37, 47, 36, 45
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_8".to_string(),
                            strand: '-',
                            start: 3,
                            end: 2,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 9,
                        identity: 66.66666666666667
                    },
                    vec![
                        71, 84, 65, 71, 84, 65, 84, 71, 84, 65, 84, 67, 65, 71, 71, 84, 84, 67, 71,
                        65, 84, 65, 71, 84, 67, 67, 84, 67, 84, 84, 71
                    ],
                    vec![
                        45, 53, 39, 34, 38, 40, 35, 48, 38, 41, 45, 50, 41, 46, 41, 52, 49, 39, 39,
                        44, 38, 49, 48, 40, 48, 47, 53, 41, 40, 37, 38
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_4".to_string(),
                            strand: '+',
                            start: 8,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 2,
                        identity: 88.0
                    },
                    vec![
                        71, 84, 65, 65, 67, 67, 84, 67, 84, 65, 67, 84, 65, 84, 84, 71, 67, 71, 65,
                        84, 65, 71, 65, 65, 67
                    ],
                    vec![
                        45, 38, 38, 50, 40, 48, 44, 44, 47, 51, 47, 45, 50, 39, 36, 35, 53, 34, 44,
                        36, 36, 39, 43, 45, 51
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_7".to_string(),
                            strand: '+',
                            start: 5,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: Some(Origin {
                            ref_id: "random_seq_5".to_string(),
                            strand: '+',
                            start: 6,
                            end: 9,
                            read_type: ReadType::Real
                        }),
                        length: 9,
                        identity: 72.41379310344827
                    },
                    vec![
                        65, 71, 84, 71, 67, 65, 84, 84, 65, 67, 65, 84, 67, 67, 84, 84, 71, 71, 84,
                        67, 71, 84, 65, 84, 65, 71, 84, 84
                    ],
                    vec![
                        39, 39, 43, 48, 37, 52, 42, 37, 46, 39, 34, 51, 35, 46, 42, 46, 43, 37, 47,
                        52, 52, 34, 49, 46, 35, 38, 47, 37
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_5".to_string(),
                            strand: '-',
                            start: 6,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 4,
                        identity: 84.84848484848484
                    },
                    vec![
                        71, 65, 67, 71, 84, 65, 67, 84, 65, 67, 71, 84, 65, 84, 65, 71, 84, 65, 67,
                        67, 71, 84, 84, 71, 84, 84, 71, 65, 67, 84, 71
                    ],
                    vec![
                        45, 38, 38, 45, 49, 52, 47, 47, 43, 52, 42, 52, 37, 38, 36, 41, 42, 35, 41,
                        45, 50, 44, 45, 38, 52, 51, 41, 44, 51, 39, 36
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_7".to_string(),
                            strand: '+',
                            start: 8,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: Some(Origin {
                            ref_id: "random_seq_10".to_string(),
                            strand: '-',
                            start: 89,
                            end: 99,
                            read_type: ReadType::Real
                        }),
                        length: 12,
                        identity: 66.66666666666667
                    },
                    vec![
                        65, 84, 65, 71, 84, 67, 84, 65, 65, 67, 67, 65, 67, 65, 84, 84, 84, 71, 71,
                        84, 65, 84, 65, 67, 67, 84, 71, 65
                    ],
                    vec![
                        38, 38, 46, 45, 34, 41, 47, 45, 38, 51, 48, 39, 38, 47, 40, 37, 42, 53, 43,
                        49, 52, 48, 53, 47, 49, 39, 49, 44
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_6".to_string(),
                            strand: '+',
                            start: 3,
                            end: 1,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 8,
                        identity: 77.41935483870968
                    },
                    vec![
                        71, 84, 71, 67, 65, 84, 71, 71, 84, 65, 67, 71, 71, 65, 84, 71, 71, 71, 67,
                        71, 67, 84, 84, 65, 71, 84, 71, 65, 65, 67, 71, 65, 65, 67
                    ],
                    vec![
                        41, 52, 41, 46, 50, 43, 53, 40, 37, 46, 42, 38, 44, 49, 36, 40, 34, 44, 34,
                        51, 45, 34, 35, 45, 40, 46, 47, 46, 36, 37, 47, 36, 36, 43
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_10".to_string(),
                            strand: '-',
                            start: 58,
                            end: 66,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 8,
                        identity: 96.0
                    },
                    vec![
                        67, 84, 67, 84, 84, 65, 84, 65, 67, 84, 65, 84, 71, 65, 84, 67, 84, 65, 67,
                        71, 67, 71, 71, 65
                    ],
                    vec![
                        35, 40, 40, 52, 45, 36, 45, 39, 48, 34, 43, 34, 47, 41, 39, 51, 48, 40, 34,
                        40, 34, 35, 40, 53
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_4".to_string(),
                            strand: '-',
                            start: 2,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 8,
                        identity: 78.26086956521739
                    },
                    vec![
                        71, 84, 71, 67, 65, 67, 71, 65, 67, 67, 71, 71, 71, 84, 84, 84, 65, 67, 67,
                        65, 65, 67, 71, 65, 65
                    ],
                    vec![
                        48, 35, 35, 41, 42, 41, 41, 50, 34, 52, 43, 50, 36, 46, 51, 35, 41, 37, 36,
                        38, 38, 43, 52, 50, 48
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_5".to_string(),
                            strand: '-',
                            start: 2,
                            end: 8,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 6,
                        identity: 79.3103448275862
                    },
                    vec![
                        71, 67, 71, 67, 71, 84, 71, 84, 65, 67, 71, 65, 65, 84, 84, 71, 65, 67, 65,
                        71, 65, 84, 84, 84, 67, 67, 67, 84
                    ],
                    vec![
                        48, 49, 53, 48, 39, 38, 43, 53, 44, 43, 35, 53, 37, 40, 44, 36, 38, 49, 53,
                        52, 39, 44, 34, 35, 44, 35, 44, 42
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "".to_string(),
                            strand: '*',
                            start: 0,
                            end: 9,
                            read_type: ReadType::Random
                        },
                        chimera: None,
                        length: 9,
                        identity: 86.95652173913044
                    },
                    vec![
                        67, 67, 65, 84, 71, 65, 65, 71, 71, 84, 84, 84, 67, 84, 71, 67, 71, 65, 67,
                        67, 67, 71, 67, 84
                    ],
                    vec![
                        53, 51, 52, 37, 41, 46, 43, 44, 43, 47, 35, 41, 40, 47, 37, 35, 46, 34, 35,
                        37, 52, 40, 40, 36
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_0".to_string(),
                            strand: '+',
                            start: 1,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 9,
                        identity: 86.20689655172413
                    },
                    vec![
                        65, 71, 71, 71, 65, 67, 71, 65, 67, 71, 84, 67, 67, 84, 65, 65, 65, 67, 71,
                        84, 84, 65, 65, 71, 71, 71, 71, 71, 65
                    ],
                    vec![
                        40, 50, 34, 44, 35, 35, 49, 37, 38, 43, 39, 49, 43, 46, 39, 39, 38, 46, 51,
                        44, 44, 51, 40, 48, 52, 48, 49, 38, 43
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_9".to_string(),
                            strand: '+',
                            start: 8,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 2,
                        identity: 80.76923076923077
                    },
                    vec![
                        67, 65, 65, 84, 65, 84, 84, 65, 67, 67, 71, 65, 71, 84, 84, 65, 71, 67, 65,
                        71, 84, 84, 71
                    ],
                    vec![
                        45, 46, 39, 44, 45, 47, 41, 34, 40, 49, 51, 52, 34, 50, 36, 38, 42, 44, 42,
                        35, 50, 36, 49
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "".to_string(),
                            strand: '*',
                            start: 0,
                            end: 8,
                            read_type: ReadType::Random
                        },
                        chimera: None,
                        length: 8,
                        identity: 66.66666666666667
                    },
                    vec![
                        71, 71, 84, 84, 71, 67, 65, 84, 65, 67, 71, 71, 71, 67, 71, 84, 71, 67, 67,
                        84, 65, 71, 65, 84, 84, 65, 67, 65, 65, 71
                    ],
                    vec![
                        45, 46, 41, 35, 37, 37, 48, 48, 43, 46, 53, 34, 53, 41, 38, 34, 40, 49, 41,
                        47, 36, 38, 38, 36, 38, 41, 37, 46, 46, 38
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "".to_string(),
                            strand: '*',
                            start: 0,
                            end: 4,
                            read_type: ReadType::Junk
                        },
                        chimera: None,
                        length: 4,
                        identity: 95.83333333333334
                    },
                    vec![
                        71, 67, 67, 71, 67, 84, 65, 84, 65, 67, 71, 84, 84, 84, 84, 84, 71, 65, 65,
                        84, 71, 84, 65
                    ],
                    vec![
                        50, 35, 44, 52, 39, 34, 50, 39, 47, 38, 45, 41, 49, 52, 47, 51, 34, 45, 53,
                        49, 43, 44, 37
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_7".to_string(),
                            strand: '-',
                            start: 4,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 6,
                        identity: 87.5
                    },
                    vec![
                        84, 67, 71, 71, 67, 71, 65, 84, 65, 67, 71, 84, 84, 71, 84, 65, 71, 84, 84,
                        84, 84, 84, 71, 71, 67, 84
                    ],
                    vec![
                        51, 50, 37, 49, 50, 34, 53, 43, 48, 37, 44, 49, 38, 52, 37, 43, 39, 49, 52,
                        43, 46, 36, 37, 49, 39, 36
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_6".to_string(),
                            strand: '+',
                            start: 6,
                            end: 4,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 8,
                        identity: 84.0
                    },
                    vec![
                        84, 65, 84, 84, 71, 65, 65, 84, 71, 67, 71, 71, 67, 71, 71, 67, 65, 67, 65,
                        84, 71, 71, 67, 67, 84, 65
                    ],
                    vec![
                        42, 46, 34, 36, 37, 34, 52, 53, 51, 39, 43, 52, 46, 50, 38, 35, 43, 37, 48,
                        48, 49, 47, 40, 51, 49, 40
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_5".to_string(),
                            strand: '+',
                            start: 4,
                            end: 9,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 6,
                        identity: 81.81818181818181
                    },
                    vec![
                        67, 65, 71, 71, 67, 65, 84, 65, 84, 84, 71, 84, 84, 84, 71, 84, 84, 71, 65,
                        65, 67, 65
                    ],
                    vec![
                        41, 47, 35, 37, 41, 36, 45, 40, 43, 51, 44, 37, 40, 45, 42, 50, 36, 43, 40,
                        53, 46, 36
                    ]
                ),
                (
                    Description {
                        origin: Origin {
                            ref_id: "random_seq_5".to_string(),
                            strand: '+',
                            start: 0,
                            end: 6,
                            read_type: ReadType::Real
                        },
                        chimera: None,
                        length: 6,
                        identity: 81.4814814814815
                    },
                    vec![
                        67, 65, 84, 67, 84, 71, 67, 67, 84, 65, 67, 84, 65, 84, 67, 67, 84, 84, 84,
                        65, 65, 65, 71, 71, 71, 65, 84
                    ],
                    vec![
                        45, 45, 50, 47, 44, 51, 41, 52, 42, 36, 51, 48, 42, 44, 44, 49, 34, 53, 39,
                        53, 53, 43, 45, 48, 46, 50, 47
                    ]
                )
            ],
            seqs
        );
    }
}
