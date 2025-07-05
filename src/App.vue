<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import DebugNavBar from "./components/debug/DebugNavBar.vue";
</script>

<template>
  <SidebarProvider>
    <DebugNavBar />
    <main>
      <SidebarTrigger />
      <slot />
    </main>
  </SidebarProvider>
</template>
