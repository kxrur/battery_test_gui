<script setup lang="ts">
import { Button } from "../ui/button";
import { commands } from "@/bindings";

const onClick = () => {
  commands.exportCsv("/home/kxr/Desktop/meh");
};
</script>

<template>
  <div>
    <Button @click="onClick">Download Logs</Button>
  </div>
</template>
