<template>
  <div class="flex flex-col gap-4">
    <!-- Command Dropdown -->
    <Select v-model="command">
      <SelectTrigger class="w-[180px]">
        <SelectValue placeholder="Select command" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Commands</SelectLabel>
          <SelectItem v-for="cmd in commandOptions" :key="cmd" :value="cmd">
            {{ cmd }}
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>

    <!-- Serial Port Dropdown -->
    <Select v-model="selectedPort">
      <SelectTrigger class="w-[180px]">
        <SelectValue placeholder="Select serial port" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Serial Ports</SelectLabel>
          <SelectItem v-for="port in ports" :key="port" :value="port">
            {{ port }}
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>

    <Button @click="sendCommand">Send</Button>
  </div>
</template>

<script setup lang="ts">
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectGroup,
  SelectLabel,
  SelectItem,
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { onMounted, ref } from "vue";
import { Command, commands } from "@/bindings";

const command = ref<Command | "">("");
const selectedPort = ref<string | "">("");

const commandOptions: Command[] = [
  "Ping",
  "RequestData",
  "SetCharge",
  "SetDischarge",
  "SetStandBy",
  "RequestCompletion",
];

const ports = ref<string[]>([]);

const fetchPorts = async () => {
  try {
    const result = await commands.detectSerialPorts();

    if (result.status === "ok") {
      ports.value = result.data;
    } else {
      console.error("Failed to detect serial ports:", result.error);
    }
  } catch (err) {
    console.error("Unexpected error while detecting ports:", err);
  }
};

const sendCommand = async () => {
  if (!command.value || !selectedPort.value) {
    console.warn("Missing command or port");
    return;
  }

  try {
    const result = await commands.commandRequest(
      command.value,
      selectedPort.value,
    );
    console.log("Result:", result);
  } catch (err) {
    console.error("Error sending command:", err);
  }
};

onMounted(fetchPorts);
</script>
