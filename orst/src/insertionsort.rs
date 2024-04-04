use super::Sorter;

pub struct InsertionSort {
    smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn it_works_dumb() {
    let mut things = vec![4, 3, 2, 1];
    InsertionSort { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}

#[test]
fn it_works_smart() {
    let mut things = vec![4, 3, 2, 1];
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
