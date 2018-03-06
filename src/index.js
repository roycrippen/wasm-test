import loadRust from './lib.rs';

const now = require('performance-now');

let rust;

function process() {
  const t0 = now();
  const solveRes = rust.solve(100);
  const sum = rust.run(100000);
  const t1 = now();
  console.log(`p006 = ${solveRes}`);
  console.log(`res = ${sum}`);
  console.log(`time = ${t1 - t0} milliseconds.\n`);
}

loadRust().then((result) => {
  rust = result.instance.exports;
  process();
});

