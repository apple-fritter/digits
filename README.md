# digits

[digits.sh](https://github.com/apple-fritter/digits/blob/main/digits.sh) is a bash script that takes a path to a plaintext file as an argument at execution time. It parses the input file and generates an output file that is sanitized of non-numeric or punctuational characters. Additionally, it collapses repeated punctuational characters to only one instance instead of two or more.

In addition to the main script, a supplementary script, [digitalpha.sh](https://github.com/apple-fritter/digits/blob/main/digitalpha.sh), is also provided, offering the same functionality with an extended capability. It allows alphanumeric and punctuational characters to pass through the sanitization process, ensuring that your text data retains both letters and special characters. This versatile script complements the main script, providing flexibility for data cleaning and preprocessing tasks where alphanumeric information is essential.

Finally, a [rust program](https://github.com/apple-fritter/digits/blob/main/source/digits.rs) has been put together to integrate into other projects, or to be used as a standalone tool. In addition to the features offered by the two bash scripts, the Rust program supports different triggers/options to allow specific types of characters during the sanitization process. The Rust program still collapses repeated punctuational marks to a single instance by default, but this behavior can be suppressed using the -p trigger.

---

## Usage

### Bash Scripts
To use these scripts, follow these steps:

1. Clone the repository or download the `digits.sh` script.
2. Open a terminal and navigate to the directory containing the `digits.sh` script.
3. Give execute permissions to the script using the command: `chmod +x digits.sh`
4. Run the script by providing the file path as an argument: `./digits.sh /path/to/input_file.txt`
5. The script will create a file named `digits.txt` containing the sanitized output.

### Rust program

To compile and run the Rust program, you can use the following commands:
```shell
$ rustc main.rs
$ ./digits <input_file> <output_file> [options]
```

Replace <input_file> and <output_file> with the actual paths to your input and output files, respectively.

#### Execution Time Arguments
- <input_file>: The path to the input file that needs to be sanitized.
- <output_file>: The path to the output file where the sanitized content will be stored.

The program collapses repeated punctuational marks to a single instance by default. Use the -p trigger to suppress this behavior.

#### Options:
```
-a  Allow alphabetic characters
-n  Allow numeric characters
-p  Allow punctuational characters
-u  Allow Unicode characters
--help  Show the help message
```

> If no triggers are used, the program will prompt for input during execution, or pressing Enter will cause the program to fail and exit.

---

## How it Works

The script follows these steps to sanitize the input file:

1. It checks if an argument (file path) was provided and displays usage instructions if no argument is provided.
2. The input file path and output file name are assigned.
3. It checks if the input file exists, displaying an error message if it doesn't.
4. The script removes non-numeric and non-punctuational characters from the input file using the `tr` command. It replaces any sequence of non-numeric and non-punctuational characters with a newline character.
5. It collapses repeated punctuations using the `awk` command. Only the first occurrence of each line is printed.
6. The sanitized output is saved to the `sanitized.txt` file.
7. A message is displayed indicating the creation of the sanitized file.

---

## Use Cases

The Sanitize Text script can be useful in various scenarios, including:

- Cleaning up text files containing non-numeric or non-punctuational characters for further processing or analysis.
- Preparing text data for machine learning or natural language processing tasks that require numeric or punctuational input.

---

## Flowchart

### digits.sh
```
┌─ Start Program
│
├─ Check if an argument (file path) was provided
│   ├─ Display usage instructions and exit if no argument provided
│   ├─ Assign input file path and output file name
│   └─ Check if input file exists
│
├─ Remove non-numeric and non-punctuational characters from the input file
│   └─ Use `tr` to replace non-numeric and non-punctuational characters with a newline character
│
├─ Collapse repeated punctuations
│   └─ Use `awk` to print only the first occurrence of each line
│
├─ Save the sanitized output to a file
│   └─ Redirect the sanitized output to the `sanitized.txt` file
│
└─ End Program
```

### digitalpha.sh
```
┌─ Start Program
│
├─ Check if an argument (file path) was provided
│   ├─ Display usage instructions and exit if no argument provided
│   ├─ Assign input file path and output file name
│   └─ Check if input file exists
│
├─ Remove non-numeric, non-punctuational, and non-alphabetical characters from the input file
│   └─ Use `tr` to replace non-alphanumeric and non-punctuational characters with a newline character
│
├─ Collapse repeated punctuations
│   └─ Use `awk` to print only the first occurrence of each line
│
├─ Save the sanitized output to a file
│   └─ Redirect the sanitized output to the `sanitized.txt` file
│
└─ End Program
```

### digits.rs
```
┌─ Start Program
│
├─ Check if input and output file paths were provided
│   ├─ Display usage instructions and exit if no arguments provided
│   ├─ Assign input and output file paths
│   └─ Check if input file exists
│
├─ Sanitize the input file
│   └─ Read the input file line by line
│       ├─ Apply sanitization rules to each line
│       │   └─ Check if characters should be allowed based on the provided triggers
│       └─ Store the sanitized lines
│
├─ Save the sanitized output to a file
│   └─ Write the sanitized lines to the output file
│
└─ End Program
```

---

## Potential Concerns

Here are a few potential concerns to be aware of:

### Bash Scripts
#### The script assumes that the input file is in plaintext format.
If it contains binary or non-text data, the behavior may not be as expected.

#### The script removes all non-numeric and non-punctuational characters.
This may result in loss of information if certain characters are needed for the desired task.

### Rust Program
#### Input Validation:
The program assumes that the provided input file path is valid and exists. It does not perform extensive validation on the input file. It's important to ensure that the input file exists and that the program has proper read permissions.

#### Error Handling:
While the program includes basic error handling using Rust's Result type, it may not cover all possible error scenarios. It's important to handle errors gracefully and provide appropriate error messages to the user.

#### Performance:
The program reads the input file line by line, which may be slower compared to processing the entire file at once. If you're working with large files, the program's performance could be a concern. Consider optimizing the program for large file processing if necessary.

#### Memory Usage:
The program loads the entire contents of each line into memory when sanitizing the file. If you're working with very long lines or large files, this could consume a significant amount of memory. Consider using buffered reading and writing for more efficient memory usage.

#### Security:
The program performs file I/O operations based on user-provided input file paths. It's important to ensure that the program has appropriate permissions and that user input is properly validated to prevent potential security vulnerabilities such as file path injection.

#### Compatibility:
The program relies on external command-line tools (`tr` and `awk`) to perform specific operations. It assumes these tools are available in the environment. Ensure that the necessary tools are installed and accessible on the system where the program will be executed.

#### Portability:
While Rust itself provides good portability across different platforms, the program's reliance on external command-line tools may limit its portability. Verify the availability of these tools on different platforms or consider alternative Rust-based approaches to achieve the desired functionality without relying on external tools.

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

## License

These files released under the [MIT License](LICENSE).
