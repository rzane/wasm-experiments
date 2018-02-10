import * as rust from './bench.rs';
import * as javascript from './bench.js';

const BENCHMARKS = [
  ['Fibonacci', 'fib', [35]],
  ['Ackermann', 'ack', [3, 9]],
  ['Takai', 'tak', [30, 22, 12]]
];

BENCHMARKS.forEach(([name, bench, args]) => {
  console.log(`==>> ${name}`);

  console.time('javascript');
  javascript[bench](...args);
  console.timeEnd('javascript');

  console.time('rust');
  rust[bench](...args);
  console.timeEnd('rust');
  console.log();
});
