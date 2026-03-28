/*
* A function that smoothes out audio frequency inputs to avoid short bass range and long treble ragne
*/
export function logarithmicInput(value: number, min: number, max: number, nbSteps: number) {
  return min + (max - min) * Math.pow(value / nbSteps, 2.5);
}
