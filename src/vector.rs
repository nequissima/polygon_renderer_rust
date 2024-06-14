pub struct vector_3d<T> {
  x: T,
  y: T,
  z: T
}

pub struct vector_2d<T> {
  x: T,
  y: T
}

pub struct triangle_2d<T> {
  p1 : vector_2d<T>,
  p2 : vector_2d<T>,
  p3 : vector_2d<T>
}

pub fn interpolate(p1 : &vector_2d<i64>, p2 : &vector_2d<i64>) -> Vec<vector_2d<i64>> {
  
}