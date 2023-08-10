use std::ops::Range;

/// Returns a range iterator.
///
/// `seg` is the number of segments to divide the whole.
///
/// `lim` is the total number of elements.
/// 
/// For `lim` which are odd, the actual number of segments is `seg + 1`.
///
/// # Examples
///
///     seg_cnt=2, lim=6: [0..3, 3..6]
///     seg_cnt=2, lim=7: [0..3, 3..6, 6..7]
pub fn rngs(seg: usize, lim: usize) -> RngItr {
    RngItr {
        idx: 0,
        stp: lim.saturating_div(seg),
        lim,
    }
}

// A range iterator.
#[derive(Debug, Clone)]
pub struct RngItr {
    idx: usize,
    stp: usize,
    lim: usize,
}

impl Iterator for RngItr {
    type Item = Range<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == usize::MAX {
            None
        } else {
            let lim = (self.idx + self.stp).min(self.lim);
            let rng = Range {
                start: self.idx,
                end: lim,
            };
            if lim == self.lim {
                self.idx = usize::MAX;
            } else {
                self.idx += self.stp;
            }
            Some(rng)
        }
    }
}
