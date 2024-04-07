pub fn sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        let middle_point = x.len() / 2;
        sort(&mut x[..middle_point], true);
        sort(&mut x[middle_point..], true);
        sub_sort(x, up);
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    unimplemented!()
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    unimplemented!()
}
