# Secrets Directory

This directory contains template files for secrets used in the Docker Compose setup.

## Setup Instructions

1. Copy the template files and remove the `.template` extension:
   ```bash
   cp postgres_password.txt.template postgres_password.txt
   cp jwt_secret.txt.template jwt_secret.txt
   cp google_vision_api_key.txt.template google_vision_api_key.txt
   ```

2. Edit each file to contain only the secret value (no newlines or extra spaces):
   - `postgres_password.txt`: PostgreSQL database password
   - `jwt_secret.txt`: Secret key for JWT token generation
   - `google_vision_api_key.txt`: Google Vision API key (optional)

## Security Notes

- Never commit actual secret files to version control
- Add `secrets/*.txt` to your `.gitignore` file
- In production, consider using a more secure secret management system
- These files should be readable only by the user running Docker:
  ```bash
  chmod 600 secrets/*.txt
  ``` 