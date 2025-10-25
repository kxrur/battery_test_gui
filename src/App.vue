<script setup lang="ts">
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import DebugNavBar from "./components/debug/DebugNavBar.vue";
import MainPage from "./components/MainPage.vue";
import { Toaster } from "@/components/ui/sonner";
import "vue-sonner/style.css";
import SidebarInset from "./components/ui/sidebar/SidebarInset.vue";
</script>

<template>
  <Toaster />
  <SidebarProvider>
    <DebugNavBar />
    <main>
      <SidebarTrigger />
      <SidebarInset class="flex flex-col min-h-screen overflow-x-hidden">
        <MainPage></MainPage>
      </SidebarInset>
    </main>
  </SidebarProvider>
</template>
