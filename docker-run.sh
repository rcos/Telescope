# Wait for the database to come online
set -e

until nc -z db 5432; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

diesel setup
diesel migration run
cargo run --release
