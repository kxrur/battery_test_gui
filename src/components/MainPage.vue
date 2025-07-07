<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { LineChart } from "@/components/ui/chart-line";
import { Badge } from "@/components/ui/badge";
import { ref } from "vue";
import { BatteryLog } from "@/bindings";
import { Channel } from "@tauri-apps/api/core";
import EventTest from "./debug/EventTest.vue";

interface TimeOption {
  label: string;
  value: number;
}

const bat: BatteryLog = {
  battery_temperature: 3,
  current: 3,
  electronic_load_temperature: 12,
  end_date: "end date",
  id: 1,
  port: "Port",
  record_id: 9,
  start_date: "start date",
  state: "state",
  status: "status",
  temperature: 3,
  voltage: 399,
};

const battery = ref<BatteryLog>(bat);

const onEvent = new Channel<BatteryLog>();
onEvent.onmessage = (batteryLog) => {
  battery.value = batteryLog;
  console.log(`got battery log:`, batteryLog);
};
</script>

<template>
  <section class="m-10 flex flex-col gap-10">
    <!--Top Section-->
    <section>
      <h1 class="text-2xl font-bold">Batteries Connected</h1>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Battery Port</TableHead>
            <TableHead>Voltage</TableHead>
            <TableHead>Current</TableHead>
            <TableHead>Temperature</TableHead>
            <TableHead>Bench Temperature</TableHead>
            <TableHead>Electronic Load Temperature</TableHead>
            <TableHead>Duration</TableHead>
            <TableHead>Bench State</TableHead>
            <TableHead>Test Status</TableHead>
            <TableHead></TableHead>
          </TableRow>
        </TableHeader>

        <!--Table Battery-->
        <TableBody>
          <TableRow>
            <TableCell>{{ battery.port }}</TableCell>
            <TableCell>{{ battery.voltage / 100 }}V</TableCell>
            <TableCell>{{ battery.current / 100 }}mA</TableCell>
            <TableCell>{{ battery.battery_temperature / 100 }}C</TableCell>
            <TableCell>{{ battery.temperature / 100 }}C</TableCell>
            <TableCell
              >{{ battery.electronic_load_temperature / 100 }}C</TableCell
            >
            <TableCell>{{ 3 }} </TableCell>
            <TableCell>
              <Badge variant="secondary"> Standby </Badge>
            </TableCell>
            <TableCell>
              <Badge variant="secondary"> Standby </Badge>
            </TableCell>
            <TableCell class="text-right">
              <EventTest :onEvent="onEvent"></EventTest>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </section>
    <section class="grid grid-cols-2 gap-5">
      <h2 class="text-2xl font-bold">Voltage [V]</h2>
      <h2 class="text-2xl font-bold">Current [mA]</h2>
    </section>

    <section class="grid grid-cols-3 gap-5">
      <h2 class="text-2xl font-bold">Battery Temperature [C]</h2>
      <h2 class="text-2xl font-bold">Bench Temperature [C]</h2>
      <h2 class="text-2xl font-bold">Electronic Load Temperature [C]</h2>
    </section>
  </section>
</template>
