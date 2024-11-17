#!/bin/bash

# Load environment variables from .env file
source .env

# Drop the existing database
echo "Dropping the existing database..."
PGPASSWORD=$POSTGRES_PASSWORD psql -U $POSTGRES_USER -h $POSTGRES_HOST -c "DROP DATABASE IF EXISTS $POSTGRES_DB;"
if [ $? -ne 0 ]; then
    echo "Failed to drop the database."
    exit 1
fi

# Recreate the database
echo "Creating a new database..."
PGPASSWORD=$POSTGRES_PASSWORD psql -U $POSTGRES_USER -h $POSTGRES_HOST -c "CREATE DATABASE $POSTGRES_DB;"
if [ $? -ne 0 ]; then
    echo "Failed to create the database."
    exit 1
fi

# Apply the migrations
echo "Applying migrations..."
cargo run --manifest-path ./migration/Cargo.toml -- up
if [ $? -ne 0 ]; then
    echo "Failed to apply migrations."
    exit 1
fi

echo "Database reset and migrations applied successfully."