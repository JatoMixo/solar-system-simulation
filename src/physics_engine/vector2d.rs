use std::iter::Sum;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

// Implement .sum() for Vector2D
impl Sum for Vector2D {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        
        let mut result = Vector2D { x: 0f64, y: 0f64 };

        for element in iter {
            result.x += element.x;
            result.y += element.y;
        }

        result
    }
}

#[cfg(test)]
mod vector2d_tests {
    use super::*;

    #[test]
    fn test_sum() {
        let vectors: Vec<Vector2D> = [
            Vector2D { x: 6f64, y: 4f64 },
            Vector2D { x: 2f64, y: 3f64 },
            Vector2D { x: 11f64, y: 15f64 },
        ].to_vec();

        assert_eq!(Vector2D { x: 19f64, y: 22f64 }, vectors.into_iter().sum());
    }
}