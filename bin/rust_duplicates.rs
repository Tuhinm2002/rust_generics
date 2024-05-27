fn find_larget(list : &[i32]) -> &i32 {

    let mut largest:&i32 = &0 as &i32;
    for item in list{

        if item > largest{
        
            largest = item;
        }
    }

    largest

}


fn main(){
 let list = vec![12,600,5,0,1];

 let ans = find_larget(&list);
println!("Largest is {}",ans); 

 let list = vec![1,2,3,4,5];
 let ans = find_larget(&list);
 println!("Largest is {}",ans);




}
