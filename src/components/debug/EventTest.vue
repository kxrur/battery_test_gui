<template>
  <div>
    <Button @click="start">Activate Process</Button>
  </div>
</template>

<script setup lang="ts">
import { Channel } from "@tauri-apps/api/core";
import { Button } from "../ui/button";
import { BatteryLog, commands } from "@/bindings";

const onEvent = new Channel<BatteryLog>();

const start = async () => {
  commands.parseLog(onEvent);
};

onEvent.onmessage = (message) => {
  console.log(`got download event`, message.end_date);
  console.log(`got download data`, message.id);
};
</script>
