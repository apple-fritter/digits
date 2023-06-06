# digits

digits is a bash script that takes a path to a plaintext file as an argument at execution time. It parses the input file and generates an output file that is sanitized of non-numeric or punctuational characters. Additionally, it collapses repeated punctuational characters to only one instance instead of two or more.

---

## Usage

To use the script, follow these steps:

1. Clone the repository or download the `digits.sh` script.
2. Open a terminal and navigate to the directory containing the `digits.sh` script.
3. Give execute permissions to the script using the command: `chmod +x digits.sh`
4. Run the script by providing the file path as an argument: `./digits.sh /path/to/input_file.txt`
5. The script will create a file named `digits.txt` containing the sanitized output.

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

---

## Potential Concerns

Here are a few potential concerns to be aware of:

- The script assumes that the input file is in plaintext format. If it contains binary or non-text data, the behavior may not be as expected.
- The script removes all non-numeric and non-punctuational characters, which may result in loss of information if certain characters are needed for the desired task.
- It's important to ensure that the input file is properly backed up before running the script, as the original content will be modified.

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

## License

These files released under the [MIT License](LICENSE).
