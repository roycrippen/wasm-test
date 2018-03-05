import loadRust from './lib.rs'

const now = require("performance-now")

loadRust().then(result => {
  // const add = result.instance.exports['add'];
  // console.log('add(2, 3) = ', add(2, 3));

  // const solve = result.instance.exports['solve'];
  // console.log('p006 = ', solve(1000));

  const run = result.instance.exports['run'];
  var t0 = now();
  var sum = run(10000);
  var t1 = now();
  console.log("\n Sum = " + sum + " time = " + (t1 - t0) + " milliseconds.")

});
