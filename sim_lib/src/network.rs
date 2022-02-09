use crate::basic_types::*;
use crate::combined_types::*;
use crate::sensor_neurons;


const WEIGHT_SCALE: f64 = 10_000.0;



#[derive(Debug, Clone, Copy)]
struct Connection {
    input_index: usize,
    output_index: usize,
    weight: f64
}

impl Connection {
    fn new(input_index: usize, output_index: usize, weight: f64) -> Self {
        Self { input_index, output_index, weight }
    }
}

#[derive(Debug, Clone, Copy)]
struct Neuron {
    value: f64,
    action : Option::<Action>, // if None it is hidden, if not none it is index into config output Index
}


impl Neuron {

    fn hidden() -> Self {
        Self {
            value : 0.0,
            action: None
        }
    }

    fn action(action: Action) -> Self {
        Self {
            value : 0.0,
            action: Some(action)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Network {
    sensor_inputs: Vec::<Connection>, // sensor index index to neuron index
    hidden_connections: Vec::<Connection>,
    neurons: Vec::<Neuron>, // valu1es of hidden and action neurons
    action_neuron_map: std::collections::HashMap::<usize,  usize> // maps action_neuron index in config to network specific neuron. Only used on create from genome
}

impl Network {

    pub fn empty() -> Self {
        Network {
            sensor_inputs: Vec::<Connection>::new(),
            hidden_connections: Vec::<Connection>::new(),
            neurons: Vec::<Neuron>::new(),
            action_neuron_map: std::collections::HashMap::new(),
        }
    }


    pub fn initialize_from_genome(&mut self, genome: &Genome, config: &Configuration, sensor_neurons: &Vec::<Sensor>, action_neurons: &Vec::<Action>) {

        self.sensor_inputs.clear();
        self.neurons.clear();
        self.hidden_connections.clear();
        self.action_neuron_map.clear();

        let inputs_count = sensor_neurons.len();
        let input_and_hidden_count = inputs_count + config.hidden_neurons;


        // setup hidden neurons
        for _ in 0..config.hidden_neurons {
            self.neurons.push(Neuron::hidden())
        }

        for gene in genome {
            let input_index = (gene.from_neuron as usize % input_and_hidden_count) as usize;
            let output_index = self.get_output_index(config.hidden_neurons, gene.to_neuron as usize, &action_neurons);

            // scale weight from i16 range to a smaller f64 range. Along -4..4
            let weight = (gene.weight as f64) / WEIGHT_SCALE;

            if input_index < inputs_count {
                // This gene starts from a sensor input
                self.sensor_inputs.push(Connection::new(input_index, output_index, weight));
            }
            else {
                // This gene starts from a hidden neuron
                self.hidden_connections.push(Connection::new(input_index - inputs_count, output_index, weight));
            }
        }
    }


    fn get_output_index(&mut self, hidden_neurons: usize, mut to_neuron: usize, action_neurons: &Vec::<Action>) -> usize {

        to_neuron = to_neuron % (hidden_neurons + action_neurons.len());
        if to_neuron < hidden_neurons {
            to_neuron
        }
        else {
            let action_index = to_neuron - hidden_neurons;

            // if we already have a connection we have set neuron up and can just get the index.
            return match self.action_neuron_map.get(&action_index) {
                Some(&idx) => idx,
                None => {
                    // neuron not set up.
                    self.neurons.push(Neuron::action(action_neurons[action_index]));
                    let idx = self.neurons.len() - 1;
                    self.action_neuron_map.insert(action_index, idx);

                    return idx;
                }
            };
        }
    }


    pub fn run(&mut self, sensor_neurons: &Vec::<Sensor>, world: &World, individual: &Individual) -> Vec<Activation> {

        // reset old values
        for i in 0..self.neurons.len() {
            self.neurons[i].value = 0.0;
        }

        // go over all over all sensor input
        for sensor_con in &self.sensor_inputs {
            let sensor = sensor_neurons[sensor_con.input_index];
            let reading = sensor_neurons::get_sensor_input(sensor, world, individual);
            self.neurons[sensor_con.output_index].value += reading * sensor_con.weight;
        }

        // go over all hidden neuron
        for hidden_con in &self.hidden_connections {

            let reading = f64::tanh(self.neurons[hidden_con.input_index].value);

            self.neurons[hidden_con.output_index].value += reading * hidden_con.weight;
        }

        self.neurons.iter().filter(|n| n.action.is_some()).map(|n|
                                                               Activation {
                                                                   action: n.action.unwrap(),
                                                                   weight: f64::tanh(n.value),
                                                                   indiv_index: individual.index
                                                               }).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn zero_weight() {
        let gene = Gene { from_neuron: 0, to_neuron: 0, weight: 0 };


        let mut config = Configuration::default();
        let sensor_neurons = vec![Sensor::Constant, Sensor::Constant, Sensor::Constant];
        config.hidden_neurons = 0;
        let action_neurons = vec![Action::MoveForward, Action::MoveX];


        let mut network = Network::empty();

        network.initialize_from_genome(&vec![ gene ], &config, &sensor_neurons, &action_neurons);

        let world = World::new(Coord {x: 128, y: 128});
        let indiv = Individual::new();


        let actions = network.run(&sensor_neurons, &world, &indiv);

        println!("{:?}", network);

        // should be action
        assert_eq!(1, actions.len());

        // should be index 0
        assert_eq!(Action::MoveForward, actions[0].action);

        // should be weight 0
        assert_eq!(f64::tanh(0.0), actions[0].weight);

    }

    #[test]
    fn weight_1() {

        let gene = Gene { from_neuron: 0, to_neuron: 1, weight: WEIGHT_SCALE as i16 };

        let mut config = Configuration::default();
        let sensor_neurons = vec![Sensor::Constant, Sensor::Constant, Sensor::Constant];
        config.hidden_neurons = 1;
        let action_neurons = vec![Action::MoveForward, Action::MoveX];

        let mut network = Network::empty();

        network.initialize_from_genome(&vec![ gene ], &config, &sensor_neurons, &action_neurons);


        let world = World::new(Coord {x: 128, y: 128});
        let indiv = Individual::new();

        let actions = network.run(&sensor_neurons, &world, &indiv);

        println!("{:?}", network);

        // should be action
        assert_eq!(1, actions.len());

        // should be index 0
        assert_eq!(Action::MoveForward, actions[0].action);

        // should be weight 0k
        assert_eq!(f64::tanh(1.0), actions[0].weight);

    }

    #[test]
    fn weight_negative_hidden() {


        let gene = Gene { from_neuron: 0, to_neuron: 0, weight: WEIGHT_SCALE as i16};

        let gene1 = Gene { from_neuron: 1, to_neuron: 1, weight: - WEIGHT_SCALE as i16 };

        let mut config = Configuration::default();
        let sensor_neurons = vec![Sensor::Constant, Sensor::Constant, Sensor::Constant];
        config.hidden_neurons = 1;
        let action_neurons = vec![Action::MoveForward, Action::MoveX];


        let mut network = Network::empty();

        network.initialize_from_genome(&vec![ gene, gene1 ], &config, &sensor_neurons, &action_neurons);


        let world = World::new(Coord {x: 128, y: 128});
        let indiv = Individual::new();

        let actions = network.run(&sensor_neurons, &world, &indiv);

        println!("{:#?}", network);

        // should be action
        assert_eq!(1, actions.len());

        // should be index 0
        assert_eq!(Action::MoveForward, actions[0].action);

        // should be weight 0
        assert_eq!(f64::tanh(-1.0), actions[0].weight);

    }
}
