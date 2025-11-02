# Royalty Tracker

## Project Title
**Royalty Tracker - Decentralized Royalty Distribution Platform**

CONTRACT DETAILS- CC7OATDNEPX7QWU24PWDZGN6IF4RRBZDBW6OFFQIBZD5WU5ZNY57VL2O
![WhatsApp Image 2025-11-02 at 16 22 06](https://github.com/user-attachments/assets/f01c1c52-c214-48bf-9edf-67b44e153caa)


## Project Description
Royalty Tracker is a decentralized platform built on the Stellar blockchain using Soroban smart contracts. It enables multiple creators (musicians, podcasters, YouTubers, writers, and other content creators) to collaboratively upload projects and automatically split royalties based on predefined percentages. 

The platform eliminates traditional intermediaries, ensuring transparent, immutable, and trustless royalty distribution. All transactions and splits are recorded on-chain, providing complete transparency and automatic execution without requiring manual intervention or third-party oversight.

## Project Vision
Our vision is to revolutionize the creator economy by:

- **Empowering Creators**: Giving artists and content creators full control over their royalty agreements without relying on traditional gatekeepers
- **Ensuring Transparency**: Making every transaction and royalty split visible and verifiable on the blockchain
- **Automating Payments**: Eliminating delays and disputes through smart contract automation
- **Reducing Costs**: Removing middlemen and their associated fees, ensuring creators receive maximum value
- **Building Trust**: Creating an immutable record of agreements that all parties can trust

We believe that creators deserve a fair, transparent, and efficient system for managing their collaborative works and receiving their rightful share of revenue.

## Key Features

### 1. **Multi-Creator Project Management**
- Create projects with multiple collaborators
- Define each creator's contribution percentage upfront
- Support for any number of creators per project

### 2. **Transparent Royalty Splits**
- Predefined percentage allocations (stored in basis points for precision)
- Automatic validation ensuring splits equal 100%
- Immutable agreements stored on-chain

### 3. **Automated Royalty Distribution**
- Real-time royalty calculation for each creator
- Automatic distribution based on predefined percentages
- No manual intervention required

### 4. **On-Chain Transparency**
- All project details stored on blockchain
- Complete audit trail of royalty additions
- Public verification of creator shares

### 5. **Decentralized Architecture**
- No central authority controlling distributions
- Trustless execution via smart contracts
- Resistant to censorship and manipulation

## Future Scope

### Phase 1: Enhanced Functionality
- **Withdrawal System**: Implement direct withdrawal functions for creators to claim their royalties
- **Multi-Token Support**: Enable royalty payments in various cryptocurrencies and tokens
- **Batch Payments**: Allow bulk royalty additions for multiple projects simultaneously
- **Project Updates**: Enable modification of creator percentages with multi-signature approval

### Phase 2: Advanced Features
- **Time-Based Vesting**: Introduce vesting schedules for royalty releases
- **Conditional Royalties**: Implement milestone-based or performance-triggered distributions
- **Sub-Splits**: Allow creators to further split their share with team members
- **NFT Integration**: Link projects to NFTs for proof of ownership and additional revenue streams

### Phase 3: Platform Expansion
- **Creator Profiles**: Build on-chain reputation and portfolio systems
- **Dispute Resolution**: Implement decentralized arbitration for conflicts
- **Analytics Dashboard**: Provide comprehensive insights on earnings and project performance
- **Cross-Chain Compatibility**: Expand to other blockchain networks for broader adoption

### Phase 4: Ecosystem Development
- **DAO Governance**: Community-driven decision making for platform upgrades
- **Creator Marketplace**: Enable trading of royalty shares as financial instruments
- **Integration APIs**: Connect with streaming platforms, NFT marketplaces, and content distribution networks
- **Mobile Application**: Native mobile apps for iOS and Android with easy access

### Long-Term Vision
- Establish industry standards for decentralized royalty management
- Partner with major content platforms for seamless integration
- Create a global network of verified creators and projects
- Build educational resources and tools for creator onboarding

---

## Smart Contract Functions

### `create_project`
Creates a new project with multiple creators and their royalty percentages.
- **Parameters**: title, description, creators (vector of Creator structs)
- **Returns**: project_id

### `add_royalties`
Adds royalty payments to an existing project.
- **Parameters**: project_id, amount
- **Returns**: void

### `view_project`
Retrieves complete project information.
- **Parameters**: project_id
- **Returns**: Project struct

### `calculate_creator_share`
Calculates the royalty share for a specific creator.
- **Parameters**: project_id, creator_address
- **Returns**: creator's share amount

---

## Getting Started

### Prerequisites
- Rust installed on your system
- Soroban CLI tools
- Stellar account for deployment

### Installation
```bash
# Clone the repository
git clone https://github.com/yourusername/royalty-tracker

# Build the contract
soroban contract build

# Run tests
cargo test
```

### Deployment
```bash
# Deploy to Stellar testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/royalty_tracker.wasm \
  --source YOUR_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

---

## Contributing
We welcome contributions from the community! Please read our contributing guidelines and submit pull requests for any enhancements.

