
// Paths and Imports p.183
// > Import types, traits, and modules, and then use relative paths
//   to access the functions, constants, and other members within
use std::default::Default;
use std::fmt::{Debug, Display};

// Generic Structs with Lifetime Parameters p.221
pub struct Puzzle<'a> {
    // Arrays p.67
    t: Vec<&'a[u8]>,
    v: [Vec<&'a[u8]>; 4],
    h: [Vec<&'a[u8]>; 4],
    b: Vec<&'a[u8]>,
}

impl Puzzle<'_> {
    // Byte Strings p.74; Associated Consts p.219
    const DRAWING: &'static [u8] = br"
         ___________  ___________  ___________  ___________     ______________________________________________________ .
        |          _||          _||          _||          _|   |          _            _            _            _    |.
        |         |_ |         |_ |         |_ |         |_    |         | |          | |          | |          | |   |.
        |           ||_          ||_          ||_          |   |         | |__        | |__        | |__        | |__ |.
        |           | _|         | _|         | _|         |   |         |____|       |____|       |____|       |____||.
        |           ||_          ||_          ||_          |   |                                                      |.
        |           | _|         | _|         | _|         |   |______________________________________________________|.
        |  _______  ||  _______  ||  _______  ||  _______  |    ______________________________________________________ .
        | |_____  | || |  _____| || |  _____| || |  _____| |   |                                                      |.
        |  _____| | || | |_____  || | |_____  || | |_____  |   |                                                      |.
        | |  _____| || |_____  | || |_____  | || |_____  | |   |     I            I            I            I         |.
        | | |_____  ||  _____| | ||  _____| | ||  _____| | |   |                                                      |.
        | |_______| || |_______| || |_______| || |_______| |   |______________________________________________________|.
        |  _  ____  ||  _______  ||  _______  ||  _______  |    ______________________________________________________ .
        | | ||    | || |_____  | || |  _____| || |  _____| |   |                                                      |.
        | | || || | ||  _____| | || | |_____  || | |_____  |   |                                                      |.
        | | || || | || |  _____| || |_____  | || |_____  | |   |     I            I            I            I         |.
        | | || || | || | |_____  ||  _____| | ||  _____| | |   |                                                      |.
        | |____||_| || |_______| || |_______| || |_______| |   |______________________________________________________|.
        |  _  _  _  ||  _  ____  ||  _______  ||  _______  |    ______________________________________________________ .
        | | || || | || | ||    | || |_____  | || |  _____| |   |                                                      |.
        | | || || | || | || || | ||  _____| | || | |_____  |   |                                                      |.
        | |       | || | || || | || |  _____| || |_____  | |   |     I            I            I            I         |.
        | | || || | || | || || | || | |_____  ||  _____| | |   |                                                      |.
        | |_||_||_| || |____||_| || |_______| || |_______| |   |______________________________________________________|.
        |  _  _  _  ||  _  _  _  ||  _  ____  ||  _______  |    ______________________________________________________ .
        | | || || | || | || || | || | ||    | || |_____  | |   |                                                      |.
        | | || || | || | || || | || | || || | ||  _____| | |   |                                                      |.
        | |    || | || |       | || | || || | || |  _____| |   |     I            I            I            I         |.
        | | || || | || | || || | || | || || | || | |_____  |   |                                                      |.
        | |_||____| || |_||_||_| || |____||_| || |_______| |   |______________________________________________________|.
        |           ||           ||           ||           |                                                           .
        |           ||           ||           ||           |                                                           .
        |___________||___________||___________||___________|                                                           .
                                                                                                                       .
         ___________                                                                   ___________                     .
        |           |                                                                 |           |                    .
        |           |                                                                 |           |                    .
        |           |___________                ______________________________________|           |                    .
        |                       |              |            |            |                        |                    .
        |                       |              |                                                  |                    .
        |                       |______________|                                                  |                    .
        |                                                                                         |                    .
        |_________________________________________________________________________________________|                    .
        ";

    pub fn new() -> Self {
        // Statics and Constants p.187
        const T_H: usize = 7;  // top piece height
        const V_W: usize = 13; // vertical pieces width
        const V_H: usize = 34; // vertical pieces height
        const H_W: usize = 56; // horizontal pieces width
        const H_H: usize = 6;  // horizontal pieces height
        const B_W: usize = 91; // bottom piece width
        const COLUMN: usize = (4 * V_W) + 3;

        // From cargo:
        // > types that don't start with an identifier need to be surrounded with angle brackets in qualified paths
        let mut t = <Vec<&[u8]>>::default();
        let mut v = <[Vec<&[u8]>; 4]>::default();
        let mut h = <[Vec<&[u8]>; 4]>::default();
        let mut b = <Vec<&[u8]>>::default();

        // "skip" p.12,365; "enumerate" p.39,354,371; "inspect" p.369
        Self::DRAWING
            // "slice.split(is_sep)" p.404; "slice.lines()" p.445
            .split(|c| b'\n' == *c)
            .skip(1)
            .take(44)
            // Slices p.77
            .map(|l| &l[8..])
            .enumerate()
            // Formatting Numbers p.457; "String::from_utf8_lossy(byte_slice)" p.450
            .inspect(|(n, l)| println!("{:02} {}", n, String::from_utf8_lossy(l)))
            .for_each(|(n, l)| {
                if n < V_H {
                    for i in 0..4 {
                        v[i].push(&l[i * V_W..(i+1) * V_W]);
                    }
                    if n < T_H {
                        t.push(&l[COLUMN..COLUMN + H_W]);
                    }
                    else if n < T_H + (4 * H_H) {
                        h[(n - T_H) / H_H].push(&l[COLUMN..COLUMN + H_W]);
                    }
                }
                else if V_H < n {
                    b.push(&l[0..B_W]);
                }
            });

        // Syntaxic sugar p.30,210
        Self { t, v, h, b }
    }

    pub fn show(&self) {
        for i in 0..4 {
            for l in &self.v[i] {
                println!("{}", String::from_utf8_lossy(l));
            }
        }
        for i in 0..4 {
            for l in &self.h[i] {
                println!("{}", String::from_utf8_lossy(l));
            }
        }
        for l in &self.t {
            println!("{}", String::from_utf8_lossy(l));
        }
        for l in &self.b {
            println!("{}", String::from_utf8_lossy(l));
        }
    }
}
