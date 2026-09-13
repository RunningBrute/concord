fn main()
{
    let args: Vec<String> = std::env::args().collect();

    println!("Provided arguments: ");

    for arg in args
    {
        println!("  - {}", arg);
    }
}
