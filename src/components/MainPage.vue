<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { onMounted, ref } from "vue";
import { BatteryLog, Test, commands, Result } from "@/bindings";
import { Channel } from "@tauri-apps/api/core";
import BeginTest from "@/components/helpers/BeginTest.vue";
import Charts from "@/components/Charts.vue";
import TestSelectorTabs from "@/components/TestSelectorTabs.vue";
import { rand } from "@vueuse/core";
import { Button } from "./ui/button";
import Card from "./ui/card/Card.vue";
import CardContent from "./ui/card/CardContent.vue";

const bat: BatteryLog = {
  battery_temperature: 22,
  current: 3,
  bench_temperature_mosfet: 11,
  bench_temperature_resistor: 33,
  load: 40,
  end_date: "end date",
  id: 1,
  port: "Port",
  record_id: 9,
  start_date: "start date",
  state: "state",
  status: "status",
  voltage: 399,
  test_id: 3,
};

const battery = ref<BatteryLog>(bat);

const onEvent = new Channel<BatteryLog>();
onEvent.onmessage = (batteryLog) => {
  battery.value = batteryLog;
  batteryLogs.value?.shift();
  batteryLogs.value?.push(batteryLog);
  console.log(`got battery log:`, batteryLog);
};

const batteryLogs = ref<BatteryLog[]>();

const tests = ref<Test[]>();
const debugRef = ref<any>();

const handleTestSelected = (test: Test) => {
  console.log('Test selected:', test);
  if (test.test_id) {
    retrieveLogsFor(test.test_id);
  }
};

function retrieveLogsFor(testId: number) {
  commands.getBatteryLogsForTest(testId)
    .then((result) => {
      if (result.status === "ok") {
        batteryLogs.value = result.data;
      } else {
        debugRef.value = result.error
      }
    })
    .catch((error) => console.log(error));
}

function retrieveAllTests() {
  commands
    .getAllTests()
    .then((value) => {
      if (value.status === "ok") {
        tests.value = value.data;
        console.log(tests);
      } else {
        debugRef.value = value.error
      }
    })
    .catch((error) => {
      console.log(error);
    });
}
onMounted(() => {
  retrieveAllTests();
});
</script>

<template>
  <section class="m-10 flex flex-col gap-10 w-fit max-w-full">
    <!--Top Section-->
    <Card class="px-4 py-4 w-10/12">
      <TestSelectorTabs v-if="tests != undefined" :tests="tests" @test-selected="handleTestSelected"
        @test-deleted="retrieveAllTests">
      </TestSelectorTabs>
    </Card>
    <section>
      <h1 class="text-2xl font-bold">Batteries Connected</h1>
      <Table class="min-w-full">
        <TableHeader>
          <TableRow>
            <TableHead>Battery Port</TableHead>
            <TableHead>Voltage</TableHead>
            <TableHead>Current</TableHead>
            <TableHead>Temperature</TableHead>
            <TableHead>Bench Temperature 1</TableHead>
            <TableHead>Bench Temperature 2</TableHead>
            <TableHead>Load</TableHead>
            <TableHead>Duration</TableHead>
            <!-- <TableHead>Bench State</TableHead>
            <TableHead>Test Status</TableHead> -->
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
            <TableCell>{{ battery.bench_temperature_mosfet / 100 }}C</TableCell>
            <TableCell>{{ battery.bench_temperature_resistor / 100 }}C</TableCell>
            <TableCell>{{ battery.load }}C</TableCell>
            <TableCell>{{ 3 }} </TableCell>
            <!-- <TableCell>
              <Badge variant="secondary"> Standby </Badge>
            </TableCell>
            <TableCell>
              <Badge variant="secondary"> Standby </Badge>
            </TableCell> -->
            <TableCell class="text-right">
              <BeginTest :onEvent="onEvent"></BeginTest>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </section>
    {{ debugRef }}
    <Charts v-if="batteryLogs != undefined" :battery-logs="batteryLogs">
    </Charts>
  </section>
</template>
