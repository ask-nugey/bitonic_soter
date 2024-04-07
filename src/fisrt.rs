pub fn sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        let middle_point = x.len() / 2;
        sort(&mut x[..middle_point], true);
        sort(&mut x[middle_point..], true);
        sub_sort(x, up);
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        compare_and_swap(x, up);
        let middle_point = x.len() / 2;
        sub_sort(&mut x[..middle_point], up);
        sub_sort(&mut x[middle_point..], up);
    }
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    let middle_point = x.len() / 2;
    for i in 0..middle_point {
        if (x[1] > x[middle_point + 1]) == up {
            x.swap(i, middle_point + i)
        }
    }
}
