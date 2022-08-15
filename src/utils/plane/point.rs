use derive_builder::Builder;

#[derive(Clone, Debug, Builder)]
pub struct Point2D<T>
where
  T: Clone,
{
  pub x: T,
  pub y: T,
}
