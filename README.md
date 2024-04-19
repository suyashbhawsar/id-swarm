# id-swarm

This repository contains the code for id-swarm, a decentralized identity management system developed as a final year project for the Masters of Engineering program. The system is designed to provide secure and efficient identity management using blockchain technology integrated with DevOps practices.

## Project Structure

- `backend/`: Contains the backend API code developed in Rust using the Actix-web framework.
- `frontend/`: Contains the frontend UI code developed in React.js.
- `smart_contracts/`: Contains the smart contract code developed using Substrate framework with Ink! for Rust-based smart contract development.

## Getting Started

### Prerequisites

- Rust programming language and Cargo package manager for backend and smart contracts.
- Node.js and npm (Node Package Manager) for frontend development.
- Substrate and Ink! for blockchain development.

### Installation

#### 1. Clone the repository:

```bash
git clone https://github.com/suyashbhawsar/id-swarm.git
```

#### 2. Install dependencies:

##### For backend:
```bash
cd backend
cargo build
```

##### For frontend:
```bash
cd frontend
npm install
```

For smart contracts:
```bash
cd smart_contracts
cargo build
```

For Substrate and Ink!:
```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```
### Usage
#### 1. Start the backend server:

```bash
cd backend
cargo run
```

#### 2. Start the frontend development server:
```bash
cd frontend
npm start
```

#### 3. [Access the application in your web browser](http://localhost:3000)