#[no_mangle]
pub fn fib(n: i32) -> i32 {
    if (n < 2) {
        return n;
    }

    return fib(n - 2) + fib(n - 1);
}

#[no_mangle]
pub fn ack(m: i32, n: i32) -> i32 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}

#[no_mangle]
pub fn tak(x: i32, y: i32, z: i32) -> i32 {
  if (y < x) {
    return tak(tak(x-1,y,z), tak(y-1,z,x), tak(z-1,x,y));
  }

  return z;
}
