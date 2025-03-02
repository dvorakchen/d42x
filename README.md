# d42x

# Usage

## Dev

Server:

```sh
cd ./backend/d42x-server
bun dev # if you don't have Bun, install it first
```

Admin Viewer:

```sh
cd ./admin
bun dev
```

## Build

Server:

```sh
cd ./backend/d42x-server
bun run build # if you don't have Bun, install it first
```

Admin Viewer:

```sh
cd ./admin
bun run build
```

Or Podman or Docker:

```sh
# launch server and database
podman-compose -f backend/compose.yaml up -d
# or use Docker

# launch admin viewer
podman-compose -f admin/compose.yaml up -d
# or use Docker
```
