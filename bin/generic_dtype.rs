fn find_largest<T:std::cmp::PartialOrd>(list:&[T]) -> &T{

    let mut largest = &list[0];

    for item in list {
        
        if item > largest{
            
            largest = item;
        }
    }

    largest
}

fn main(){
    
    println!("A program on finding largest from an array with all kinds of dtype using generic form and dtypes");

    let num_list:Vec<u32> = vec![1,2,3];
    let ans = find_largest(&num_list);
    println!("{}",ans);

    let char_list = vec!['y','j','t','a'];
    let ans = find_largest(&char_list);
    println!("{}",ans);

}
