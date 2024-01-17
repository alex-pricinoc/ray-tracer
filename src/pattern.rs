use crate::{color, Color, Matrix, Shape, Tuple};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PatternDesign {
    Stripe(Color, Color),
    Gradient(Color, Color),
    Ring(Color, Color),
    Checkers(Color, Color),
    Test,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pattern {
    design: PatternDesign,
    transform: Matrix<4>,
}

impl Pattern {
    pub fn color_at(&self, point: Tuple) -> Color {
        use PatternDesign::*;

        match self.design {
            Stripe(a, b) => {
                if point.x.floor() as isize % 2 == 0 {
                    a
                } else {
                    b
                }
            }
            Gradient(a, b) => {
                let distance = b - a;
                let fraction = point.x - point.x.floor();

                a + distance * fraction
            }
            Ring(a, b) => {
                let x2 = point.x * point.x;
                let z2 = point.z * point.z;
                if (x2 + z2).sqrt() as isize % 2 == 0 {
                    a
                } else {
                    b
                }
            }
            PatternDesign::Checkers(a, b) => {
                if (point.x.floor() + point.y.floor() + point.z.floor()) as isize % 2 == 0 {
                    a
                } else {
                    b
                }
            }
            Test => color(point.x, point.y, point.z),
        }
    }

    pub fn color_at_object(&self, object: &dyn Shape, world_point: Tuple) -> Color {
        let object_point = object.props().transform.inverse() * world_point;
        let pattern_point = self.transform.inverse() * object_point;

        self.color_at(pattern_point)
    }

    #[must_use]
    pub fn transform(mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;

        self
    }
}

#[must_use]
pub fn stripe(a: Color, b: Color) -> Pattern {
    Pattern {
        design: PatternDesign::Stripe(a, b),
        transform: Matrix::identity(),
    }
}

#[must_use]
pub fn gradient(a: Color, b: Color) -> Pattern {
    Pattern {
        design: PatternDesign::Gradient(a, b),
        transform: Matrix::identity(),
    }
}

#[must_use]
pub fn ring(a: Color, b: Color) -> Pattern {
    Pattern {
        design: PatternDesign::Ring(a, b),
        transform: Matrix::identity(),
    }
}

#[must_use]
pub fn checkers(a: Color, b: Color) -> Pattern {
    Pattern {
        design: PatternDesign::Checkers(a, b),
        transform: Matrix::identity(),
    }
}

#[must_use]
#[allow(dead_code)]
pub fn test() -> Pattern {
    Pattern {
        design: PatternDesign::Test,
        transform: Matrix::identity(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = PatternDesign::Stripe(WHITE, BLACK);

        let PatternDesign::Stripe(a, b) = pattern else {
            unreachable!()
        };

        assert_fuzzy_eq!(a, WHITE);
        assert_fuzzy_eq!(b, BLACK);
    }

    #[test]
    fn a_stripe_is_constant_in_y() {
        let pattern = stripe(WHITE, BLACK);

        assert_fuzzy_eq!(pattern.color_at(pt(0, 0, 0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0, 1, 0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0, 2, 0)), WHITE);
    }

    #[test]
    fn a_stripe_is_constant_in_z() {
        let pattern = stripe(WHITE, BLACK);

        assert_fuzzy_eq!(pattern.color_at(pt(0, 0, 0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0, 0, 1)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0, 0, 2)), WHITE);
    }
    #[test]
    fn a_stripe_alternates_in_x() {
        let pattern = stripe(WHITE, BLACK);

        assert_fuzzy_eq!(pattern.color_at(pt(0, 0, 0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0.9, 0, 0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(1, 0, 0)), BLACK);
        assert_fuzzy_eq!(pattern.color_at(pt(-0.1, 0, 0)), BLACK);
        assert_fuzzy_eq!(pattern.color_at(pt(-1, 0, 0)), BLACK);
        assert_fuzzy_eq!(pattern.color_at(pt(-1.1, 0, 0)), WHITE);
    }

    #[test]
    fn stripes_with_an_object_tranformation() {
        let object = Sphere::default().transform(Matrix::scaling(2, 2, 2));
        let pattern = stripe(WHITE, BLACK);
        let c = pattern.color_at_object(&object, pt(1.5, 0, 0));

        assert_fuzzy_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::default();
        let mut pattern = stripe(WHITE, BLACK);
        pattern.transform = Matrix::scaling(2, 2, 2);
        let c = pattern.color_at_object(&object, pt(1.5, 0, 0));

        assert_fuzzy_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_pattern_transformation() {
        let object = Sphere::default().transform(Matrix::scaling(2, 2, 2));
        let mut pattern = stripe(WHITE, BLACK);
        pattern.transform = Matrix::translation(0.5, 0, 0);
        let c = pattern.color_at_object(&object, pt(2.5, 0, 0));

        assert_fuzzy_eq!(c, WHITE);
    }

    #[test]
    fn default_pattern_transformation() {
        let pattern = test();

        assert_fuzzy_eq!(pattern.transform, Matrix::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut pattern = test();
        pattern.transform = Matrix::translation(1, 2, 3);

        assert_fuzzy_eq!(pattern.transform, Matrix::translation(1, 2, 3));
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let s = Sphere::default().transform(Matrix::scaling(2, 2, 2));
        let pattern = test();
        let c = pattern.color_at_object(&s, pt(2, 3, 4));

        assert_fuzzy_eq!(c, color(1, 1.5, 2));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let shape = Sphere::default();
        let mut pattern = test();
        pattern.transform = Matrix::scaling(2, 2, 2);
        let c = pattern.color_at_object(&shape, pt(2, 3, 4));

        assert_fuzzy_eq!(c, color(1, 1.5, 2));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let s = Sphere::default().transform(Matrix::scaling(2, 2, 2));
        let mut pattern = test();
        pattern.transform = Matrix::translation(0.5, 1, 1.5);
        let c = pattern.color_at_object(&s, pt(2.5, 3, 3.5));

        assert_fuzzy_eq!(c, color(0.75, 0.5, 0.25));
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = gradient(WHITE, BLACK);

        assert_fuzzy_eq!(pattern.color_at(pt(0, 0, 0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0.25, 0, 0)), color(0.75, 0.75, 0.75));
        assert_fuzzy_eq!(pattern.color_at(pt(0.5, 0, 0)), color(0.5, 0.5, 0.5));
        assert_fuzzy_eq!(pattern.color_at(pt(0.75, 0, 0)), color(0.25, 0.25, 0.25));
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = ring(WHITE, BLACK);
        assert_fuzzy_eq!(pattern.color_at(pt(0, 0, 0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(1, 0, 0)), BLACK);
        assert_fuzzy_eq!(pattern.color_at(pt(0, 0, 1)), BLACK);
        // 0.708 = just slightly more than √2/2
        assert_fuzzy_eq!(pattern.color_at(pt(0.708, 0, 0.708)), BLACK);
    }

    #[test]
    fn pattern_checkers() {
        // checkers should repeat in x
        let pattern = checkers(WHITE, BLACK);
        assert_fuzzy_eq!(pattern.color_at(pt(0.0, 0.0, 0.0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0.99, 0.0, 0.0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(1.01, 0.0, 0.0)), BLACK);

        // checkers should repeat in y
        let pattern = checkers(WHITE, BLACK);
        assert_fuzzy_eq!(pattern.color_at(pt(0.0, 0.0, 0.0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0.0, 0.99, 0.0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0.0, 1.01, 0.0)), BLACK);

        // checkers should repeat in z
        let pattern = checkers(WHITE, BLACK);
        assert_fuzzy_eq!(pattern.color_at(pt(0.0, 0.0, 0.0)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0.0, 0.0, 0.99)), WHITE);
        assert_fuzzy_eq!(pattern.color_at(pt(0.0, 0.0, 1.01)), BLACK);
    }
}
