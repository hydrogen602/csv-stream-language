
# Commands

Note: `<arg>` is a required argument, `[arg]` is optional

- `read <filename: str>`
  - Read a file and create a stream of data. It will attempt to parse each value if possible.
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
