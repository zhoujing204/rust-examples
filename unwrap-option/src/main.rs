fn get_config_value(key: &str) -> Option<String> {
    match key {
        "host" => Some(String::from("localhost")),
        "port" => Some(String::from("8080")),
        _ => None
    }
}

fn main() {
    // unwrap_or: Provides a default value directly
    let host = get_config_value("server")
        .unwrap_or(String::from("127.0.0.1"));


    // unwrap_or_else: Provides a default value via closure
    // Better when the default value is expensive to compute
    let port = get_config_value("my_port")
        .unwrap_or_else(|| {                  // closure parameter is empty for unwrap_or_else
            println!("Computing default port...");
            String::from("3000")
        });

    // Chaining multiple operations
    let uppercase_host = get_config_value("host")
        .unwrap_or(String::from("127.0.0.1"))
        .to_uppercase();

    println!("Host: {}", host);
    println!("Port: {}", port);
    println!("Uppercase host: {}", uppercase_host);


    let option_value : Option<i32> = Some(5);
    let unwrapped_value= option_value.unwrap();

    let none_value = None;
    let unwrap_or_value = none_value.unwrap_or(0);

    let outside_value = 10;
    let unwrap_or_else_value = none_value.unwrap_or_else(||  {
        println!("Can't unwrap none value. Doubling the outside value: {}", outside_value);
        outside_value * 2
    });

    println!("unwrapped_value: {}", unwrapped_value);
    println!("unwrap_or_value: {}", unwrap_or_value);
    println!("unwrap_or_else_value: {}", unwrap_or_else_value);
}