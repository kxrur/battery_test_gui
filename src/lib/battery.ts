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
		() => batteries.value.map(battery => battery.voltage)
	)
	
	const batteries_temperatures = computed(
		() => batteries.value.map(battery => battery.battery_temperature)
	)
	
	const batteries_currents = computed(
		() => batteries.value.map(battery => battery.current)
	)
	
	const battery_benches_temperatures = computed(
		() => batteries.value.map(battery => battery.temperature)
	)
	
	const bench_loads_temperatures = computed(
		() => batteries.value.map(battery => battery.electronic_load_temperature)
	)
	
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
