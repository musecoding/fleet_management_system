
# Intelligent Fleet Management System

This project is a decentralized platform built on the Internet Computer for managing drivers, vehicles, bookings, fuel consumption, maintenance, emergency assistance, and route optimization. It allows users to create and manage records for drivers, their vehicles, bookings, fuel consumption records, maintenance schedules, emergency assistance requests, and optimized routes.

## Key Features

### 1. Driver Management

- **Create Driver:** Allows users to create driver profiles.
- **Get All Drivers:** Retrieve a list of all drivers in the system.
- **Get Driver by ID:** Retrieve a specific driver's details by their ID.

### 2. Vehicle Management

- **Create Vehicle:** Allows users to add vehicles to the system.
- **Get All Vehicles:** Retrieve a list of all vehicles in the system.
- **Get Vehicle by ID:** Retrieve a specific vehicle's details by their ID.

### 3. Booking Management

- **Create Booking:** Allows users to create bookings for vehicles.
- **Get All Bookings:** Retrieve a list of all bookings in the system.
- **Get Booking by ID:** Retrieve a specific booking's details by its ID.

### 4. Fuel Consumption Management

- **Record Fuel Consumption:** Allows users to record fuel consumption for vehicles.
- **Get All Fuel Consumption Records:** Retrieve a list of all fuel consumption records in the system.
- **Get Fuel Consumption Record by ID:** Retrieve a specific fuel consumption record by its ID.

### 5. Maintenance Management

- **Schedule Maintenance:** Allows users to schedule maintenance for vehicles.
- **Get All Maintenance Records:** Retrieve a list of all maintenance records in the system.
- **Get Maintenance Record by ID:** Retrieve a specific maintenance record by its ID.

### 6. Emergency Assistance Management

- **Request Emergency Assistance:** Allows users to request emergency assistance for vehicles.
- **Get All Emergency Assistance Records:** Retrieve a list of all emergency assistance records in the system.
- **Get Emergency Assistance Record by ID:** Retrieve a specific emergency assistance record by its ID.

### 7. Route Optimization

- **Create Optimized Route:** Allows users to create optimized routes for journeys.
- **Get All Routes:** Retrieve a list of all optimized routes in the system.
- **Get Route by ID:** Retrieve a specific route's details by its ID.

## Error Handling

- **Not Found:** Returns an error if a requested item is not found.
- **Invalid Payload:** Returns an error if the provided data is invalid.
- **Unauthorized Access:** Returns an error if a user tries to perform an action without necessary permissions.

## Sample Payloads

### DriverPayload

```json
{
  "name": "John Doe",
  "license_number": "D1234567",
  "contact_info": "0734566787"
}
```

### VehiclePayload

```json
{
  "registration_number": "AB-12-CD-3456",
  "model": "Toyota Corolla",
  "capacity": 5,
  "location": "Nairobi"
}
```

### BookingPayload

```json
{
  "vehicle_id": 1,
  "driver_id": 1,
  "from_location": "Nairobi",
  "to_location": "Mombasa",
  "start_time": 1627849200,
  "end_time": 1627852800
}
```

### FuelConsumptionPayload

```json
{
  "vehicle_id": 1,
  "amount": 50.0,
  "date": 1627849200
}
```

### MaintenancePayload

```json
{
  "vehicle_id": 1,
  "description": "Oil change",
  "scheduled_date": 1627849200
}
```

### EmergencyAssistancePayload

```json
{
  "vehicle_id": 1,
  "description": "Flat tire",
  "location": "Highway A2"
}
```

### RoutePayload

```json
{
  "from_location": "Nairobi",
  "to_location": "Mombasa"
}
```

## Requirements

- rustc 1.64 or higher

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```

- rust wasm32-unknown-unknown target

```bash
$ rustup target add wasm32-unknown-unknown
```

- candid-extractor

```bash
$ cargo install candid-extractor
```

- install `dfx`

```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:

```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:

```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
