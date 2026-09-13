use std::collections::HashMap;

fn print_arguments(args: &Vec<String>)
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

    print_arguments(&args);

    let statistics: HashMap<String, i16> = HashMap::new();
    for arg in args
    {
        match statistics.get(&arg)
        {
            Some(word) => println!("New word found: {}", word),
            None => println!("New word")
        }
    }
}
