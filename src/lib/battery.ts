import { ref, computed, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event';

export enum BatteryBenchState  {
	STANDBY,
	CHARGE,
	DISCHARGE
}

export enum CompletionStatus {
	SUCCESS,
	FAIL,
	IN_PROGRESS
}

export interface BatteryBench {
	id: number;
	port: String;
	temperature: number;
	battery_temperature: number;
	electronic_load_temperature: number;
	voltage: number;
	current: number;
	state: BatteryBenchState;
	status: CompletionStatus;
	start_date: Date|null;
	end_date: Date|null;
}

export function useBatteryManager() {
	const batteries = ref<BatteryBench[]>([]);
	
	const batteries_voltages = computed(
		() => {
			let data: Map<String, number[]> = new Map<String, number[]>()
			batteries.value.map(battery => {
				
				!data.has(battery.port) && data.set(battery.port, []);
				data.get(battery.port)?.push(battery.voltage)

			});
			
			return data;
		})
	
	const batteries_temperatures = computed(
		() => {
			let data: Map<String, number[]> = new Map<String, number[]>()
			batteries.value.map(battery => {
				
				!data.has(battery.port) && data.set(battery.port, []);
				data.get(battery.port)?.push(battery.battery_temperature)

			});
			
			return data;
		})
	
	const batteries_currents = computed(
		() => {
			let data: Map<String, number[]> = new Map<String, number[]>()
			batteries.value.map(battery => {
				
				!data.has(battery.port) && data.set(battery.port, []);
				data.get(battery.port)?.push(battery.current)

			});
			
			return data;
		})
	
	const battery_benches_temperatures = computed(
		() => {
			let data: Map<String, number[]> = new Map<String, number[]>()
			batteries.value.map(battery => {
				
				!data.has(battery.port) && data.set(battery.port, []);
				data.get(battery.port)?.push(battery.temperature)

			});
			
			return data;
		})
	
	const bench_loads_temperatures = computed(
		() => {
			let data: Map<String, number[]> = new Map<String, number[]>()
			batteries.value.map(battery => {
				
				!data.has(battery.port) && data.set(battery.port, []);
				data.get(battery.port)?.push(battery.electronic_load_temperature)

			});
			
			return data;
		})
	
	onMounted(async () => {
		await listen('display-battery', event => {


			console.log(event.payload.port);
		});
	});

	return {
		batteries,
		batteries_voltages,
		batteries_currents,
		batteries_temperatures,
		battery_benches_temperatures,
		bench_loads_temperatures
	}
}
