export function fib(n) {
  if (n < 2) {
    return n;
  }

  return fib(n - 2) + fib(n - 1);
}

export function ack(m, n) {
  if (m === 0) {
    return n + 1;
  } else if (n === 0) {
    return ack(m - 1, 1);
  } else {
    return ack(m - 1, ack(m, n - 1));
  }
}

export function tak(x, y, z) {
  if (y < x) {
    return tak(tak(x-1,y,z), tak(y-1,z,x), tak(z-1,x,y));
  }

  return z;
}
