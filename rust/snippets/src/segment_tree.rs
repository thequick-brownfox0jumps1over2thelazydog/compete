#![allow(dead_code)]
#![allow(unused_variables)]

use cargo_snippet::snippet;

#[snippet("segment_tree")]
#[derive(Debug)]
struct SegmentTree {
    bottom_length: usize,
    initial_value: usize,
    values: Vec<usize>,
}

#[snippet("segment_tree")]
impl SegmentTree {
    fn new(n_items: usize, initial_value: usize) -> Self {
        let bottom_length = if n_items.count_ones() == 1 {
            n_items
        } else {
            1 << (64 - n_items.leading_zeros())
        };

        Self {
            bottom_length,
            initial_value,
            values: vec![initial_value; bottom_length << 1],
        }
    }

    fn update(&mut self, index: usize, value: usize) {
        let i = self.bottom_length + index;
        self.values[i] = value;

        self.apply(i >> 1)
    }

    fn apply(&mut self, index: usize) {
        let child_index = index << 1;
        self.values[index] = self.rule(self.values[child_index], self.values[child_index + 1]);

        if index > 1 {
            self.apply(index >> 1)
        }
    }

    fn query(&self, left: usize, right: usize) -> usize {
        fn _query(
            tree: &SegmentTree,
            left: usize,
            right: usize,
            search_from: usize,
            search_to: usize,
            search_index: usize,
        ) -> usize {
            if right < search_from || search_to < left {
                return tree.initial_value;
            }
            if left <= search_from && search_to <= right {
                return tree.values[search_index];
            }

            let search_midium = (search_from + search_to) >> 1;
            let next_search_index = search_index << 1;
            let left_value = _query(
                tree,
                left,
                right,
                search_from,
                search_midium,
                next_search_index,
            );
            let right_value = _query(
                tree,
                left,
                right,
                search_midium + 1,
                search_to,
                next_search_index + 1,
            );
            tree.rule(left_value, right_value)
        }

        _query(self, left, right, 0, self.bottom_length - 1, 1)
    }

    fn rule(&self, a: usize, b: usize) -> usize {
        todo!()
    }
}
