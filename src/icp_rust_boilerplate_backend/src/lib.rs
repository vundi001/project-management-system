#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define type aliases for memory management
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define the structure for a project
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Project {
    id: u64,
    name: String,
    description: String,
    start_date: u64, // Assuming start_date is a Unix timestamp
    due_date: u64,   // Assuming due_date is a Unix timestamp
}

// Define the structure for a task
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Task {
    id: u64,
    project_id: u64,
    name: String,
    description: String,
    start_date: u64, // Assuming start_date is a Unix timestamp
    due_date: u64,   // Assuming due_date is a Unix timestamp
    status: TaskStatus,
    assigned_users: Vec<u64>, // Stores user IDs assigned to this task
}

// Define the possible statuses for a task
#[derive(Debug, PartialEq, candid::CandidType, Deserialize, Serialize, Clone)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

// Define the structure for a user
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
    // Add any other relevant fields for the user
}

// Define the structure for a task assignment
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct TaskAssignment {
    user_id: u64,
    task_id: u64,
}

// Implement serialization and deserialization for Project
impl Storable for Project {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement bounds for Project serialization
impl BoundedStorable for Project {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement serialization and deserialization for Task
impl Storable for Task {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement bounds for Task serialization
impl BoundedStorable for Task {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement serialization and deserialization for User
impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement bounds for User serialization
impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement serialization and deserialization for TaskAssignment
impl Storable for TaskAssignment {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement bounds for TaskAssignment serialization
impl BoundedStorable for TaskAssignment {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Thread-local storage for memory management, ID counter, project storage, task storage, user storage, and task assignment storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static PROJECT_STORAGE: RefCell<StableBTreeMap<u64, Project, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static TASK_STORAGE: RefCell<StableBTreeMap<u64, Task, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static TASK_ASSIGNMENT_STORAGE: RefCell<StableBTreeMap<(u64, u64), TaskAssignment, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

// Define the possible errors
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

// Implement CRUD operations for projects
#[ic_cdk::update]
fn add_project(name: String, description: String, start_date: u64, due_date: u64) -> Result<Project, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let project = Project {
        id,
        name,
        description,
        start_date,
        due_date,
    };

    PROJECT_STORAGE.with(|storage| storage.borrow_mut().insert(id, project.clone()));
    Ok(project)
}

#[ic_cdk::update]
fn delete_project(id: u64) -> Result<(), Error> {
    match PROJECT_STORAGE.with(|storage| storage.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Project with id={} not found", id),
        }),
    }
}

// Implement CRUD operations for tasks
#[ic_cdk::update]
fn add_task(project_id: u64, name: String, description: String, start_date: u64, due_date: u64, assigned_users: Vec<u64>) -> Result<Task, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let task = Task {
        id,
        project_id,
        name,
        description,
        start_date,
        due_date,
        status: TaskStatus::Todo,
        assigned_users,
    };

    TASK_STORAGE.with(|storage| storage.borrow_mut().insert(id, task.clone()));
    Ok(task)
}

#[ic_cdk::update]
fn delete_task(id: u64) -> Result<(), Error> {
    match TASK_STORAGE.with(|storage| storage.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Task with id={} not found", id),
        }),
    }
}

// Implement CRUD operations for users
#[ic_cdk::update]
fn add_user(name: String) -> Result<User, Error> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment id counter");

    let user = User {
        id,
        name,
    };

    USER_STORAGE.with(|storage| storage.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

#[ic_cdk::update]
fn delete_user(id: u64) -> Result<(), Error> {
    match USER_STORAGE.with(|storage| storage.borrow_mut().remove(&id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("User with id={} not found", id),
        }),
    }
}

// Implement CRUD operations for task assignments
#[ic_cdk::update]
fn assign_task_to_user(task_id: u64, user_id: u64) -> Result<(), Error> {
    // Check if both task and user exist
    let task_exists = TASK_STORAGE.with(|storage| storage.borrow().contains_key(&task_id));
    let user_exists = USER_STORAGE.with(|storage| storage.borrow().contains_key(&user_id));

    if task_exists && user_exists {
        // Check if the task is already assigned to the user
        let assignment_exists = TASK_ASSIGNMENT_STORAGE.with(|storage| storage.borrow().contains_key(&(user_id, task_id)));

        if !assignment_exists {
            let assignment = TaskAssignment { user_id, task_id };
            TASK_ASSIGNMENT_STORAGE.with(|storage| storage.borrow_mut().insert((user_id, task_id), assignment));
            Ok(())
        } else {
            Err(Error::InvalidInput { msg: format!("Task with id={} is already assigned to user with id={}", task_id, user_id) })
        }
    } else {
        Err(Error::NotFound {
            msg: format!("Task with id={} or user with id={} not found", task_id, user_id),
        })
    }
}

#[ic_cdk::update]
fn unassign_task_from_user(task_id: u64, user_id: u64) -> Result<(), Error> {
    // Check if the task assignment exists
    match TASK_ASSIGNMENT_STORAGE.with(|storage| storage.borrow_mut().remove(&(user_id, task_id))) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Task with id={} is not assigned to user with id={}", task_id, user_id),
        }),
    }
}

// Implement query operations for the project management system
#[ic_cdk::query]
fn get_project(id: u64) -> Result<Project, Error> {
    match PROJECT_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(project) => Ok(project.clone()),
        None => Err(Error::NotFound {
            msg: format!("Project with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_task(id: u64) -> Result<Task, Error> {
    match TASK_STORAGE.with(|storage| storage.borrow().get(&id)) {
        Some(task) => Ok(task.clone()),
        None => Err(Error::NotFound {
            msg: format!("Task with id={} not found", id),
        }),
    }
}

// Implement update operation for projects
#[ic_cdk::update]
fn update_project(id: u64, name: String, description: String, start_date: u64, due_date: u64) -> Result<Project, Error> {
    match PROJECT_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(project) = storage.get(&id) {
            // Create a cloned copy of the project to update
            let mut updated_project = project.clone();
            // Update the project fields
            updated_project.name = name;
            updated_project.description = description;
            updated_project.start_date = start_date;
            updated_project.due_date = due_date;
            // Replace the old project with the updated one
            storage.insert(id, updated_project.clone());
            Ok(updated_project)
        } else {
            Err(Error::NotFound {
                msg: format!("Project with id={} not found", id),
            })
        }
    }) {
        Ok(project) => Ok(project),
        Err(e) => Err(e),
    }
}

// Implement update operation for tasks
#[ic_cdk::update]
fn update_task(id: u64, name: String, description: String, start_date: u64, due_date: u64, status: TaskStatus, assigned_users: Vec<u64>) -> Result<Task, Error> {
    match TASK_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(task) = storage.get(&id) {
            // Create a cloned copy of the task to update
            let mut updated_task = task.clone();
            // Update the task fields
            updated_task.name = name;
            updated_task.description = description;
            updated_task.start_date = start_date;
            updated_task.due_date = due_date;
            updated_task.status = status;
            updated_task.assigned_users = assigned_users;
            // Replace the old task with the updated one
            storage.insert(id, updated_task.clone());
            Ok(updated_task)
        } else {
            Err(Error::NotFound {
                msg: format!("Task with id={} not found", id),
            })
        }
    }) {
        Ok(task) => Ok(task),
        Err(e) => Err(e),
    }
}

// Implement operation to change task status
#[ic_cdk::update]
fn change_task_status(task_id: u64, status: TaskStatus, assigned_users: Vec<u64>) -> Result<Task, Error> {
    match TASK_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(task) = storage.get(&task_id) {
            // Create a cloned copy of the task to update
            let mut cloned_task = task.clone();
            // Update the status of the cloned task
            cloned_task.status = status;
            cloned_task.assigned_users = assigned_users;
            // Replace the old task with the updated one
            storage.insert(task_id, cloned_task.clone());
            Ok(cloned_task)
        } else {
            Err(Error::NotFound {
                msg: format!("Task with id={} not found", task_id),
            })
        }
    }) {
        Ok(task) => Ok(task),
        Err(e) => Err(e),
    }
}

#[ic_cdk::update]
fn update_user(id: u64, name: String) -> Result<User, Error> {
    match USER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(user) = storage.get(&id) {
            // Create a cloned copy of the user to update
            let mut updated_user = user.clone();
            // Update the user fields
            updated_user.name = name;
            // Replace the old user with the updated one
            storage.insert(id, updated_user.clone());
            Ok(updated_user)
        } else {
            Err(Error::NotFound {
                msg: format!("User with id={} not found", id),
            })
        }
    }) {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

// Export the Candid interface
ic_cdk::export_candid!();
