# Employee Management System

This Rust program simulates a simple Employee Management System. It allows users to perform various operations such as adding employees, listing employees by department, listing all employees alphabetically, and exiting the system.

## How to Use

1. Clone this repository to your local machine.
2. Navigate to the project directory.
3. Run `cargo run` to compile and execute the program.
4. Follow the menu prompts to perform operations.

## Menu Options

- **Add Employee**: Allows the user to add a new employee by providing their name and department.
- **List Employees by Department**: Displays a list of employees grouped by department.
- **List All Employees (Alphabetical)**: Shows a list of all employees sorted alphabetically by name, including their respective departments.
- **Exit**: Quits the Employee Management System.

## Usage Example

```bash
$ cargo run

Employee Management System
1. Add Employee
2. List Employees by Department
3. List All Employees (Alphabetical)
4. Exit

$ 1

Enter employee name and department (e.g., Sally Engineering):
John Sales
Employee 'John' added to department 'SALES'.

Employee Management System
1. Add Employee
2. List Employees by Department
3. List All Employees (Alphabetical)
4. Exit

$ 2

List of Employees by Department:
Department: SALES
- John

Employee Management System
1. Add Employee
2. List Employees by Department
3. List All Employees (Alphabetical)
4. Exit

$ 3

List of All Employees (Alphabetical):
John (SALES)

Employee Management System
1. Add Employee
2. List Employees by Department
3. List All Employees (Alphabetical)
4. Exit

$ 4

Exiting Employee Management System...
```

## Logic

- The program uses a HashMap to store employees, where the key is the department name and the value is a vector of employees in that department.
- It provides a looped menu interface where users can select options to add employees, list employees by department, list all employees alphabetically, or exit the system.
- Adding an employee prompts the user to input the employee's name and department. It then adds the employee to the corresponding department in the HashMap.
- Listing employees by department displays employees grouped by their departments, with names sorted alphabetically within each department.
- Listing all employees alphabetically shows a sorted list of all employees along with their respective departments.