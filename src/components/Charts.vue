<script setup lang="ts">
import { BatteryLog } from "@/bindings";
import { LineChart } from "@/components/ui/chart-line";
import { computed, ref } from "vue";

const props = defineProps<{
  batteryLogs: BatteryLog[];
}>();

const batteryIds = computed(() =>
  Array.from(new Set(props.batteryLogs.map((log) => log.id.toString()))),
);

function createSeries(metric: keyof BatteryLog) {
  const series: Record<string, number[]> = {};
  const byIndex: { [key: string]: Record<string, number> }[] = [];

  batteryIds.value.forEach((id) => {
    series[id] = [];
  });

  console.log("battery log: ", props.batteryLogs)

  props.batteryLogs.forEach((log, i) => {
    if (!series[log.id]) series[log.id] = [];
    series[log.id].push(log[metric] as number);
  });

  const length = Math.max(...Object.values(series).map((arr) => arr.length));
  for (let i = 0; i < length; i++) {
    const entry: Record<string, any> = { index: i };
    for (const id of batteryIds.value) {
      entry[id] = series[id]?.[i] ?? null;
    }
    byIndex.push(entry);
  }

  console.log(byIndex);

  return byIndex;
}

const voltages = computed(() => createSeries("voltage"));
const currents = computed(() => createSeries("current"));
const battery_temp = computed(() => createSeries("battery_temperature"));
const bench_temp = computed(() => createSeries("bench_temperature_mosfet"));
const load = computed(() => createSeries("load"));
</script>

<template>
  <div>
    <section class="grid grid-cols-2 gap-5">
      <h2 class="text-2xl font-bold">Voltage [V]</h2>
      <h2 class="text-2xl font-bold">Current [mA]</h2>

      <LineChart class="max-h-64" :data="voltages" index="index" :categories="batteryIds" />
      <LineChart class="max-h-64" :data="currents" index="index" :categories="batteryIds" />
    </section>

    <section class="grid grid-cols-3 gap-5">
      <h2 class="text-2xl font-bold">Battery Temperature [C]</h2>
      <h2 class="text-2xl font-bold">Bench Temperature MOSFET[C]</h2>
      <h2 class="text-2xl font-bold">Load [Ohm]</h2>

      <LineChart class="max-h-64" :data="battery_temp" index="index" :categories="batteryIds" />
      <LineChart class="max-h-64" :data="bench_temp" index="index" :categories="batteryIds" />
      <LineChart class="max-h-64" :data="load" index="index" :categories="batteryIds" />
    </section>
  </div>
</template>
