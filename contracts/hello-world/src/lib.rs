#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Vec, Address, symbol_short};

// Structure to store creator information and their royalty percentage
#[contracttype]
#[derive(Clone)]
pub struct Creator {
    pub address: Address,
    pub percentage: u64, // Percentage in basis points (e.g., 2500 = 25%)
}

// Structure to store project information
#[contracttype]
#[derive(Clone)]
pub struct Project {
    pub project_id: u64,
    pub title: String,
    pub description: String,
    pub creators: Vec<Creator>,
    pub total_royalties: u64,
    pub created_at: u64,
}

// Mapping project ID to Project
#[contracttype]
pub enum ProjectBook {
    Project(u64)
}

// Counter for generating unique project IDs
const PROJECT_COUNT: Symbol = symbol_short!("P_COUNT");

#[contract]
pub struct RoyaltyTrackerContract;

#[contractimpl]
impl RoyaltyTrackerContract {

    // Function to create a new project with multiple creators and their royalty splits
    pub fn create_project(
        env: Env, 
        title: String, 
        description: String, 
        creators: Vec<Creator>
    ) -> u64 {
        
        // Validate that total percentage equals 10000 (100%)
        let mut total_percentage: u64 = 0;
        for creator in creators.iter() {
            total_percentage += creator.percentage;
        }
        
        if total_percentage != 10000 {
            log!(&env, "Total percentage must equal 10000 (100%)");
            panic!("Invalid royalty split percentages");
        }

        // Get and increment project counter
        let mut project_count: u64 = env.storage().instance().get(&PROJECT_COUNT).unwrap_or(0);
        project_count += 1;

        let timestamp = env.ledger().timestamp();

        // Create new project
        let project = Project {
            project_id: project_count,
            title: title.clone(),
            description: description.clone(),
            creators: creators.clone(),
            total_royalties: 0,
            created_at: timestamp,
        };

        // Store project data
        env.storage().instance().set(&ProjectBook::Project(project_count), &project);
        env.storage().instance().set(&PROJECT_COUNT, &project_count);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Project Created with ID: {}", project_count);
        
        project_count
    }

    // Function to add royalties to a project
    pub fn add_royalties(env: Env, project_id: u64, amount: u64) {
        let mut project = Self::view_project(env.clone(), project_id);

        if project.project_id == 0 {
            log!(&env, "Project not found");
            panic!("Project does not exist");
        }

        project.total_royalties += amount;

        env.storage().instance().set(&ProjectBook::Project(project_id), &project);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Added {} royalties to Project ID: {}", amount, project_id);
    }

    // Function to view project details
    pub fn view_project(env: Env, project_id: u64) -> Project {
        let key = ProjectBook::Project(project_id);
        
        env.storage().instance().get(&key).unwrap_or(Project {
            project_id: 0,
            title: String::from_str(&env, "Not_Found"),
            description: String::from_str(&env, "Not_Found"),
            creators: Vec::new(&env),
            total_royalties: 0,
            created_at: 0,
        })
    }

    // Function to calculate royalty share for a specific creator
    pub fn calculate_creator_share(env: Env, project_id: u64, creator_address: Address) -> u64 {
        let project = Self::view_project(env.clone(), project_id);

        if project.project_id == 0 {
            log!(&env, "Project not found");
            return 0;
        }

        for creator in project.creators.iter() {
            if creator.address == creator_address {
                let share = (project.total_royalties * creator.percentage) / 10000;
                log!(&env, "Creator share: {}", share);
                return share;
            }
        }

        log!(&env, "Creator not found in project");
        0
    }
}