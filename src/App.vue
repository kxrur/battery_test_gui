<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import DebugNavBar from "./components/debug/DebugNavBar.vue";
import MainPage from "./components/MainPage.vue";
import { Toaster } from "@/components/ui/sonner";
import "vue-sonner/style.css";
</script>

<template>
  <Toaster />
  <SidebarProvider>
    <DebugNavBar />
    <main>
      <SidebarTrigger />
      <MainPage></MainPage>
    </main>
  </SidebarProvider>
</template>
