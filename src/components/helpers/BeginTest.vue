<template>
  <div>
    <Button @click="start">Activate Process</Button>
  </div>
</template>

<script setup lang="ts">
import { Channel } from "@tauri-apps/api/core";
import { Button } from "../ui/button";
import { BatteryLog, commands } from "@/bindings";

const props = defineProps<{
  onEvent: Channel<BatteryLog>;
}>();

const start = async () => {
  commands.parseLog(props.onEvent);
};
</script>
