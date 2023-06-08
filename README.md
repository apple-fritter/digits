# digits
Digits ensures your data is ready for further analysis, machine learning tasks, or natural language processing (NLP) applications.

[digits.sh](https://github.com/apple-fritter/digits/blob/main/digits.sh) is a bash script that takes a path to a plaintext file as an argument at execution time. It parses the input file and generates an output file that is sanitized of non-numeric or punctuational characters. Additionally, it collapses repeated punctuational characters to only one instance instead of two or more.

In addition to the main script, a supplementary script, [digitalpha.sh](https://github.com/apple-fritter/digits/blob/main/digitalpha.sh), is also provided, offering the same functionality with an extended capability. It allows alphanumeric and punctuational characters to pass through the sanitization process. This versatile script complements the main script, providing flexibility for data cleaning and preprocessing tasks where alphanumeric information is essential.

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

## Use Cases

Digits can be useful in various scenarios, including:

- Cleaning up text files for further processing or analysis.
- Preparing text data for machine learning or natural language processing tasks that require standardized input.

---

## Flowchart

### [digits.sh](https://github.com/apple-fritter/digits/blob/main/digits.sh)
```
┌─ Start Program
│
├─ Check if an argument (file path) was provided
│   ├─ No argument provided:
│   │   └─ Display usage instructions
│   └─ Argument provided:
│       └─ Assign input and output file paths
│
├─ Check if input file exists
│   ├─ File does not exist:
│   │   └─ Display "File not found" message and exit
│   └─ File exists:
│       └─ Proceed to next step
│
├─ Sanitize the input file
│   └─ Remove non-numeric and non-punctuational characters
│       from the input file using `tr`
│
├─ Collapse repeated punctuations
│   └─ Remove consecutive duplicate lines using `awk`
│
├─ Save the sanitized output to a file
│   └─ Redirect the sanitized output to `sanitized.txt`
│
└─ End Program
```

### [digitalpha.sh](https://github.com/apple-fritter/digits/blob/main/digitalpha.sh)
```
┌─ Start Program
│
├─ Check if an argument (file path) was provided
│   ├─ No argument provided:
│   │   └─ Display usage instructions
│   └─ Argument provided:
│       └─ Assign input and output file paths
│
├─ Check if input file exists
│   ├─ File does not exist:
│   │   └─ Display "File not found" message and exit
│   └─ File exists:
│       └─ Proceed to next step
│
├─ Sanitize the input file
│   └─ Remove non-alphanumeric and non-punctuational characters
│       from the input file using `tr`
│
├─ Collapse repeated punctuations
│   └─ Remove consecutive duplicate lines using `awk`
│
├─ Save the sanitized output to a file
│   └─ Redirect the sanitized output to `sanitized.txt`
│
└─ End Program
```

### [digits.rs](https://github.com/apple-fritter/digits/blob/main/source/digits.rs)
```
┌─ Start Program
│
├─ Check if input arguments are provided
│   ├─ If no arguments provided:
│   │   └─ Display usage instructions
│   │      and exit program
│   └─ If arguments provided:
│       ├─ Assign input file path and output file name
│       └─ Proceed to next step
│
├─ Check if input file exists
│   ├─ If file does not exist:
│   │   └─ Display "File not found" message
│   │      and exit program
│   └─ If file exists:
│       └─ Proceed to sanitization step
│
├─ Sanitize the input file
│   ├─ Read the contents of the input file
│   ├─ Apply specified sanitization methods
│   │   ├─ Remove non-numeric characters if triggered
│   │   ├─ Remove non-alphabetic characters if triggered
│   │   ├─ Allow repeated punctuational characters if triggered
│   │   └─ Allow unicode characters if triggered
│   │
│   └─ Generate the sanitized output
│
├─ Save the sanitized output to a file
│   ├─ Create or overwrite the output file
│   ├─ Write the sanitized output to the file
│   └─ Display "Sanitized file created" message
│
└─ End Program
```

---

## Potential Concerns

Here are a few potential concerns to be aware of:

### Bash Scripts
The scripts assume that the input file is in plaintext format. If it contains binary or non-text data, the behavior may not be as expected.

### Rust Program

#### Performance Optimization:
Depending on the size of the input file, consider optimizing the program's performance. For example, you could process the file in chunks rather than loading the entire file into memory at once, which can be beneficial for large files.

#### Memory Usage:
The program loads the entire contents of each line into memory when sanitizing the file. If you're working with very long lines or large files, this could consume a significant amount of memory. Consider using buffered reading and writing for more efficient memory usage.

#### Security:
The program performs file I/O operations based on user-provided input file paths. It's important to ensure that the program has appropriate permissions and that user input is properly validated to prevent potential security vulnerabilities such as file path injection.

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

## License

These files released under the [MIT License](LICENSE).
