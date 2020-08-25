mod bank;

fn main() {
use bank::account;
let mut my_account=account{name:"Alex".to_string(),
account_no:"123456".to_string(),
sort_code:"1234".to_string(), 
balance:0.0};
let mut your_account=account{name:String::from("Bob"),
..my_account};
println!("{}", my_account.name);
println!("{}", my_account.balance);
   println!("{}", your_account.name);
   &my_account.deposit(23.5);
   println!("{}", my_account.name);
println!("{}", my_account.balance);

}
