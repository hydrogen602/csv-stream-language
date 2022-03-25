
# Commands

Note: `<arg>` is a required argument, `[arg]` is optional

- `read <filename: str>`
  - Read a file and create a stream of data
- `drop head <count: int>`
  - Drop `count` rows from the beginning (aka top of table)
- `print [header: str]`
  - Print out the current data stream. If header is specified, print the string before the data.
- `columns <config: tuple[int]>`
  - Reorder or drop columns. Columns are 1-indexed, so for example, if we have 3 columns and specify `(3,2,1)`, we reverse the order of columns.
- `parse <config: tuple[ident]>`
  - Parse strings to data types like integers, decimal numbers, or dates. Valid elements of `config` are: `int` for integers, `float` for decimal numbers, and `date` for dates.