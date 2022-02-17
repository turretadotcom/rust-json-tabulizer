# rust-json-tabulizer
A CLI Windows-based Rust program that transforms a JSON file to a text-based table form.

Built and tested using Rust 1.58.1.

TODO: Exception handling and unit tests

# Sample Usage:

```
Usage: [path to your program] [input file path] [output file path]
    /<home directory>/app/sample.json
    /<home directory>/app/result/sortbyid.table
```
  
# Sample JSON File Content

```
  [
  {
    "Id": "0005",
    "Type": "donut",
    "Name": "Cake",
    "Batter": "Chocolate",
    "Topping": "Powdered Sugar"
  },
  {
    "Id": "0002",
    "Type": "donut",
    "Name": "Cake",
    "Batter": "Chocolate",
    "Topping": "Powdered Sugar"
  }
]
```

# Sample Output

```
+------+-------+------+-----------+----------------+
| Id   | Type  | Name | Batter    | Topping        |
+------+-------+------+-----------+----------------+
| 0002 | donut | Cake | Chocolate | Powdered Sugar |
+------+-------+------+-----------+----------------+
| 0005 | donut | Cake | Chocolate | Powdered Sugar |
+------+-------+------+-----------+----------------+
```
