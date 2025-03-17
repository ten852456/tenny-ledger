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
- Docker & Docker Compose (optional, for containerized setup)

### Installation

#### Option 1: Local Setup

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
   
   # OCR settings - Optional
   # Add this if you want to use Google Vision API for enhanced OCR
   # GOOGLE_VISION_API_KEY=your-google-vision-api-key
   ```

5. Initialize the database and load fixtures:
   ```
   npm run init-db
   ```

#### Option 2: Docker Setup

1. Clone the repository:
   ```
   git clone https://github.com/ten852456/tenny-ledger.git
   cd tenny-ledger
   ```

2. Build and start the Docker containers:
   ```
   docker-compose up -d
   ```

3. Initialize the database with fixtures:
   ```
   docker-compose exec backend tenny-ledger load-fixtures
   ```

The application will be available at:
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080

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

## Project Structure

### Frontend (Next.js)

- `/frontend/pages` - Next.js pages and routing
- `/frontend/components` - React components
- `/frontend/hooks` - Custom React hooks
- `/frontend/services` - API service interfaces
- `/frontend/styles` - Global styles and Tailwind config
- `/frontend/public` - Static assets
- `/frontend/pages/api/proxy` - API proxy to avoid CORS issues

### Backend (Rust)

- `/rust-backend/src/handlers` - API endpoint handlers
- `/rust-backend/src/models` - Database models
- `/rust-backend/src/ocr` - OCR processing logic
- `/rust-backend/src/fixtures` - Database fixtures for testing/development
- `/rust-backend/migrations` - Database migrations

### Docker Setup

- `Dockerfile.frontend` - Frontend container configuration
- `Dockerfile.backend` - Backend container configuration
- `docker-compose.yml` - Multi-container Docker configuration

## License

MIT 

### API Keys and Security

The application uses a secure configuration module to handle sensitive information like API keys:

1. **Google Vision API (Optional)**: For enhanced OCR processing, you can configure a Google Vision API key. The application will fall back to local Tesseract OCR if no API key is provided.

2. **Secure API Key Handling**:
   - API keys are stored securely and never exposed in logs or error messages
   - Keys are accessed through a secure configuration module
   - The app automatically detects which OCR engines are available based on configuration

3. **File Upload Security**:
   - File types are validated to only allow image formats (JPG, PNG, PDF)
   - File size is limited to prevent abuse (10MB max)
   - Temporary files are automatically cleaned up after processing

4. **Production Deployment**:
   - In production, it's recommended to use environment variables instead of .env files
   - For Docker deployments, use Docker secrets or environment variables in docker-compose.yml
   - Never commit API keys or sensitive data to version control 