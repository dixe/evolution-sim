use rand::Rng;
use crate::basic_types::Gene;

pub type GenomeFunc<R: rand::Rng> = fn(rng: &mut R, genome_len: usize) -> Vec::<Gene>;

pub fn fixed_genome<R: rand::Rng>(rng: &mut R, genome_len: usize, from_neuron: u8, to_neuron: u8) -> Vec::<Gene> {

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

pub fn random_genome<R: rand::Rng>(rng: &mut R, genome_len: usize) -> Vec::<Gene> {
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
