use crate::{Matrix, Tuple, F};

#[must_use]
pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix<4> {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());

    let true_up = left.cross(forward);

    let orientation = matrix![
             left.x,     left.y,     left.z, 0;
          true_up.x,  true_up.y,  true_up.z, 0;
         -forward.x, -forward.y, -forward.z, 0;
                  0,          0,          0, 1;
    ];

    orientation * Matrix::translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn the_trasformation_matrix_for_the_default_orientation() {
        let from = pt(0, 0, 0);
        let to = pt(0, 0, -1);
        let up = v(0, 1, 0);

        let t = view_transform(from, to, up);

        assert_fuzzy_eq!(t, Matrix::identity());
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_the_positive_z_direction() {
        let from = pt(0, 0, 0);
        let to = pt(0, 0, 1);
        let up = v(0, 1, 0);
        let t = view_transform(from, to, up);

        assert_fuzzy_eq!(t, Matrix::scaling(-1, 1, -1));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = pt(0, 0, 8);
        let to = pt(0, 0, 0);
        let up = v(0, 1, 0);

        let t = view_transform(from, to, up);

        assert_fuzzy_eq!(t, Matrix::translation(0, 0, -8));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = pt(1, 3, 2);
        let to = pt(4, -2, 8);
        let up = v(1, 1, 0);
        let t = view_transform(from, to, up);

        let m = matrix![
          -0.50709, 0.50709,  0.67612, -2.36643;
           0.76772, 0.60609,  0.12122, -2.82843;
          -0.35857, 0.59761, -0.71714,  0.00000;
           0.00000, 0.00000,  0.00000,  1.00000;
        ];

        assert_fuzzy_eq!(t, m);
    }
}
