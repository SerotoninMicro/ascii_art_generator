ASCII Art Generator

Converts an input image into ASCII art representation.
Overview

This Rust-based application converts an input image into ASCII art, offering a unique way to visualize images using text characters. The program first converts the input image to grayscale, adjusts its contrast, resizes it, and then maps pixel intensities to ASCII characters based on their brightness. The resulting ASCII art can be saved to a text file for easy sharing and viewing.
Features

    Image Conversion: Supports various image formats (JPEG, PNG, etc.) for input.
    Contrast Adjustment: Allows users to adjust the contrast of the input image.
    Customizable Output: Users can customize the output ASCII art by adjusting the aspect ratio and ASCII character set.
    Output Saving: Saves the generated ASCII art to a text file for easy sharing and distribution.

Installation

Ensure you have Rust and Cargo installed. Clone the repository and run the following command to build the project:

bash

cargo build --release

Usage

Run the compiled binary with the path to your input image as an argument. For example:

```cargo run```

bash

./ascii_art_generator input.jpg

This will generate ASCII art from input.jpg and save it to output.txt.

You can adjust the contrast and output dimensions by modifying the constants in the source code.
Contributing

Contributions are welcome! If you'd like to contribute to this project, please fork the repository and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.
License

This project is licensed under the MIT License - see the LICENSE file for details.
Acknowledgements

This project utilizes the image crate for image processing in Rust.
