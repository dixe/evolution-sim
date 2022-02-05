use rand::Rng;
use crate::basic_types::{Gene, Genome};

pub type GenomeFunc<R: rand::Rng> = fn(rng: &mut R, genome_len: usize) -> Genome;

pub fn fixed_genome<R: rand::Rng>(rng: &mut R, genome_len: usize, from_neuron: u8, to_neuron: u8) -> Genome {

    let mut res = vec![];
    for _ in 0..genome_len {
        res.push(Gene {
            from_neuron,
            to_neuron,
            weight: 10_000 // about 1
        })
    }
    res

}

pub fn genome_to_rgb(genome: &Genome) -> (u8, u8, u8) {


    let len = genome.len() as f32;

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    let mut rgb = [0.0;3];
    for (i, gene) in genome.iter().enumerate() {
        rgb[0] += gene.from_neuron as f32;
        rgb[1] += gene.to_neuron as f32;
        // Add 2^15 - 1 to make positive. Divde by 256 to simlate u8 range
        rgb[2] += ((gene.weight as f32) + (2^15 -1) as f32) / 256.0;
    }

    ((rgb[0]/ len) as u8, (rgb[1]/ len) as u8, (rgb[2]/ len) as u8)
}

pub fn random_genome<R: rand::Rng>(rng: &mut R, genome_len: usize) -> Genome {
    let mut res = vec![];
    for _ in 0..genome_len {
        res.push(Gene {
            from_neuron: rng.gen(),
            to_neuron: rng.gen(),
            weight: rng.gen()
        })
    }
    res
}


fn random_gene<R: rand::Rng>(rng: &mut R) -> Gene {
    Gene {
        from_neuron: rng.gen(),
        to_neuron: rng.gen(),
        weight: rng.gen()
    }
}
