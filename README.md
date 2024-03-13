### Overview
The Project Management System is a robust Rust-based application engineered to simplify project and task administration for teams. It furnishes a suite of functionalities enabling the creation, modification, and deletion of projects, tasks, and user profiles. Additionally, it offers features for assigning tasks to users and supervising task statuses, fostering seamless collaboration and productivity within teams.

### Table of Contents
1. [Dependencies](#dependencies)
2. [Data Structures](#data-structures)
3. [Functions](#functions)
4. [Usage](#usage)
5. [Setting Up the Project](#setup)

### Dependencies <a name="dependencies"></a>
- **serde**: A serialization and deserialization library for Rust.
- **candid**: A library facilitating Candid serialization and deserialization.
- **ic_stable_structures**: A library providing stable data structures for the Internet Computer.
- **std**: The standard library for Rust.

### Data Structures <a name="data-structures"></a>
#### Structs
1. **Project**: Represents a project with attributes such as ID, name, description, start date, and due date.
2. **Task**: Represents a task with attributes including ID, project ID, name, description, start date, due date, status, and assigned users.
3. **User**: Represents a user with attributes including ID and name.
4. **TaskAssignment**: Represents the assignment of a task to a user.

#### Enums
1. **TaskStatus**: Enumerates the possible statuses for a task, including Todo, InProgress, and Done.

### Functions <a name="functions"></a>
The Project Management System encompasses a rich set of functions facilitating efficient project, task, and user management. Key functions include:
- `add_project`: Add a new project to the system.
- `delete_project`: Remove a project from the system.
- `add_task`: Append a new task to a project.
- `delete_task`: Eliminate a task from a project.
- `add_user`: Introduce a new user to the system.
- `delete_user`: Withdraw a user from the system.
- `assign_task_to_user`: Assign a task to a user.
- `unassign_task_from_user`: Revoke task assignment from a user.
- `get_project`: Retrieve details of a project by its ID.
- `get_task`: Retrieve details of a task by its ID.
- `update_project`: Modify details of a project.
- `update_task`: Modify details of a task.
- `change_task_status`: Alter the status of a task.
- `update_user`: Modify details of a user.

### Usage <a name="usage"></a>
The Project Management System presents an intuitive interface for team members to engage with the system seamlessly. Users can effortlessly create projects, assign tasks, monitor task statuses, and manage user profiles. Administrators have additional capabilities such as user administration and project oversight.

Users navigate through the interface, executing desired actions with ease, while the system ensures proper error handling to mitigate issues such as invalid inputs or missing data.

### Setting Up the Project <a name="setup"></a>
To commence work on the Project Management System, follow these steps:

1. **Install Rust and Dependencies**
   - Ensure you have Rust installed, version 1.64 or higher. You can install it using the following commands:
     ```bash
     $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
     $ source "$HOME/.cargo/env"
     ```
   - Install the `wasm32-unknown-unknown` target:
     ```bash
     $ rustup target add wasm32-unknown-unknown
     ```
   - Install `candid-extractor`:
     ```bash
     $ cargo install candid-extractor
     ```

2. **Install DFINITY SDK (`dfx`)**
   - Install `dfx` using the following commands:
     ```bash
     $ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
     $ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
     $ source ~/.bashrc
     $ dfx start --background
     ```

3. **Update Dependencies**
   - Update the `dependencies` block in `/src/{canister_name}/Cargo.toml` with the required dependencies.

4. **Autogenerate DID**
   - Add the provided script to the root directory of the project.
   - Update line 16 with the name of your canister.
   - Run the script each time you modify/add/remove exported functions of the canister.

5. **Running the Project Locally**
   - Start the replica, running in the background:
     ```bash
     $ dfx start --background
     ```
   - Deploy your canisters to the replica and generate your Candid interface:
     ```bash
     $ npm run gen-deploy
     ```