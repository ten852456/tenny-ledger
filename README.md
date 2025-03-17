# Tenny Ledger - Bill Scanning and Financial Tracking App

A modern web application for scanning, processing, and managing bill information to track personal finances.

## Features

- **Image Upload & Processing**: Upload bill images from your device and preview them before processing
- **OCR Text Extraction**: Extract text from bill images using high-performance Rust-based OCR
- **Transaction Management**: Store and manage extracted bill data as transactions
- **Reporting & Analytics**: Generate financial reports from transaction data
- **User Authentication**: Secure user accounts with NextAuth.js
- **Offline Capability**: Access your data even when offline using PWA features

## Architecture

This project uses a hybrid tech stack:

- **Frontend**: Next.js + React + TypeScript
- **Backend**: Rust with actix-web and OCR processing
- **Database**: PostgreSQL for data storage

## Getting Started

### Prerequisites

- Node.js (v14+)
- Rust (latest stable)
- PostgreSQL

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/ten852456/tenny-ledger.git
   cd tenny-ledger
   ```

2. Set up the frontend:
   ```
   cd frontend
   npm install
   ```

3. Set up the Rust backend:
   ```
   cd rust-backend
   cargo build
   ```

4. Create a `.env` file in the `rust-backend` directory:
   ```
   # Server settings
   HOST=127.0.0.1
   PORT=8080
   CORS_ORIGIN=http://localhost:3000

   # Database settings
   DATABASE_URL=postgres://username:password@localhost/tenny_ledger
   
   # JWT settings
   JWT_SECRET=your-secret-key
   ```

### Development

1. Start the frontend:
   ```
   cd frontend
   npm run dev
   ```

2. Start the backend:
   ```
   cd rust-backend
   cargo run
   ```

The frontend will be available at http://localhost:3000 and the backend at http://localhost:8080.

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/auth/register` | POST | Register new user |
| `/api/auth/login` | POST | User login |
| `/api/ocr/process` | POST | Process bill image using OCR |
| `/api/transactions` | GET | Get user transactions |
| `/api/transactions` | POST | Create new transaction |
| `/api/categories` | GET | Get transaction categories |

## License

MIT 