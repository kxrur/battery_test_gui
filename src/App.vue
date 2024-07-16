<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { LineChart } from '@/components/ui/chart-line'
import { Badge } from '@/components/ui/badge'
import { useBatteryManager } from '@/lib/battery.ts'
import { invoke } from '@tauri-apps/api/tauri' // added to use the export_to_csv() from backend and invoke tauri commands
import { appDataDir } from '@tauri-apps/api/path'// added to obtain directory path


const batteryManager = useBatteryManager();

const data = [
  {
    'year': 1970,
    '0x00': 2.04,
    '0x01': 1.53,
  },
  {
    'year': 1971,
    '0x00': 1.96,
    '0x01': 1.58,
  },
  {
    'year': 1972,
    '0x00': 1.96,
    '0x01': 1.61,
  },
  {
    'year': 1973,
    '0x00': 1.93,
    '0x01': 1.61,
  },
  {
    'year': 1974,
    '0x00': 1.88,
    '0x01': 1.67,
  },
  {
    'year': 1975,
    '0x00': 1.79,
    '0x01': 1.64,
  },
];

// invokes the export_csv_command tauri command and creates the csv file in the project's main directory (supposed to)
async function exportToCSV() {
  try {
    // const projectDir = await invoke('get_project_dir', { steps: 3 });
    let projectDir = "C://Users//zephr//Desktop//SC";
    console.log('Project Directory:', projectDir); // Debug
    const csvPath = projectDir;
    await invoke('export_csv_command', { csvPath });
    alert('CSV export successful!');
  } catch (error) {
    console.error('Failed to export CSV:', error);
    alert('Failed to export CSV.');
  }
}

</script>

<template>
  <section class="m-10 flex flex-col gap-10">
    <section>
      <h1 class="text-2xl font-bold">Batteries Connected</h1>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Battery IDs</TableHead>
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
        <TableBody>
          <TableRow>
            <TableCell>0x00</TableCell>
            <TableCell>5V</TableCell>
            <TableCell>0.2mA</TableCell>
            <TableCell>20.02C</TableCell>
            <TableCell>20.02C</TableCell>
            <TableCell>20.02C</TableCell>
            <TableCell>10:30:03</TableCell>
            <TableCell>
              <Badge variant="secondary">
                Standby
              </Badge>
            </TableCell>
            <TableCell>
              <Badge variant="secondary">
                Standby
              </Badge>
            </TableCell>
            <TableCell class="text-right">
              <Button>Begin Test</Button>
            </TableCell>
          </TableRow>
          <TableRow>
            <TableCell>0x01</TableCell>
            <TableCell>5V</TableCell>
            <TableCell>0.2mA</TableCell>
            <TableCell>20.02C</TableCell>
            <TableCell>20.02C</TableCell>
            <TableCell>20.02C</TableCell>
            <TableCell>10:30:03</TableCell>
            <TableCell>
              <Badge variant="secondary">
                Standby
              </Badge>
            </TableCell>
            <TableCell>
              <Badge variant="secondary">
                Standby
              </Badge>
            </TableCell>
            <TableCell class="text-right">
              <Button>Begin Test</Button>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </section>
    <section class="grid grid-cols-2 gap-5">
      <h2 class="text-2xl font-bold">Voltage [V]</h2>
      <h2 class="text-2xl font-bold">Current [mA]</h2>

      <LineChart
        class="max-h-64"
        :data="data"
        index="year"
        :categories="['0x00', '0x01']"
      />
      <LineChart
        class="max-h-64"
        :data="data"
        index="year"
        :categories="['0x00', '0x01']"
      />
    </section>
    <section class="grid grid-cols-3 gap-5">
      <h2 class="text-2xl font-bold">Battery Temperature [C]</h2>
      <h2 class="text-2xl font-bold">Bench Temperature [C]</h2>
      <h2 class="text-2xl font-bold">Electronic Load Temperature [C]</h2>

      <LineChart
        class="max-h-64"
        :data="data"
        index="year"
        :categories="['0x00', '0x01']"
      />
      <LineChart
        class="max-h-64"
        :data="data"
        index="year"
        :categories="['0x00', '0x01']"
      />
      <LineChart
        class="max-h-64"
        :data="data"
        index="year"
        :categories="['0x00', '0x01']"
      />
    </section>
    <section>
      <Button @click="exportToCSV">Export to CSV</Button>
    </section>
  </section>
</template>