use std::env::{args,Args};
fn main() {
    println!("Hello, world!");
    let mut args=args();
    println!("{:?}",args);
    let first =args.nth(1).unwrap();
    let operator=args.nth(0).unwrap();
    let secound=args.nth(0).unwrap();
    

    let first_args=first.parse::<i32>().unwrap();
    let secound_args=secound.parse::<i32>().unwrap();
    let operator=operator.parse::<char>().unwrap();
 
    let result= operation(first_args,operator,secound_args);
    println!("{} {} {} = {}",first_args,operator,secound_args,result);

}
fn operation(no1:i32,operator:char,no2:i32)->i32
{
    match operator{
        '+' => no1 + no2,
        '-' => no1 - no2,
        '/' => no1 / no2,
        '*' =>no1 * no2,
        _ =>panic!("Invalid operator used"),
    }
}