fn print_arguments(args: Vec<String>)
{
    println!("Provided arguments:");
    
    for arg in args
    {
        println!("  •{}", arg);
    }
}

fn main()
{
    let args: Vec<String> = std::env::args().collect();

    print_arguments(args);
}
