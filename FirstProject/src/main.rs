// const A:i32=50;
// fn main() {
//     const  num:[i32;4]=[3,3,4,4];
//     println!("Hello, world!{A} ");
//     let Ball=383;
// }
// fn open(num:i32){
//     let w=num+1;
// }

// fn main() {
//     let n=5;
//     let mut fact=1;
//     for i in  1..=n {
//         fact*=i;
//     }
//     println!("{fact}");
// }
fn main(){
    // let  b=String::from("Buytyer");
    // // fun(b.clone());
    // let x=&b[..];
    // println!("{x}");
    // let x=[1,3,4,2,4];
    // let slice=&x[2..4];
    struct Coffee{
        name:String,
        price:i32,
    }
    let mocha=Coffee{
        name:String::from("Mocha"),
        price:500,
    };
    print!("{}",mocha.name);


}
// fn fun(mut meal:String){
//     meal.push_str(" and fries");
// }
