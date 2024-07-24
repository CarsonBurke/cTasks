use std::{collections::HashMap};

use battery::units::{ElectricPotential, Energy, Power, ThermodynamicTemperature};
use sysinfo::{Disk, DiskKind, Pid, System};


#[derive(Debug)]
pub struct CpuData {
    pub cpu_usage_percent: f32,
    pub frequency: u64,
    pub logical_cores_usage_percents: Vec<f32>,
    pub logical_cores_frequencies: Vec<u64>,
}

impl CpuData {
    fn new() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            frequency: 0,
            logical_cores_usage_percents: vec![],
            logical_cores_frequencies: vec![],
        }
    }

    pub fn update(&mut self, cpu_info: &[sysinfo::Cpu], logical_core_count: u32) {
        let mut total_used: f32 = 0.;
        let mut total_frequency: u64 = 0;
        let mut logical_cores_usage_percents: Vec<f32> = Vec::new();
        let mut logical_cores_frequencies: Vec<u64> = Vec::new();

        for (_, logical_core) in cpu_info.iter().enumerate() {
            let cpu_usage = logical_core.cpu_usage();
            let frequency = logical_core.frequency();

            logical_cores_usage_percents.push(cpu_usage);

            total_frequency += frequency;
            logical_cores_frequencies.push(frequency);

            total_used += cpu_usage;
        }

        self.cpu_usage_percent = total_used / logical_core_count as f32;
        self.frequency = total_frequency / logical_core_count as u64;
        self.logical_cores_usage_percents = logical_cores_usage_percents;
        self.logical_cores_frequencies = logical_cores_frequencies;
    }
}

#[derive(Debug)]
pub struct DiskData {
    pub read: u64,
    pub written: u64,
    pub kind: DiskKind,
    pub name: String,
    pub space_total: u64,
    pub space_used: u64,
    pub in_depth: Option<DiskDataInDepth>,
}

impl DiskData {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            read: 0,
            written: 0,
            space_total: 0,
            space_used: 0,
            kind: DiskKind::Unknown(0),
            in_depth: Some(DiskDataInDepth {
                is_removable: false,
            }),
        }
    }

    pub fn update(&mut self, disk_name: &String, disk: &Disk) {
        self.name = disk_name.clone();
        self.space_total = disk.total_space();
        self.space_used = self.space_total - disk.available_space();
        self.read = 0;
        self.written = 0;
        self.kind = disk.kind();
    }

    pub fn update_in_depth(&mut self, disk_name: &String, disk: &Disk) {
        let in_depth = DiskDataInDepth {
            is_removable: disk.is_removable(),
        };

        self.in_depth = Some(in_depth);
    }
}

#[derive(Debug)]
pub struct DiskDataInDepth {
    pub is_removable: bool,
}

#[derive(Debug)]
pub struct BatteryData {
    pub index: String,
    pub vendor: String,
    pub model: String,
    /// Number of cycles the battery has gone through
    pub cycles: u32,
    pub temperature: ThermodynamicTemperature,
    pub energy_rate: Power,
    pub designed_capacity: Energy,
    pub current_capacity: Energy,
    pub energy: Energy,
    pub voltage: ElectricPotential,
    // Wether the batter is discharging, charging, empty, full or unknown
    pub state: battery::State,
    /// Time to either drain or reach capacity, depending on the delta
    pub time_to_behaviour: Option<battery::units::Time>,
    pub state_of_health: battery::units::Ratio,
    pub state_of_charge: battery::units::Ratio,
    pub technology: battery::Technology,
}

#[derive(Debug)]
pub struct ApplicationsData {
    pub applications_count: u32,
    pub applications: Option<HashMap<Pid, ApplicationData>>,
}

#[derive(Debug)]
pub struct ApplicationData {
    // Unsure about some of these properties
    pub name: String,
    pub path: String,
    pub pid: Pid,
    pub parent_pid: Pid,
    pub memory_usage: u64,
    pub cpu_usage: f32,
}

#[derive(Debug)]
pub struct MemoryData {
    pub ram_usage: u64,
    pub ram_total: u64,
    pub ram_usage_percent: f32,
    pub swap_usage: u64,
    pub swap_total: u64,
    pub swap_usage_percent: f32,
    pub in_depth: Option<MemoryDataInDepth>,
}

impl MemoryData {
    pub fn new() -> Self {
        Self {
            ram_usage: 0,
            ram_total: 0,
            ram_usage_percent: 0.,
            swap_usage: 0,
            swap_total: 0,
            swap_usage_percent: 0.,
            in_depth: Some(MemoryDataInDepth::new()),
        }
    }

    pub fn update(&mut self, system_info: &System) {
        // ram

        self.ram_usage = system_info.used_memory();
        self.ram_total = system_info.total_memory();

        self.ram_usage_percent = self.ram_usage as f32 / self.ram_total as f32 * 100.;

        // swap

        self.swap_usage = system_info.used_swap();
        self.swap_total = system_info.total_swap();

        self.swap_usage_percent = self.swap_usage as f32 / self.swap_total as f32 * 100.;
    }
}

#[derive(Debug)]
pub struct MemoryDataInDepth {
    pub is_removable: bool,
}

impl MemoryDataInDepth {
    fn new() -> Self {
        Self {
            is_removable: false,
        }
    }
}

#[derive(Debug)]
pub struct ResourceData {
    pub disks: HashMap<String, DiskData>,
    pub batteries: HashMap<String, BatteryData>,
    pub cpu: CpuData,
    pub memory: MemoryData,
}

impl ResourceData {
    pub fn new() -> Self {
        Self {
            disks: HashMap::new(),
            batteries: HashMap::new(),
            cpu: CpuData::new(),
            memory: MemoryData::new(),
        }
    }
}