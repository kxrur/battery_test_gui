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
import {BatteryLog, Test, commands, Result} from "@/bindings";
import { Channel } from "@tauri-apps/api/core";
import BeginTest from "@/components/helpers/BeginTest.vue";
import Charts from "@/components/Charts.vue";
import TestSelectorTabs from "@/components/TestSelectorTabs.vue";
import { rand } from "@vueuse/core";
import { Button } from "./ui/button";

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

const testSelectorTabs = ref<typeof TestSelectorTabs>();
const tests = ref<Test[]>();
const debugRef = ref<any>();

const handleTabClick = () => {
  const selectedTestId = testSelectorTabs.value?.selectedTest.test_id;
  retrieveLogsFor(selectedTestId);
};
function retrieveLogsFor(testId: number){
  commands.getBatteryLogsForTest(testId)
      .then((result) => {
        if (result.status === "ok") {
        batteryLogs.value = result.data;
        } else{
          debugRef.value = result.error
        }
      })
      .catch((error) => console.log(error));
}

function retrieveAllTests(){
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
    tests.value = [{test_id:1, test_name:"test", start_date:"date"}, {test_id:2, test_name:"test", start_date:"date"}];
  });
</script>

<template>
  <section class="m-10 flex flex-col gap-10">
    <!--Top Section-->
    <TestSelectorTabs ref="testSelectorTabs" v-if="tests != undefined" :tests="tests" @click="handleTabClick"></TestSelectorTabs>
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
              <BeginTest :onEvent="onEvent"></BeginTest>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </section>
    {{debugRef}}
    <Charts v-if="batteryLogs != undefined" :battery-logs="batteryLogs">
    </Charts>
  </section>
</template>
