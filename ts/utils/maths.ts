export const sigmoid = (x: number, k: number = 1) => {
    return 1 / (1 + Math.exp(-x / k));
};

export const calculateVectorProduct = (vector1: number[], vector2: number[]): number => {
    let result = 0;

    vector1.forEach((a, i) => {
        result += a * vector2[i];
    });

    return result;
};

export const calculateMatrixVectorProduct = (matrix: number[][], vector: number[]): number[] => {
    const matrixVectorProduct: number[] = []

    matrix.forEach((matrixVector, i) => {
        matrixVectorProduct[i] = calculateVectorProduct(matrixVector, vector);
    });

    return matrixVectorProduct;
};