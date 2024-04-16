# NFT LendSwap

NFT LendSwap is a decentralized NFT lending and borrowing platform built on Solana blockchain. It allows users to lend and borrow NFT assets securely using smart contracts. This repository contains the codebase for NFT LendSwap platform.

## Features

- **Decentralized NFT Lending & Borrowing:** Users can lend their NFT assets to earn interest or borrow NFTs by providing collateral.
- **Smart Contracts:** Implemented in Rust, ensuring security and transparency of transactions.
- **Next.js Frontend:** Frontend interface built using Next.js, providing a fast and responsive user experience.
- **Typescript:** Written in TypeScript for enhanced code maintainability and readability.
- **Tailwind CSS and Shadcn:** Utilizes Tailwind CSS and Shadcn for frontend styling and components, offering a sleek and modern design.

## Technologies Used

- **Solana:** Blockchain platform used for executing smart contracts and managing NFT assets.
- **Rust:** Programming language for writing smart contracts, ensuring efficiency and security.
- **Next.js:** React framework for building the frontend interface, providing server-side rendering and fast performance.
- **TypeScript:** Typed superset of JavaScript, enhancing code quality and maintainability.
- **Tailwind CSS:** Utility-first CSS framework for designing responsive and customizable UI components.
- **Shadcn:** Frontend library for UI components and styling, offering pre-designed components for rapid development.

## Getting Started

### Prerequisites

- Node.js
- npm or yarn
- Rust
- Solana CLI

### Installation

1. Clone the repository:

```bash
git clone https://github.com/ShivankK26/NFT-LendSwap.git
```

2. Install dependencies:

```bash
npm install   # or yarn install
```

3. Build and run the project:

```bash
anchor build
```

4. List the project deployment keys and copy the address to a clipboard:

```
anchor keys list
```

5. Update your Anchor.toml file, by using the address generated in the previous step:

```
[programs.devnet]
nft_lend_borrow = "<ADD YOUR ADDRESS HERE>"
```

6. Update your lib.rs file by adding the the address generated in step 4 to the declare_id!() macro

```
    // snip

pub use errors::ErrorCodes;
pub use instructions::*;
pub use states::*;

declare_id!("<YOUR ADDRESS HERE>");

#[program]
pub mod nft_lend_borrow {
    // code
```
