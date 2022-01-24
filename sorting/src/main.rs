use sorting;

fn main() {
    //bubble-sort
    let mut i = vec![1,10,2,3,6,9];
    sorting::bubble_sort(&mut i);
    println!("Bubble Sort: {:?}", i);
    //insertion-sort
    let mut i = vec![1,10,2,3,6,9];
    sorting::insertion_sort(&mut i);
    println!("Insertion Sort: {:?}", i);
}
