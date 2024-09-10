const { plus100, Algo } = require('./index')

console.assert(plus100(0) === 100, 'Simple test failed')

console.info('Simple test passed')

const algo = new Algo("blake3");
console.assert(algo.hash("hello") === "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f", 'Algo blake3 failed');

const algo2 = new Algo("default");
console.assert(algo2.hash("hello") === "16156531084128653017", 'Algo default failed');
