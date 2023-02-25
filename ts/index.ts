import { createArray } from "./utils/core";
import { sigmoid } from "./utils/maths";

class InputNumberInputsError extends Error {
    constructor(got: number, expected: number) {
        super(`incorrect number of inputs. layer requires exactly ${expected} inputs but got ${got} inputs`);
    };
};

const randomParameter = (max: number = 10) => {
    return parseFloat((Math.random() * max).toFixed(2));
};

export class Neurone {
    activation: number;
    bias: number;
    // here, 'weights' is the weights of all of the edges going to this neurone from each neurone in the previous layer, i.e. the input weights
    // as such, it may be undefined if the neurone is part of the input layer
    weights?: number[];

    constructor(activation: number, bias: number, weights?: number[]) {
        this.activation = activation;
        this.bias = bias;
        this.weights = weights ? [...weights] : undefined;
    };

    initializeRandom(options?: {
        weights: boolean;
        bias: boolean;
    }): void {
        if (!options || options?.weights) {
            if (!this.weights) return;
    
            for (let i = 0; i < this.weights.length; i++) {
                this.weights[i] = randomParameter();
            };
        };
        if (!options || options?.bias) {
            this.bias = randomParameter();
        };
    };

    calculateActivation(inputs: number[]): number {
        let sum = 0;

        if (inputs.length !== this.weights?.length)
            throw new InputNumberInputsError(inputs.length, this.weights?.length || 0)

        for (let i = 0; i < inputs.length; i++) {
            sum += inputs[i] * this.weights![i];
        };

        const activation = sigmoid(sum + this.bias, 20);
        
        this.activation = activation;
        return activation;
    };
};

export class Layer {
    neurones: Neurone[] = [];

    constructor(numberNeurones: number, previousLayer?: Layer) {
        this.neurones = [...Array(numberNeurones)].map(() => new Neurone(Math.floor(Math.random() * 10) / 10, 0, previousLayer ? createArray(previousLayer.neurones.length) : undefined));
    };

    initializeRandom(): void {
        this.neurones.forEach(neurone => {
            neurone.initializeRandom();
        });
    };

    calculateOutputs(inputs: number[]): number[] {
        let outputs: number[] = [];

        this.neurones.forEach(neurone => {
            outputs.push(neurone.calculateActivation(inputs));
        });

        return outputs;
    };

    calculateCost(expected: number[]): number {
        let sum = 0;

        this.neurones.forEach((neurone, i) => {
            sum += Math.pow(neurone.activation - expected[i], 2)
        });

        return sum;
    };
};

class NeuralNetwork {
    layers: Layer[] = [];

    constructor(layerSizes: number[]) {
        layerSizes.forEach((numberNeurones, i) => {
            const layer = new Layer(numberNeurones, i > 0 ? this.layers[i - 1] : undefined);
            layer.initializeRandom();

            this.layers.push(layer);
        });
    };

    calculateOutput(inputs: number[]): number[] {
        const inputLayer = this.layers[0];
        inputLayer.neurones.forEach((neurone, i) => {
            neurone.activation = inputs[i]; 
        });

        if (inputs.length !== inputLayer.neurones.length)
            throw new InputNumberInputsError(inputs.length, inputLayer.neurones.length)

        let currentLayer: Layer = this.layers[1];
        currentLayer.calculateOutputs(inputs);

        this.layers.slice(2, this.layers.length - 1).forEach(layer => {
            currentLayer = layer;
            layer.calculateOutputs(currentLayer.neurones.map(neurone => neurone.activation));
        });

        return this.layers[this.layers.length -  1].neurones.map(neurone => neurone.activation);
    };
};

export {
    NeuralNetwork
};