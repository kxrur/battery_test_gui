<template>
  <div>
    <Input placeholder="command" :model-value="command"></Input>
    <Button @click="sendCommand">Send</Button>
  </div>
</template>

<script setup lang="ts">
import { Input } from "@/components/ui/input";
import { Button } from "../ui/button";
import { ref } from "vue";

const command = ref<string>("command");

const sendCommand = () => {
  console.log("command: ", command.value);
};
</script>
