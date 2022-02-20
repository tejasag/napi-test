import { sum, isEven, Animal } from "./index.js";

console.log(sum(1, 2));
console.log(isEven(2));

const animal = new Animal("dog", 3);
console.log(animal.getName());
(async () => {
  console.log(await animal.getAge());
})();
