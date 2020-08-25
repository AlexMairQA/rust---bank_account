pub struct account
{
pub name:String,
pub account_no:String,
pub sort_code:String,
pub balance: f64
}
impl account
{
pub fn deposit(&mut self, amount:f64)
{
if amount<=0.0
{
println!("amount must be greater than zero");
}
else
{
self.balance+=amount;
}
}

pub fn withdrawal(&mut self, amount:f64)
{
if amount>0.0 && self.balance>=0.0
{
self.balance-=amount;
}
else
{
println!("Can't withdraw amount");
}
}
}

pub fn print_name()
{
let mut my_account=account{name:"Alex".to_string(), account_no:"123456".to_string(),sort_code:"1234".to_string(), balance:0.0};
    println!("{}", my_account.name);
	}

