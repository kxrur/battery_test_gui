<template>
  <div>
    <h1>Help: {{ l }}</h1>
  </div>
</template>

<script setup lang="ts">
import { events } from "@/bindings";
import { ref } from "vue";

const l = ref<string>("no event");

// For all windows
events.demoEvent.listen((e) => {
  console.log("from backend", e);
  l.value = e.payload;
});
</script>
