fn main() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];

    let slice_v: &[f64] = &v;
    let slice_a: &[f64] = &a;

    print(slice_v);
    print(slice_a);

    print(&slice_v[0..2]);
    print(&slice_a[2..]);
    print(&slice_v[1..3]);
}

fn print(n: &[f64]) {
    for e in n {
        println!("{}", e);
    }
}
