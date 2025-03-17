#!/bin/bash

# Script to set up secret files for Docker Compose

# Create secrets directory if it doesn't exist
mkdir -p secrets

# Copy template files if they don't exist
for template in secrets/*.txt.template; do
  if [ -f "$template" ]; then
    secret_file="${template%.template}"
    if [ ! -f "$secret_file" ]; then
      echo "Creating $secret_file from template..."
      cp "$template" "$secret_file"
      
      # If this is the Google Vision API key file and it contains the template value, mark it as optional
      if [[ "$secret_file" == *"google_vision_api_key.txt" ]] && grep -q "your_google_vision_api_key_here" "$secret_file"; then
        echo "NOTE: The Google Vision API key is set to a placeholder value."
        echo "      This is optional - without it, the app will use Tesseract OCR only."
      fi
    else
      echo "Secret file $secret_file already exists. Skipping..."
    fi
  fi
done

# Set proper permissions on secret files
chmod 600 secrets/*.txt 2>/dev/null || true

echo "Secret files setup complete. Please edit the secret files with your actual values."
echo "The following files were created or already existed:"
ls -la secrets/*.txt 2>/dev/null || echo "No secret files found."

echo ""
echo "Remember to update the values in these files with your actual secrets!"
echo "The Google Vision API key is optional - without it, the app will use Tesseract OCR only." 