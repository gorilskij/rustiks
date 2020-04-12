// all massively WIP

//use crate::cube::Cube;
//use crate::cube::resort::Resort;
//
//// TODO: remove pub s
//// manipulation (for internal use)
//#[allow(dead_code)]
//impl Cube {
//    pub fn flip_edges_at(&mut self, edge0: EdgePosition, edge1: EdgePosition) {
//        let mut flipped = 0;
//
//        self.iter_edges_mut()
//            .filter(|e|
//                e.is_at(edge0) || e.is_at(edge1))
//            .for_each(|e| {
//                e.pos = pos!(e.pos.1, e.pos.0);
//                e.resort();
//                flipped += 1;
//            });
//
//        assert_eq!(flipped, 2, "expected to flip 2 edges, flipped {}", flipped);
//    }
//
//    pub fn rotate_corner_clockwise_illegal(&mut self, position: CornerPosition, rotations: u8) {
//        assert!(rotations == 1 || rotations == 2, "illegal rotations: {}", rotations);
//
//        let mut corner = self.corner_at_mut(position);
//        let CornerPosition(f0, f1, f2) = corner.id;
//        let mut id = [f0, f1, f2];
//
//        let even = id
//            .iter()
//            .filter(|f| f.is_even())
//            .count();
//
//        if even < 2 { // 0 or 1
//            id.rotate_right(rotations as usize);
//        } else { // 2 or 3
//            id.rotate_left(rotations as usize);
//        }
//
//        corner.id = (id[0], id[1], id[2]).into();
//        corner.resort();
//    }
//
//    pub fn rotate_corners_at(
//        &mut self,
//        clockwise: CornerPosition,
//        anti_clockwise: CornerPosition,
//    ) {
//        self.rotate_corner_clockwise_illegal(clockwise, 1);
//        self.rotate_corner_clockwise_illegal(anti_clockwise, 2);
//    }
//}
