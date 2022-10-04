use itertools::izip;

pub fn phase_metrics(
    tids: &[i32],
    starts: Vec<i64>,
    ends: Vec<i64>,
    phasesets: &Vec<Option<u32>>,
) -> Vec<i64> {
    let num_reads = phasesets.len();
    let mut phased_reads = izip!(tids, starts, ends, phasesets)
        .filter(|(_, _, _, p)| p.is_some())
        .collect::<Vec<_>>();
    phased_reads.sort_unstable();

    let num_phased_reads = phased_reads.len();

    let mut phased_reads_iter = phased_reads.into_iter();
    let (mut chrom1, mut start1, mut block_end, mut phaseset1) = phased_reads_iter.next().unwrap();
    let mut phaseblocks = vec![];
    for (chrom, start, end, phaseset) in phased_reads_iter {
        if chrom == chrom1 && phaseset == phaseset1 {
            block_end = end;
            continue;
        } else {
            phaseblocks.push(block_end - start1);
            chrom1 = chrom;
            start1 = start;
            block_end = end;
            phaseset1 = phaseset;
        }
    }
    phaseblocks.push(block_end - start1);

    println!(
        "Fraction reads phased\t{}",
        (num_phased_reads as f32) / (num_reads as f32)
    );
    println!("Number of phaseblocks\t{}", phaseblocks.len());
    let phased_bases = phaseblocks.iter().sum::<i64>();
    println!("Total bases phased [Gb]\t{}", phased_bases as f64 / 1e9);
    println!("Median phaseblock length\t{}", median(&phaseblocks));
    println!(
        "N50 phaseblock length\t{}",
        get_n50(&phaseblocks, phased_bases)
    );
    phaseblocks
}

fn median(array: &[i64]) -> f64 {
    if (array.len() % 2) == 0 {
        let ind_left = array.len() / 2 - 1;
        let ind_right = array.len() / 2;
        (array[ind_left] + array[ind_right]) as f64 / 2.0
    } else {
        array[(array.len() / 2)] as f64
    }
}

fn get_n50(lengths: &Vec<i64>, nb_bases_total: i64) -> i64 {
    let mut acc = 0;
    for val in lengths.iter() {
        acc += *val;
        if acc as i64 > nb_bases_total / 2 {
            return *val;
        }
    }

    lengths[lengths.len() - 1]
}
