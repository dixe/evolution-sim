use rand::Rng;
use crate::basic_types::{Gene, Genome, GENE_BITS};

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


pub fn mutate_genome<R: rand::Rng>(rng: &mut R, genome: &mut Genome) {

    let len = genome.len();

    match rng.gen::<f32>() {
        // bit flip
        x if x <= 1.0 => {
            let bit_index = rng.gen_range(0..32 * len);
            bit_flip(genome, bit_index);
        },
        _ => {}
    }
}


// Flip bit at index bit_index in the genome. Seeing the whole genome a one long string of bits
// bit_index should be lower than genome.len() * gene.length (32bit atm)
fn bit_flip(genome: &mut Genome, mut bit_index: usize) {

    let gene_index = bit_index / GENE_BITS;

    let gene = &mut genome[gene_index];

    match bit_index {
        x if x < 16 => {// weight
            gene.weight ^= 1 << bit_index
        },
        x if x < 24 => { // to_neuron
            gene.to_neuron ^= 1 << (bit_index - 16)
        },
        x  => { // from_neuron
            gene.from_neuron ^= 1 << (bit_index - 24)
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bit_flip_test() {

        let mut rng = rand::thread_rng();
        let gene = fixed_genome(&mut rng, 1, 0, 0);

        let mut mut_gene = gene.clone();

        // FLIPPING FROM NEURON
        let before = mut_gene[0].from_neuron;
        bit_flip(&mut mut_gene, 31);

        assert_ne!(before, mut_gene[0].from_neuron);
        assert_eq!(128, mut_gene[0].from_neuron);

        // FLIPPING TO NEURON

        let before = mut_gene[0].to_neuron;
        bit_flip(&mut mut_gene, 17);


        assert_ne!(before, mut_gene[0].to_neuron);
        assert_eq!(2 , mut_gene[0].to_neuron);

        // FLIPPING WEIGHT
        let before = mut_gene[0].weight;
        bit_flip(&mut mut_gene, 2);

        assert_ne!(before, mut_gene[0].weight);
        assert_eq!(10004, mut_gene[0].weight);

    }

}
