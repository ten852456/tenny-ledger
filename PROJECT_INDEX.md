# Tenny Ledger Project Index

## Project Overview

Tenny Ledger is a bill scanning and financial tracking application built with Next.js for the frontend and Rust for the backend. The application allows users to upload bill images, extract data using OCR, manage transactions, and generate financial reports.

## Directory Structure

```
tenny-ledger/
├── frontend/             # Next.js frontend application
│   ├── components/       # React components
│   ├── contexts/         # React context providers
│   ├── hooks/            # Custom React hooks
│   ├── pages/            # Next.js pages and API routes
│   │   ├── api/          # API routes including proxy
│   │   │   └── proxy/    # API proxy middleware
│   │   ├── admin/        # Admin panel pages
│   │   └── transactions/ # Transaction management pages
│   ├── public/           # Static assets
│   ├── services/         # API service interfaces
│   ├── styles/           # CSS and styling
│   ├── types/            # TypeScript type definitions
│   ├── utils/            # Utility functions
│   ├── .env.local        # Environment variables
│   ├── next.config.js    # Next.js configuration
│   ├── package.json      # Frontend dependencies
│   └── tailwind.config.js # Tailwind CSS configuration
│
├── rust-backend/         # Rust backend application
│   ├── src/              # Source code
│   │   ├── fixtures/     # Database fixtures
│   │   ├── handlers/     # API endpoint handlers
│   │   ├── models/       # Database models
│   │   ├── ocr/          # OCR processing logic
│   │   ├── services/     # Business logic services
│   │   ├── utils/        # Utility functions
│   │   ├── config.rs     # Application configuration
│   │   ├── db.rs         # Database connection
│   │   ├── error.rs      # Error handling
│   │   ├── main.rs       # Application entry point
│   │   └── routes.rs     # API route definitions
│   ├── migrations/       # Database migrations
│   ├── tests/            # Automated tests
│   ├── .env              # Environment variables
│   └── Cargo.toml        # Rust dependencies
│
├── scripts/              # Utility scripts
│   └── init-db.js        # Database initialization script
│
├── Dockerfile.frontend   # Frontend Docker configuration
├── Dockerfile.backend    # Backend Docker configuration
├── docker-compose.yml    # Docker Compose configuration
└── README.md             # Project documentation
```

## Key Components

### Frontend Components

- **Authentication**
  - Login and Registration pages
  - Auth context for state management

- **Dashboard**
  - Summary statistics
  - Recent transactions
  - Quick actions

- **Transactions**
  - Transaction list and filtering
  - Transaction details view
  - Transaction form for creating/editing

- **Image Upload & OCR**
  - Image uploader component
  - Bill preview
  - OCR result review and editing

- **Reports**
  - Spending by category
  - Monthly spending trends
  - Data visualization

- **Admin Panel**
  - User management
  - System settings
  - Data management

### Backend Components

- **API Layer**
  - RESTful endpoints with Actix-web
  - JWT authentication
  - Error handling

- **OCR Processing**
  - Image preprocessing
  - Text extraction with Tesseract
  - Data parsing and extraction

- **Database**
  - PostgreSQL with Diesel ORM
  - Migrations and schema management
  - Fixtures for development/testing

- **Services**
  - User management
  - Transaction processing
  - Category management

## Dockerization

The project is containerized using Docker to provide a consistent runtime environment:

- `Dockerfile.frontend` - Multi-stage build for Next.js frontend
- `Dockerfile.backend` - Multi-stage build for Rust backend with OCR dependencies
- `docker-compose.yml` - Orchestration for frontend, backend, and PostgreSQL

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - User login
- `GET /api/auth/me` - Get current user information

### OCR
- `POST /api/ocr/process` - Process bill image using OCR

### Transactions
- `GET /api/transactions` - List transactions with optional filtering
- `GET /api/transactions/:id` - Get transaction details
- `POST /api/transactions` - Create new transaction
- `PUT /api/transactions/:id` - Update transaction
- `DELETE /api/transactions/:id` - Delete transaction

### Categories
- `GET /api/categories` - List categories
- `POST /api/categories` - Create category
- `PUT /api/categories/:id` - Update category
- `DELETE /api/categories/:id` - Delete category

### Reports
- `GET /api/reports/spending-by-category` - Get spending by category
- `GET /api/reports/monthly-spending` - Get monthly spending data
- `GET /api/reports/transaction-trends` - Get transaction trends

## Development

### Local Development

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

### Docker Development

1. Build and start all containers:
   ```
   docker-compose up -d
   ```

2. Initialize database with test data:
   ```
   docker-compose exec backend tenny-ledger load-fixtures
   ```

3. Access logs:
   ```
   docker-compose logs -f
   ```

## Dependencies

### Frontend Dependencies
- Next.js for React framework
- SWR for data fetching
- Tailwind CSS for styling
- Axios for API requests
- Next-Auth for authentication
- Chart.js for data visualization

### Backend Dependencies
- Actix-web for web framework
- Diesel for ORM
- Serde for serialization
- Leptess/Tesseract for OCR
- JWT for authentication
- Regex for text parsing 