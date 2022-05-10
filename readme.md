
# Commands

Note: `<arg>` is a required argument, `[arg]` is optional

- `read <filename: str>`
  - Read a csv file and create a stream of data. It will attempt to parse each value if possible.
- `write <filename: str>`
  - Write the stream of data to file as csv
- `drop head <count: int>`
  - Drop `count` rows from the beginning (aka top of table)
- `print [header: str]`
  - Print out the current data stream. If header is specified, print the string before the data.
- `columns <order: tuple[int]>`
  - Reorder or drop columns. Columns are 1-indexed, so for example, if we have 3 columns and specify `(3,2,1)`, we reverse the order of columns.
- `parse <types: tuple[ident]>`
  - Parse strings to data types like integers, decimal numbers, or dates. Valid elements of `config` are: `int` for integers, `float` for decimal numbers, and `date` for dates.
- `classify <column: int> <rules: tuple[rule]>`
  - Classify the elements in specified column according to a given set of rules. If more than one rule apply, the first matching rule is used. The produced classification is added as a column to the right of the table (i.e. appended to each row). If no rule matches, it will use
- `filter <column: int> <match pattern>`
  - Only keeps rows where the element in the specified column matches the pattern
- `sum <column: int>`
  - Sum all the entries in the given columns. Strings will be appended, and if strings are numbers are encountered, the numbers will be turned into strings.


# Command Line Arguments

Command line arguments can be used anywhere where an argument is required. Simply put $n, where n
is the nth command line argument. The 0th argument is always the name of the script.

For example:
If `test.fluss` is 
```
read $1 >> drop head 2 >> print >> sum 3 >> print
```
then
```
cargo run -- test.fluss '"data.csv"'
```
will substitute the `$1` with `"data.csv"`.
Note: Command line arguments are parsed as arguments just like arguments in the script file. Bash removes the outer quotes and so `'"data.csv"'` means that the script parses `"data.csv"`, which it recognizes as a string. Without the extra quotes, it would try to interpret it as an command option like `head` instead of an arbitrary string.

# Syntax
## Match Rules
Rules follow the syntax: `<match pattern> => <value>`
- Value can be a string, int, or float
- If nothing matches, the default value of the empty string is used.

### Strings
- String matching is case-insensitive
- `"foo" => "This is foo"`
  - Matches `"foo"` exactly
- `"foo*" => "This is foo"`
  - Matches strings starting with `"foo"`
- `"*foo" => "This is foo"`
  - Matches strings ending with `"foo"`
- `"*foo*" => "This is foo"`
  - Matches strings containing `"foo"`
