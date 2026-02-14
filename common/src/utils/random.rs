use rand::Rng;

pub trait Random {
  fn random(rng: &mut impl Rng) -> Self;
}
