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
	port: string;
	temperature: number;
	battery_temperature: number;
	electronic_load_temperature: number;
	voltage: number;
	current: number;
	state: BatteryBenchState;
	status: CompletionStatus;
	start_date: Date;
	end_date: Date;
}

type chartData = {
	[key : string] : any,
}

export function useBatteryManager() {
	const batteries = ref<Map<string,BatteryBench[]>>( new Map<string , BatteryBench[]>);
	
	const open_ports = computed(
		() => [...batteries.value].map(battery => battery[0])
	)

	const latest_data = computed(() => 
	[...batteries.value].map(battery => battery[1][battery[1].length - 1]))

	const batteries_voltages = computed(
		() => {

			const data: chartData[] = [];

			[...batteries.value].forEach(battery => {

				for(let i=0; i < battery[1].length; i++)
				{

					if(data[i] == undefined)
						data[i] = { 'index' : i };
					
					data[i][battery[0]] = battery[1][i].voltage / 100;
				}
			})

			return data;
		})
	
	const batteries_temperatures = computed(
		() => {

			const data: chartData[] = [];

			[...batteries.value].forEach(battery => {

				for(let i=0; i < battery[1].length; i++)
				{

					if(data[i] == undefined)
						data[i] = { 'index' : i };
					
					data[i][battery[0]] = battery[1][i].battery_temperature / 100;
				}
			})

			return data;
		})
	
	const batteries_currents = computed(
		() => {

			const data: chartData[] = [];

			[...batteries.value].forEach(battery => {

				for(let i=0; i < battery[1].length; i++)
				{

					if(data[i] == undefined)
						data[i] = { 'index' : i };
					
					data[i][battery[0]] = battery[1][i].current / 100;
				}
			})

			return data;
		})	
	const battery_benches_temperatures = computed(
		() => {

			const data: chartData[] = [];

			[...batteries.value].forEach(battery => {

				for(let i=0; i < battery[1].length; i++)
				{

					if(data[i] == undefined)
						data[i] = { 'index' : i };
					
					data[i][battery[0]] = battery[1][i].temperature / 100;
				}
			})

			return data;
		})
	const bench_loads_temperatures = computed(
		() => {

			const data: chartData[] = [];

			[...batteries.value].forEach(battery => {

				for(let i=0; i < battery[1].length; i++)
				{

					if(data[i] == undefined)
						data[i] = { 'index' : i };
					
					data[i][battery[0]] = battery[1][i].electronic_load_temperature / 100;
				}
			})

			return data;
		})

	onMounted(async () => {
		await listen('display-battery', event => {

			const payload = event.payload as BatteryBench;
			
			!batteries.value?.has(payload.port) && batteries.value?.set(payload.port, []);

			batteries.value?.get(payload.port)?.push(payload);

		});
	});

	return {
		batteries,
		latest_data,
		open_ports,
		batteries_voltages,
		batteries_currents,
		batteries_temperatures,
		battery_benches_temperatures,
		bench_loads_temperatures
	}
}
