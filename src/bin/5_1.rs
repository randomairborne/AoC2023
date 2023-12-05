fn main() {
    let lines: Vec<String> = std::fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(ToString::to_string)
        .collect();
    let seeds: Vec<usize> = lines[0]
        .trim_start_matches("seeds: ")
        .split(' ')
        .map(|v| v.parse().unwrap())
        .collect();
    let mut first = true;
    let mut maps = Maps::default();
    let mut current_map: &mut Vec<MapEntry> = &mut Vec::new();
    for line in lines {
        if first || line.trim().is_empty() {
            first = false;
            continue;
        }
        if let Some(mapkind) = MapKind::from_str(&line) {
            current_map = maps.mut_ref_from_kind(mapkind);
            continue;
        }
        let map_values: Vec<usize> = line.split(' ').map(|v| v.parse().unwrap()).collect();
        let entry = MapEntry {
            dest_start: map_values[0],
            src_start: map_values[1],
            range: map_values[2],
        };
        current_map.push(entry);
    }
    let min = seeds
        .iter()
        .map(|seed| maps.seed_to_location(*seed))
        .min()
        .unwrap();
    println!("{min}");
}

struct MapEntry {
    dest_start: usize,
    src_start: usize,
    range: usize,
}

#[derive(Default)]
struct Maps {
    seed_to_soil: Vec<MapEntry>,
    soil_to_fertilizer: Vec<MapEntry>,
    fertilizer_to_water: Vec<MapEntry>,
    water_to_light: Vec<MapEntry>,
    light_to_tempature: Vec<MapEntry>,
    tempature_to_humidity: Vec<MapEntry>,
    humidity_to_location: Vec<MapEntry>,
}

impl Maps {
    fn mut_ref_from_kind(&mut self, kind: MapKind) -> &mut Vec<MapEntry> {
        match kind {
            MapKind::SeedToSoil => &mut self.seed_to_soil,
            MapKind::SoilToFertilizer => &mut self.soil_to_fertilizer,
            MapKind::FertilizerToWater => &mut self.fertilizer_to_water,
            MapKind::WaterToLight => &mut self.water_to_light,
            MapKind::LightToTempature => &mut self.light_to_tempature,
            MapKind::TempatureToHumidity => &mut self.tempature_to_humidity,
            MapKind::HumidityToLocation => &mut self.humidity_to_location,
        }
    }
    fn ref_from_kind(&self, kind: MapKind) -> &Vec<MapEntry> {
        match kind {
            MapKind::SeedToSoil => &self.seed_to_soil,
            MapKind::SoilToFertilizer => &self.soil_to_fertilizer,
            MapKind::FertilizerToWater => &self.fertilizer_to_water,
            MapKind::WaterToLight => &self.water_to_light,
            MapKind::LightToTempature => &self.light_to_tempature,
            MapKind::TempatureToHumidity => &self.tempature_to_humidity,
            MapKind::HumidityToLocation => &self.humidity_to_location,
        }
    }
    fn seed_to_location(&self, seed: usize) -> usize {
        let soil = self.layer(seed, MapKind::SeedToSoil);
        let fertilizer = self.layer(soil, MapKind::SoilToFertilizer);
        let water = self.layer(fertilizer, MapKind::FertilizerToWater);
        let light = self.layer(water, MapKind::WaterToLight);
        let temp = self.layer(light, MapKind::LightToTempature);
        let humidity = self.layer(temp, MapKind::TempatureToHumidity);
        self.layer(humidity, MapKind::HumidityToLocation)
    }
    fn layer(&self, prev: usize, kind: MapKind) -> usize {
        for item in self.ref_from_kind(kind) {
            if (item.src_start..item.src_start + item.range).contains(&prev) {
                return item.dest_start + (prev - item.src_start);
            }
        }
        prev
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum MapKind {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTempature,
    TempatureToHumidity,
    HumidityToLocation,
}

impl MapKind {
    fn from_str(input: &str) -> Option<Self> {
        let out = match input {
            "seed-to-soil map:" => Self::SeedToSoil,
            "soil-to-fertilizer map:" => Self::SoilToFertilizer,
            "fertilizer-to-water map:" => Self::FertilizerToWater,
            "water-to-light map:" => Self::WaterToLight,
            "light-to-temperature map:" => Self::LightToTempature,
            "temperature-to-humidity map:" => Self::TempatureToHumidity,
            "humidity-to-location map:" => Self::HumidityToLocation,
            _ => return None,
        };
        Some(out)
    }
}
