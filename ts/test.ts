import { NeuralNetwork } from ".";

// we will take the example from Sebasitan Lague's video
// the network will have 2 inputs
// one for the colour of fruit (red being 0 and violet being 1)
// and the other for the percentage of spots covering the fruit
// the output is the chance that eating the fuit will kill you
// a violet fruit with not spots is expected to be extremely safe
// a red fruit with alot of spots is expected to be fatal

const nn = new NeuralNetwork([
    2,
    4,
    4,
    1,
]);

const o = nn.calculateOutput([0, 0]);

nn.layers.forEach(layer => {
    console.log(layer.neurones);
});

console.log(o);