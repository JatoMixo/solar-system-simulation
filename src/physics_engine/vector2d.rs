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

impl std::ops::Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[cfg(test)]
mod vector2d_tests {
    use super::*;

    #[test]
    fn test_iterator_sum() {
        let vectors: Vec<Vector2D> = [
            Vector2D { x: 6f64, y: 4f64 },
            Vector2D { x: 2f64, y: 3f64 },
            Vector2D { x: 11f64, y: 15f64 },
        ].to_vec();

        assert_eq!(Vector2D { x: 19f64, y: 22f64 }, vectors.into_iter().sum());
    }

    #[test]
    fn test_add() {
        let vector_0 = Vector2D { x: 4f64, y: 5f64 };
        let vector_1 = Vector2D { x: 11f64, y: 2f64 };

        assert_eq!(Vector2D { x: 15f64, y: 7f64 }, vector_0 + vector_1);
    }

    #[test]
    fn test_add_assign() {
        let mut vector_0 = Vector2D { x: 4f64, y: 5f64 };
        let vector_1 = Vector2D { x: 11f64, y: 2f64 };

        vector_0 += vector_1;

        assert_eq!(Vector2D { x: 15f64, y: 7f64 }, vector_0);
    }
}