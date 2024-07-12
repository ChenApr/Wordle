type StdRng

pub fn stdrng_seed_from_u32(seed : Int) -> StdRng = "__moonbit_rand_unstable" "stdrng_seed_from_u64"

pub fn stdrng_gen_range(rng : StdRng, ubound : Int) -> Int = "__moonbit_rand_unstable" "stdrng_gen_range"

test "shuffle" {
  let xs = Array::new_with_index(2315, fn(i) { i })
  let rng = stdrng_seed_from_u32(20220817)
  fn rand(upper : Int) -> Int {
    stdrng_gen_range(rng, upper)
  }

  xs.shuffle_in_place(~rand)
  inspect(xs[0], content="1728")!
  inspect(xs[1], content="145")!
  inspect(xs[2], content="617")!
  inspect(xs[3], content="1399")!
}

test "shuffle2" {
  let xs = Array::new_with_index(2315, fn(i) { i })
  let rng = stdrng_seed_from_u32(20220817)
  fn rand(upper : Int) -> Int {
    stdrng_gen_range(rng, upper)
  }

  xs.shuffle_in_place(~rand)
  inspect(xs[817], content="981")!
  inspect(xs[818], content="2304")!
  inspect(xs[819], content="1811")!
  inspect(xs[820], content="1821")!
}
