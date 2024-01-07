pub struct Pos {
    x: u8,
    y: u8,
}

const BOARD_MASK: u32 = 0b1111_1111_1111_1111_1111;
const C1: u32 = 0b0000_0000_0000_0000_1111;
const C2: u32 = C1 << 4;
const C3: u32 = C1 << 8;
const C4: u32 = C1 << 12;
const R1: u32 = C1 << 16;
const R2: u32 = 0b0001_0001_0001_0001;
const R3: u32 = R2 << 1;
const R4: u32 = R2 << 2;
const R5: u32 = R2 << 3;

const CELL_MASKS: [u32; 20] = [
    C1 & R1, C2 & R1, C3 & R1, C4 & R1,
    C1 & R2, C2 & R2, C3 & R2, C4 & R2,
    C1 & R3, C2 & R3, C3 & R3, C4 & R3,
    C1 & R4, C2 & R4, C3 & R4, C4 & R4,
    C1 & R5, C2 & R5, C3 & R5, C4 & R5,
];

#[derive(Debug, PartialEq)]
pub struct Board {
    mask: u32,
    p_2x1: u32,
    p_2x2: u32,
    p_1x2: u32,
    p_1x1: u32,
}

impl Board {
    pub fn new(p_2x2: u32, p_2x1: u32, p_1x2: u32, p_1x1: u32) -> Board {
        let mut mask = p_1x1;
        mask |= p_2x1 | (p_2x1 << 1);
        mask |= p_1x2 | (p_1x2 << 4);
        mask |= p_2x2 | (p_2x2 << 5) | (p_2x2 << 4) | (p_2x2 << 1);
        Board {
            mask: p_2x2 | p_1x2 | p_2x1 | p_1x1,
            p_2x2,
            p_2x1,
            p_1x2,
            p_1x1,
        }
    }

    pub fn flip_h(&self) -> Board {
        let p_2x2 = flip_2(self.p_2x2);
        let p_2x1 = flip_2(self.p_2x1);
        let p_1x2 = flip_1(self.p_1x2);
        let p_1x1 = flip_1(self.p_1x1);
        return Board::new(p_2x2, p_2x1, p_1x2, p_1x1);

        fn flip_1(p: u32) -> u32 {
            0
                | ((p & C1) << 3)
                | ((p & C2) << 1)
                | ((p & C3) >> 1)
                | ((p & C4) >> 3)
        }
        fn flip_2(p: u32) -> u32 {
            0
                | ((p & C1) << 2)
                | (p & C2)
                | ((p & C3) >> 2)
        }
    }

    pub fn follow(&self) -> Vec<Board> {
        let free = !self.mask & BOARD_MASK;
        let mut v = Vec::new();
        for i in 0..=19
        {
            if free & CELL_MASKS[i] != 0 {
                self.follow_single(&mut v, i);
            }
        }
        return v;
    }


    fn follow_single(&self, v: &mut Vec<Board>, index: usize) {
        let m = CELL_MASKS[index];
        self.follow_2x2(v, m);
        self.follow_2x1(v, m);
        self.follow_1x2(v, m);
        self.follow_1x1(v, m);
    }

    fn follow_2x2(&self, v: &mut Vec<Board>, m: u32) {
        if left(left(m)) & self.p_2x2 != 0 && down(m) & self.mask == 0 {
            v.push(Board::new(
                right(self.p_2x2),
                self.p_2x1,
                self.p_1x2,
                self.p_1x1,
            ));
        }

        if right(m) & self.p_2x2 != 0 && down(m) & self.mask == 0 {
            v.push(Board::new(
                left(self.p_2x2),
                self.p_2x1,
                self.p_1x2,
                self.p_1x1,
            ));
        }

        if up(up(m)) & self.p_2x2 != 0 && right(m) & self.mask == 0 {
            v.push(Board::new(
                down(self.p_2x2),
                self.p_2x1,
                self.p_1x2,
                self.p_1x1,
            ));
        }

        if down(m) & self.p_2x2 != 0 && right(m) & self.mask == 0 {
            v.push(Board::new(
                up(self.p_2x2),
                self.p_2x1,
                self.p_1x2,
                self.p_1x1,
            ));
        }
    }

    fn follow_2x1(&self, v: &mut Vec<Board>, m: u32) {
        if left(left(m)) & self.p_2x1 != 0 {
            v.push(Board::new(
                self.p_2x2,
                (self.p_2x1 ^ left(left(m))) | left(m),
                self.p_1x2,
                self.p_1x1,
            ));
        }

        if right(m) & self.p_2x1 != 0 {
            v.push(Board::new(
                self.p_2x2,
                (self.p_2x1 ^ right(m)) | m,
                self.p_1x2,
                self.p_1x1,
            ));
        }

        if (up(m) & self.p_2x1 != 0) && (right(m) & self.mask == 0) {
            v.push(Board::new(
                self.p_2x2,
                (self.p_2x1 ^ up(m)) | m,
                self.p_1x2,
                self.p_1x1,
            ));
        }

        if (down(m) & self.p_2x1 != 0) && (right(m) & self.mask == 0) {
            v.push(Board::new(
                self.p_2x2,
                (self.p_2x1 ^ down(m)) | m,
                self.p_1x2,
                self.p_1x1,
            ));
        }
    }

    fn follow_1x2(&self, v: &mut Vec<Board>, m: u32) {
        if up(up(m)) & self.p_1x2 != 0 {
            v.push(Board::new(
                self.p_2x2,
                self.p_2x1,
                (self.p_1x2 ^ up(up(m))) | up(m),
                self.p_1x1,
            ));
        }

        if down(m) & self.p_1x2 != 0 {
            v.push(Board::new(
                self.p_2x2,
                self.p_2x1,
                (self.p_1x2 ^ down(m)) | m,
                self.p_1x1,
            ));
        }

        if left(m) & self.p_1x2 != 0 && down(m) & self.mask == 0 {
            v.push(Board::new(
                self.p_2x2,
                self.p_2x1,
                (self.p_1x2 ^ left(m)) | m,
                self.p_1x1,
            ));
        }

        if right(m) & self.p_1x2 != 0 && down(m) & self.mask == 0 {
            v.push(Board::new(
                self.p_2x2,
                self.p_2x1,
                (self.p_1x2 ^ right(m)) | m,
                self.p_1x1,
            ));
        }
    }

    fn follow_1x1(&self, v: &mut Vec<Board>, m: u32) {
        if left(m) & self.p_1x1 != 0 {
            v.push(Board::new(
                self.p_2x2,
                self.p_2x1,
                self.p_1x2,
                (self.p_1x1 ^ left(m)) | m,
            ));
        }

        if right(m) & self.p_1x1 != 0 {
            v.push(Board::new(
                self.p_2x2,
                self.p_2x1,
                self.p_1x2,
                (self.p_1x1 ^ right(m)) | m,
            ));
        }

        if up(m) & self.p_1x1 != 0 {
            v.push(Board::new(
                self.p_2x2,
                self.p_2x1,
                self.p_1x2,
                (self.p_1x1 ^ up(m)) | m,
            ));
        }

        if down(m) & self.p_1x1 != 0 {
            v.push(Board::new(
                self.p_2x2,
                self.p_2x1,
                self.p_1x2,
                (self.p_1x1 ^ down(m)) | m,
            ));
        }
    }
}

fn up(m: u32) -> u32 {
    m >> 4
}

fn down(m: u32) -> u32 {
    (m << 4) & BOARD_MASK
}

fn right(m: u32) -> u32 {
    (m << 1) & !C1 & BOARD_MASK
}

fn left(m: u32) -> u32 {
    (m >> 1) & !C4
}