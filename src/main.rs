use customer::customer::CustomerService;

mod customer;

#[tokio::main]
async fn main() {
    let mut customerService = CustomerService::new().await;

    let mut customer = customerService.create_customer().await;
    customer.update_balance(10.54f64);
    println!("{}", customer);
    customer.update_balance(50f64);
    println!("{}", customer);
    customerService.save_customer(customer).await;
}
