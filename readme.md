# Gene

Implement Gene as a string of bits. Can be either 32, 64, more or in between.
One gene describes a neuro connection.

# Coordinate system
-1..1 maps in X to left..right
-1..1 maps in Y to down..top

## Examples
(-1,-1) is left bottom.
(0,1) is middle top
(1,-1) is right bottom

# Network

## Neurons

Neuros output a value between -1 and 1

### Sensory inputs
A number of input neurons.

- [x] rng

- [ ] age

- [ ] Blocked Left/right
- [ ] Blocked Left/right
- [ ] Blocked Forward

- [ ] Oscillator

- [x] Location X Left/right (middle of map 0 west -1 east 1)
- [x] Location Y Up/Down (middle of map 0 down -1 up 1)

- [x] world border distance x
- [x] world border distance y

- [ ] border distance x (distance to a wall to the left/right) might now be the world wall
- [ ] border distance y (distance to a wall up/down) might now be the world wall

- [ ] last move Y
- [ ] Last Move X

- [x] pheromone density
- [ ] pheromone gradient left/right
- [ ] pheromone gradient ip/down

- [ ] population left/right. Maybe has a max distance
- [ ] population up/down. Maybe has a max distance

- [ ] Genetic similarity of forward neighbor. Maybe have a distance so we can look X tiles forward and take the first one

- [ ] population denistiy in neightborhood
- [ ] population gradient left/right maybe merge these to be left right compared to forward?? and not fixed left/right
- [ ] population gradient up/down
- [ ] population gradient forward






### Outputs

- [x] Move forward (forward is last moved direction)
- [x] Move Left/right
- [x] Move up/down
- [ ] Move random

- [ ] set oscillator period

- [ ] Emit pheromone

- [ ] Responsivness
