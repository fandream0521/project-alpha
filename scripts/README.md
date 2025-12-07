# Scripts for Ticket Management System

Quick setup scripts for the Ticket Management System.

## Quick Start

```bash
# 1. Set up environment
./scripts/setup.sh

# 2. Edit .env file
nano .env

# 3. Initialize database
./scripts/init-db.sh

# 4. Insert sample data
./scripts/seed-db.sh

# 5. Test everything
./scripts/test.sh
```

## Available Scripts

### `setup.sh`
Set up environment and create .env file.

### `init-db.sh`
Initialize database with required tables.

### `seed-db.sh`
Insert sample data for testing.

### `test.sh`
Test database connection and system functionality.

## SQL Scripts

### `create-tables.sql`
Database table creation script (used by init-db.sh).

## Configuration

All scripts automatically load environment variables from `.env` file.

Required environment variable:
- `DATABASE_URL`: PostgreSQL connection string

Example:
```
DATABASE_URL=postgresql://username:password@localhost:5432/ticket_db
```

## Troubleshooting

If scripts fail, check:
1. PostgreSQL is running: `pg_isready`
2. DATABASE_URL is correct in .env
3. Database exists and user has permissions

Manual connection test:
```bash
psql "$DATABASE_URL" -c "SELECT version();"
```

Database creation (if needed):
```bash
psql -U postgres -c "CREATE DATABASE ticket_db;"
```