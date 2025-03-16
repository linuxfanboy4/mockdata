# Rust MockData

Rust MockData is a powerful and versatile library designed to generate realistic mock data for your Rust applications. Whether you're building a testing suite, prototyping, or need placeholder data, Rust MockData provides a comprehensive set of tools to generate a wide variety of data types, including names, emails, phone numbers, addresses, UUIDs, dates, times, and more. With its flexible and extensible design, Rust MockData is the ultimate solution for all your mock data needs.

## Features

- **Comprehensive Data Generation**: Generate a wide range of mock data types, including:
  - Names (first and last names)
  - Email addresses
  - Phone numbers
  - Addresses
  - Countries
  - Job titles
  - Credit card numbers
  - Dates and times
  - UUIDs
  - Paragraphs and sentences
  - JSON objects with nested mock data

- **Customizable Randomness**: Control the randomness of generated data with a seed-based random number generator, ensuring reproducible results for testing and debugging.

- **Lightweight and Fast**: Built with performance in mind, Rust MockData is optimized for speed and efficiency, making it suitable for both small and large-scale applications.

- **Easy to Use**: With a simple and intuitive API, Rust MockData allows you to generate mock data with minimal effort, enabling you to focus on building your application.

## Installation

To add Rust MockData to your project, simply run the following command:

```bash
cargo add rust-mockdata
```

This will add the latest version of Rust MockData to your `Cargo.toml` file.

## Usage

Using Rust MockData is straightforward. First, create an instance of the `MockData` struct with a seed value. Then, use the provided methods to generate the desired mock data.

### Example

```rust
use rust_mockdata::MockData;
use std::collections::HashMap;

fn main() {
    let mut mock = MockData::new(12345);

    println!("Name: {}", mock.name());
    println!("Email: {}", mock.email());
    println!("Phone: {}", mock.phone());
    println!("Address: {}", mock.address());
    println!("Country: {}", mock.country());
    println!("Job Title: {}", mock.job_title());
    println!("Credit Card: {}", mock.credit_card());
    println!("Date: {}", mock.date());
    println!("Time: {}", mock.time());
    println!("UUID: {}", mock.uuid());
    println!("Bio: {}", mock.paragraph(3));

    let json_data: HashMap<String, String> = mock.json();
    println!("JSON Data: {:?}", json_data);
}
```

### Output

```
Name: Alice Smith
Email: xyz12345@example.com
Phone: +12-345-6789
Address: 123 MainSt, CityA
Country: USA
Job Title: Software Engineer
Credit Card: 1234-5678-9012-3456
Date: 2023-05-15
Time: 14:30:45
UUID: 1a2b3c4d-5e6f-7g8h-9i0j-1k2l3m4n5o6p
Bio: Lorem ipsum dolor sit amet. Consectetur adipiscing elit. Sed do eiusmod tempor incididunt.
JSON Data: {"uuid": "1a2b3c4d-5e6f-7g8h-9i0j-1k2l3m4n5o6p", "name": "Alice Smith", "email": "xyz12345@example.com", ...}
```

## Advanced Usage

### Customizing Data Generation

You can customize the generation of mock data by adjusting the parameters passed to the methods. For example, you can control the length of generated strings, the range of integers, or the number of sentences in a paragraph.

```rust
let mut mock = MockData::new(67890);

// Generate a string with a specific length
let custom_string = mock.string(20);

// Generate an integer within a specific range
let custom_integer = mock.integer(100, 200);

// Generate a paragraph with a specific number of sentences
let custom_paragraph = mock.paragraph(5);
```

### Reproducible Results

By using a fixed seed value, you can ensure that the generated mock data is reproducible. This is particularly useful for testing and debugging purposes.

```rust
let mut mock = MockData::new(42);

// The following calls will always produce the same results
let name1 = mock.name();
let name2 = mock.name();

assert_eq!(name1, name2);
```

## Contributing

Contributions to Rust MockData are welcome! If you have suggestions for new features, improvements, or bug fixes, please open an issue or submit a pull request on the [GitHub repository](https://github.com/linuxfanboy4/mockdata).

## License

Rust MockData is licensed under the MIT License. See the [LICENSE](https://github.com/linuxfanboy4/mockdata/blob/main/LICENSE) file for more details.

## Acknowledgments

Rust MockData is developed and maintained by Calestial Ashley. Special thanks to the Rust community for their support and contributions.

---

Rust MockData is the ultimate tool for generating realistic mock data in Rust. With its powerful features and ease of use, it is the perfect choice for developers looking to streamline their testing and prototyping workflows. Start using Rust MockData today and experience the difference!
