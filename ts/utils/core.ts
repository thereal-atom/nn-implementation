export const createArray = (length: number, random?: boolean, minMax?: [number, number]): number[] => {
    return [...Array(length)].map(() => random && minMax ? Math.floor(Math.random() * (minMax[1] - minMax[0]) + minMax[0]) : 0);
};