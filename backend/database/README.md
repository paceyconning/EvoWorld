# Database Setup

This directory contains database-related files and setup instructions for EvoWorld.

## Quick Setup

1. **Run the setup script** (recommended):
   ```bash
   ./scripts/setup_database.sh
   ```

2. **Or set up manually**:
   ```bash
   # Create user and database
   psql -U postgres -h localhost -c "CREATE USER evoworld WITH PASSWORD 'evoworld_password';"
   psql -U postgres -h localhost -c "CREATE DATABASE evoworld OWNER evoworld;"
   psql -U postgres -h localhost -c "GRANT ALL PRIVILEGES ON DATABASE evoworld TO evoworld;"
   ```

## Configuration

The database configuration is defined in `config.toml` and can be overridden with environment variables:

- **Default**: Uses the configuration in `config.toml`
- **Environment Variable**: Set `DATABASE_URL` to override the connection string
- **Example**: `DATABASE_URL=postgresql://user:pass@host:port/db`

## Environment Variables

Copy `env.example` to `.env` and modify as needed:

```bash
cp env.example .env
# Edit .env with your preferred settings
```

## Database Schema

The database schema is automatically created when the application starts. The schema includes:

- `world_states` - Historical world state snapshots
- `events` - Simulation events and logs
- `analytics` - Performance and statistics data

## Troubleshooting

### Connection Issues
- Ensure PostgreSQL is running: `systemctl status postgresql`
- Check if the database exists: `psql -U evoworld -d evoworld -c "\l"`
- Verify credentials in `config.toml`

### Permission Issues
- Make sure the `evoworld` user has proper permissions
- Run: `psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE evoworld TO evoworld;"`

### Development vs Production
- **Development**: Uses local PostgreSQL with default credentials
- **Production**: Set `DATABASE_URL` environment variable with production credentials 