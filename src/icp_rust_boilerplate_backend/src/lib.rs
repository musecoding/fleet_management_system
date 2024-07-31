#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Roles Enumeration
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum UserRole {
    #[default]
    User,
    Admin,
    Manager,
    Driver,
}

// Vehicle Status Enumeration
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum VehicleStatus {
    #[default]
    Available,
    Booked,
    Maintenance,
}

// Driver struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Driver {
    id: u64,
    name: String,
    license_number: String,
    contact_info: String,
    points: u64,
    created_at: u64,
}

// Vehicle struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Vehicle {
    id: u64,
    registration_number: String,
    model: String,
    capacity: u64,
    status: VehicleStatus,
    location: String,
    created_at: u64,
}

// Booking struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Booking {
    id: u64,
    vehicle_id: u64,
    // user_id: u64,
    driver_id: u64,
    from_location: String,
    to_location: String,
    start_time: u64,
    end_time: u64,
    status: String, // "pending", "approved", "completed"
    created_at: u64,
}

// Fuel Consumption struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FuelConsumption {
    id: u64,
    vehicle_id: u64,
    amount: f64,
    date: u64,
}

// Maintenance struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Maintenance {
    id: u64,
    vehicle_id: u64,
    description: String,
    scheduled_date: u64,
    status: String, // "pending", "completed"
    created_at: u64,
}

// Emergency Assistance struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EmergencyAssistance {
    id: u64,
    vehicle_id: u64,
    description: String,
    location: String,
    status: String, // "pending", "resolved"
    created_at: u64,
}

// Route struct for optimization
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Route {
    id: u64,
    from_location: String,
    to_location: String,
    optimized_route: String,
    distance: f64,
    time_estimate: u64,
}

// Implementing the Storable trait for the structs
impl Storable for Driver {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Driver {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Vehicle {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Vehicle {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Booking {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Booking {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for FuelConsumption {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FuelConsumption {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Maintenance {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Maintenance {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for EmergencyAssistance {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EmergencyAssistance {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Route {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Route {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Thread-local storage for memory managers and data structures
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static DRIVER_STORAGE: RefCell<StableBTreeMap<u64, Driver, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static VEHICLE_STORAGE: RefCell<StableBTreeMap<u64, Vehicle, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static BOOKING_STORAGE: RefCell<StableBTreeMap<u64, Booking, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static FUEL_CONSUMPTION_STORAGE: RefCell<StableBTreeMap<u64, FuelConsumption, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static MAINTENANCE_STORAGE: RefCell<StableBTreeMap<u64, Maintenance, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static EMERGENCY_ASSISTANCE_STORAGE: RefCell<StableBTreeMap<u64, EmergencyAssistance, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));

    static ROUTE_STORAGE: RefCell<StableBTreeMap<u64, Route, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
}

// Payload structs
#[derive(candid::CandidType, Deserialize, Serialize)]
struct DriverPayload {
    name: String,
    license_number: String,
    contact_info: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct VehiclePayload {
    registration_number: String,
    model: String,
    capacity: u64,
    location: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct BookingPayload {
    vehicle_id: u64,
    // user_id: u64,
    driver_id: u64,
    from_location: String,
    to_location: String,
    start_time: u64,
    end_time: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct FuelConsumptionPayload {
    vehicle_id: u64,
    amount: f64,
    date: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct MaintenancePayload {
    vehicle_id: u64,
    description: String,
    scheduled_date: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct EmergencyAssistancePayload {
    vehicle_id: u64,
    description: String,
    location: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct RoutePayload {
    from_location: String,
    to_location: String,
}

// Message enum
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Function to create a new driver
#[ic_cdk::update]
fn create_driver(payload: DriverPayload) -> Result<Driver, Message> {
    if payload.name.is_empty()
        || payload.license_number.is_empty()
        || payload.contact_info.is_empty()
    {
        return Err(Message::InvalidPayload(
            "Ensure 'name', 'license_number', and 'contact_info' are provided.".to_string(),
        ));
    }

    // Validate the driver's license number by checking if it exists in any vehicle
    let license_exists = VEHICLE_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, vehicle)| vehicle.registration_number == payload.license_number)
    });

    if !license_exists {
        return Err(Message::InvalidPayload(
            "Driver's license number does not exist in any vehicle.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let driver = Driver {
        id,
        name: payload.name,
        license_number: payload.license_number,
        contact_info: payload.contact_info,
        points: 0,
        created_at: current_time(),
    };

    DRIVER_STORAGE.with(|storage| storage.borrow_mut().insert(id, driver.clone()));
    Ok(driver)
}


// Function to get all drivers
#[ic_cdk::query]
fn get_drivers() -> Result<Vec<Driver>, Message> {
    DRIVER_STORAGE.with(|storage| {
        let drivers: Vec<Driver> = storage
            .borrow()
            .iter()
            .map(|(_, driver)| driver.clone())
            .collect();
        if drivers.is_empty() {
            Err(Message::NotFound("No drivers found".to_string()))
        } else {
            Ok(drivers)
        }
    })
}

// Function to get a driver by ID
#[ic_cdk::query]
fn get_driver_by_id(id: u64) -> Result<Driver, Message> {
    DRIVER_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .map(|driver| driver.clone())
            .ok_or(Message::NotFound("Driver not found".to_string()))
    })
}

// Function to create a new vehicle
#[ic_cdk::update]
fn create_vehicle(payload: VehiclePayload) -> Result<Vehicle, Message> {
    if payload.registration_number.is_empty()
        || payload.model.is_empty()
        || payload.capacity == 0
        || payload.location.is_empty()
    {
        return Err(Message::InvalidPayload(
            "Ensure 'registration_number', 'model', 'capacity', and 'location' are provided."
                .to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let vehicle = Vehicle {
        id,
        registration_number: payload.registration_number,
        model: payload.model,
        capacity: payload.capacity,
        status: VehicleStatus::Available,
        location: payload.location,
        created_at: current_time(),
    };

    VEHICLE_STORAGE.with(|storage| storage.borrow_mut().insert(id, vehicle.clone()));
    Ok(vehicle)
}

// Function to get all vehicles
#[ic_cdk::query]
fn get_vehicles() -> Result<Vec<Vehicle>, Message> {
    VEHICLE_STORAGE.with(|storage| {
        let vehicles: Vec<Vehicle> = storage
            .borrow()
            .iter()
            .map(|(_, vehicle)| vehicle.clone())
            .collect();
        if vehicles.is_empty() {
            Err(Message::NotFound("No vehicles found".to_string()))
        } else {
            Ok(vehicles)
        }
    })
}

// Function to get a vehicle by ID
#[ic_cdk::query]
fn get_vehicle_by_id(id: u64) -> Result<Vehicle, Message> {
    VEHICLE_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .map(|vehicle| vehicle.clone())
            .ok_or(Message::NotFound("Vehicle not found".to_string()))
    })
}

// Function to create a new booking
#[ic_cdk::update]
fn create_booking(payload: BookingPayload) -> Result<Booking, Message> {
    // Validate the booking payload
    if payload.from_location.is_empty() || payload.to_location.is_empty() {
        return Err(Message::InvalidPayload("Ensure 'from_location', 'to_location', and valid 'start_time' and 'end_time' are provided.".to_string()));
    }

    // Validate the driver ID
    let driver = DRIVER_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&payload.driver_id)
            .map(|driver| driver.clone())
    });

    if driver.is_none() {
        return Err(Message::NotFound("Driver not found".to_string()));
    }

    // Validate the vehicle ID
    let vehicle = VEHICLE_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&payload.vehicle_id)
            .map(|vehicle| vehicle.clone())
    });

    if vehicle.is_none() {
        return Err(Message::NotFound("Vehicle not found".to_string()));
    }

    let vehicle = vehicle.unwrap();
    if vehicle.status != VehicleStatus::Available {
        return Err(Message::InvalidPayload(
            "Vehicle is not available.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let booking = Booking {
        id,
        vehicle_id: payload.vehicle_id,
        driver_id: payload.driver_id,
        from_location: payload.from_location,
        to_location: payload.to_location,
        start_time: payload.start_time,
        end_time: payload.end_time,
        status: "pending".to_string(),
        created_at: current_time(),
    };

    // Update vehicle status to Booked
    let updated_vehicle = Vehicle {
        status: VehicleStatus::Booked,
        ..vehicle
    };
    VEHICLE_STORAGE.with(|storage| {
        storage
            .borrow_mut()
            .insert(vehicle.id, updated_vehicle.clone())
    });

    BOOKING_STORAGE.with(|storage| storage.borrow_mut().insert(id, booking.clone()));
    Ok(booking)
}

// Function to get all bookings
#[ic_cdk::query]
fn get_bookings() -> Result<Vec<Booking>, Message> {
    BOOKING_STORAGE.with(|storage| {
        let bookings: Vec<Booking> = storage
            .borrow()
            .iter()
            .map(|(_, booking)| booking.clone())
            .collect();
        if bookings.is_empty() {
            Err(Message::NotFound("No bookings found".to_string()))
        } else {
            Ok(bookings)
        }
    })
}

// Function to get a booking by ID
#[ic_cdk::query]
fn get_booking_by_id(id: u64) -> Result<Booking, Message> {
    BOOKING_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .map(|booking| booking.clone())
            .ok_or(Message::NotFound("Booking not found".to_string()))
    })
}

// Function to record fuel consumption
#[ic_cdk::update]
fn record_fuel_consumption(payload: FuelConsumptionPayload) -> Result<FuelConsumption, Message> {
    // Validate the fuel consumption payload
    if payload.amount <= 0.0 {
        return Err(Message::InvalidPayload(
            "Ensure 'amount' is greater than zero.".to_string(),
        ));
    }

    // Validate the vehicle ID
    let vehicle = VEHICLE_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&payload.vehicle_id)
            .map(|vehicle| vehicle.clone())
    });

    if vehicle.is_none() {
        return Err(Message::NotFound("Vehicle not found".to_string()));
    }

    // Update the fuel consumption record
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let fuel_consumption = FuelConsumption {
        id,
        vehicle_id: payload.vehicle_id,
        amount: payload.amount,
        date: payload.date,
    };

    FUEL_CONSUMPTION_STORAGE
        .with(|storage| storage.borrow_mut().insert(id, fuel_consumption.clone()));
    Ok(fuel_consumption)
}

// Function to get all fuel consumption records
#[ic_cdk::query]
fn get_fuel_consumptions() -> Result<Vec<FuelConsumption>, Message> {
    FUEL_CONSUMPTION_STORAGE.with(|storage| {
        let records: Vec<FuelConsumption> = storage
            .borrow()
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err(Message::NotFound(
                "No fuel consumption records found".to_string(),
            ))
        } else {
            Ok(records)
        }
    })
}

// Function to get a fuel consumption record by ID
#[ic_cdk::query]
fn get_fuel_consumption_by_id(id: u64) -> Result<FuelConsumption, Message> {
    FUEL_CONSUMPTION_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .map(|record| record.clone())
            .ok_or(Message::NotFound(
                "Fuel consumption record not found".to_string(),
            ))
    })
}

// Function to schedule maintenance
#[ic_cdk::update]
fn schedule_maintenance(payload: MaintenancePayload) -> Result<Maintenance, Message> {
    // Validate the maintenance payload
    if payload.description.is_empty() || payload.scheduled_date <= current_time() {
        return Err(Message::InvalidPayload(
            "Ensure 'description' and valid 'scheduled_date' are provided.".to_string(),
        ));
    }

    // Validate the vehicle ID
    let vehicle = VEHICLE_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&payload.vehicle_id)
            .map(|vehicle| vehicle.clone())
    });

    if vehicle.is_none() {
        return Err(Message::NotFound("Vehicle not found".to_string()));
    }

    // Update the maintenance record
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let maintenance = Maintenance {
        id,
        vehicle_id: payload.vehicle_id,
        description: payload.description,
        scheduled_date: payload.scheduled_date,
        status: "pending".to_string(),
        created_at: current_time(),
    };

    MAINTENANCE_STORAGE.with(|storage| storage.borrow_mut().insert(id, maintenance.clone()));
    Ok(maintenance)
}

// Function to get all maintenance records
#[ic_cdk::query]
fn get_maintenances() -> Result<Vec<Maintenance>, Message> {
    MAINTENANCE_STORAGE.with(|storage| {
        let records: Vec<Maintenance> = storage
            .borrow()
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err(Message::NotFound(
                "No maintenance records found".to_string(),
            ))
        } else {
            Ok(records)
        }
    })
}

// Function to get a maintenance record by ID
#[ic_cdk::query]
fn get_maintenance_by_id(id: u64) -> Result<Maintenance, Message> {
    MAINTENANCE_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .map(|record| record.clone())
            .ok_or(Message::NotFound(
                "Maintenance record not found".to_string(),
            ))
    })
}

// Function to request emergency assistance
#[ic_cdk::update]
fn request_emergency_assistance(
    payload: EmergencyAssistancePayload,
) -> Result<EmergencyAssistance, Message> {
    // Validate the emergency assistance payload
    if payload.description.is_empty() || payload.location.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'description' and 'location' are provided.".to_string(),
        ));
    }

    // Validate the vehicle ID
    let vehicle = VEHICLE_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&payload.vehicle_id)
            .map(|vehicle| vehicle.clone())
    });

    if vehicle.is_none() {
        return Err(Message::NotFound("Vehicle not found".to_string()));
    }

    // Update the emergency assistance record
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let assistance = EmergencyAssistance {
        id,
        vehicle_id: payload.vehicle_id,
        description: payload.description,
        location: payload.location,
        status: "pending".to_string(),
        created_at: current_time(),
    };

    EMERGENCY_ASSISTANCE_STORAGE
        .with(|storage| storage.borrow_mut().insert(id, assistance.clone()));
    Ok(assistance)
}

// Function to get all emergency assistance records
#[ic_cdk::query]
fn get_emergency_assistances() -> Result<Vec<EmergencyAssistance>, Message> {
    EMERGENCY_ASSISTANCE_STORAGE.with(|storage| {
        let records: Vec<EmergencyAssistance> = storage
            .borrow()
            .iter()
            .map(|(_, record)| record.clone())
            .collect();
        if records.is_empty() {
            Err(Message::NotFound(
                "No emergency assistance records found".to_string(),
            ))
        } else {
            Ok(records)
        }
    })
}

// Function to get an emergency assistance record by ID
#[ic_cdk::query]
fn get_emergency_assistance_by_id(id: u64) -> Result<EmergencyAssistance, Message> {
    EMERGENCY_ASSISTANCE_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .map(|record| record.clone())
            .ok_or(Message::NotFound(
                "Emergency assistance record not found".to_string(),
            ))
    })
}

// Function to create an optimized route
#[ic_cdk::update]
fn create_route(payload: RoutePayload) -> Result<Route, Message> {
    // Validate the route payload
    if payload.from_location.is_empty() || payload.to_location.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'from_location' and 'to_location' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    // Dummy data for route optimization
    let optimized_route = format!(
        "Optimized route from {} to {}",
        payload.from_location, payload.to_location
    );
    let distance = 100.0; // Dummy distance
    let time_estimate = 3600; // Dummy time estimate in seconds

    let route = Route {
        id,
        from_location: payload.from_location,
        to_location: payload.to_location,
        optimized_route,
        distance,
        time_estimate,
    };

    ROUTE_STORAGE.with(|storage| storage.borrow_mut().insert(id, route.clone()));
    Ok(route)
}

// Function to get all routes
#[ic_cdk::query]
fn get_routes() -> Result<Vec<Route>, Message> {
    ROUTE_STORAGE.with(|storage| {
        let routes: Vec<Route> = storage
            .borrow()
            .iter()
            .map(|(_, route)| route.clone())
            .collect();
        if routes.is_empty() {
            Err(Message::NotFound("No routes found".to_string()))
        } else {
            Ok(routes)
        }
    })
}

// Function to get a route by ID
#[ic_cdk::query]
fn get_route_by_id(id: u64) -> Result<Route, Message> {
    ROUTE_STORAGE.with(|storage| {
        storage
            .borrow()
            .get(&id)
            .map(|route| route.clone())
            .ok_or(Message::NotFound("Route not found".to_string()))
    })
}

// Helper function to get the current time
fn current_time() -> u64 {
    time()
}

ic_cdk::export_candid!();
