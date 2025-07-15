<template>
  <div>
    <Button @click="generateFakeData" :disabled="isLoading">
      <span v-if="isLoading">Generating...</span>
      <span v-else>Generate Fake Data</span>
    </Button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { toast } from "vue-sonner";
import { commands } from "@/bindings";
import { Button } from "@/components/ui/button";

const isLoading = ref(false);

async function generateFakeData() {
  isLoading.value = true;

  try {
    await commands.populateFakeData();

    toast("Success!", {
      description: "1000 battery logs were generated.",
    });
  } catch (err) {
    toast("Error generating data", {
      description:
        err instanceof Error ? err.message : "Unexpected error occurred.",
    });
  } finally {
    isLoading.value = false;
  }
}
</script>
