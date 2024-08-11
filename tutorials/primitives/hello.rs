fn main(){
println!("Hello World!");

println!("{} days", 30);

println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

println!("{subject} {verb} {object}", subject = "A", verb= "B", object = "C");


println!("{}", 10000);
println!("{:b}", 10000);
println!("{:o}", 10000);
println!("{number:0>5}", number = 100);
println!("{number2:0<5}", number2 = 10000);
}
