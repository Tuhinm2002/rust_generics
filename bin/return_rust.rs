fn no_return_statement(num:i32) -> i32 {
    println!("Here no semicolon is used after writing num.");
    num

}

fn with_return_statement(num:i32) -> i32 {
println!("Here semicolon is used because return keyword is used");
return num;

}


fn main(){


   let num1:i32 = 5;
    let ans1:i32 = no_return_statement(num1);
    println!("{}",ans1);
    println!("\n");
    let ans2 = with_return_statement(num1);
    println!("{}",ans2);


}
