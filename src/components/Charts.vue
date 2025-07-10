<script setup lang="ts">
import { BatteryLog } from "@/bindings";
import { LineChart } from "@/components/ui/chart-line";
import { computed, ref } from "vue";

const props = defineProps<{
  batteryLogs: BatteryLog[];
}>();

const open_ports = computed(() =>
  Array.from(new Set(props.batteryLogs.map((log) => log.port))),
);

function createSeries(metric: keyof BatteryLog) {
  const series: Record<string, number[]> = {};
  const byIndex: { [key: string]: Record<string, number> }[] = [];

  open_ports.value.forEach((port) => {
    series[port] = [];
  });

  props.batteryLogs.forEach((log, i) => {
    if (!series[log.port]) series[log.port] = [];
    series[log.port].push(log[metric] as number);
  });

  const length = Math.max(...Object.values(series).map((arr) => arr.length));
  for (let i = 0; i < length; i++) {
    const entry: Record<string, any> = { index: i };
    for (const port of open_ports.value) {
      entry[port] = series[port]?.[i] ?? null;
    }
    byIndex.push(entry);
  }

  return byIndex;
}

const voltages = computed(() => createSeries("voltage"));
const currents = computed(() => createSeries("current"));
const battery_temp = computed(() => createSeries("battery_temperature"));
const bench_temp = computed(() => createSeries("temperature"));
const elec_temp = computed(() => createSeries("electronic_load_temperature"));
</script>

<template>
  <div>
    <section class="grid grid-cols-2 gap-5">
      <h2 class="text-2xl font-bold">Voltage [V]</h2>
      <h2 class="text-2xl font-bold">Current [mA]</h2>

      <LineChart
        class="max-h-64"
        :data="voltages"
        index="index"
        :categories="open_ports"
      />
      <LineChart
        class="max-h-64"
        :data="currents"
        index="index"
        :categories="open_ports"
      />
    </section>

    <section class="grid grid-cols-3 gap-5">
      <h2 class="text-2xl font-bold">Battery Temperature [C]</h2>
      <h2 class="text-2xl font-bold">Bench Temperature [C]</h2>
      <h2 class="text-2xl font-bold">Electronic Load Temperature [C]</h2>

      <LineChart
        class="max-h-64"
        :data="battery_temp"
        index="index"
        :categories="open_ports"
      />
      <LineChart
        class="max-h-64"
        :data="bench_temp"
        index="index"
        :categories="open_ports"
      />
      <LineChart
        class="max-h-64"
        :data="elec_temp"
        index="index"
        :categories="open_ports"
      />
    </section>
  </div>
</template>
