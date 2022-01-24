use sorting::bubble_sort;

fn main() {
    //bubble-sort
    let mut i = vec![1,10,2,3,6,9];
    bubble_sort(&mut i);
    println!("{:#?}", i);
}
