<script setup lang="ts">
import { ref } from "vue";
import { Plus, Download, Calendar } from "lucide-vue-next";

import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";

import ManualAddBatteryLog from "./ManualAddBatteryLog.vue";
import DownloadLogs from "@/components/helpers/DownloadLogs.vue";
import BeginTest from "@/components/helpers/BeginTest.vue";

// Define dialog items and associated components
const dialogItems = [
  {
    key: "add",
    title: "Add Battery Log",
    icon: Plus,
    component: ManualAddBatteryLog,
  },
  {
    key: "download",
    title: "Download Csv",
    icon: Download,
    component: DownloadLogs,
  },
  {
    key: "event",
    title: "Event Test",
    icon: Calendar,
    component: BeginTest,
  },
] as const;

type DialogKey = (typeof dialogItems)[number]["key"];

const currentDialog = ref<null | DialogKey>(null);
const openDialog = (type: DialogKey) => (currentDialog.value = type);
const closeDialog = () => (currentDialog.value = null);
</script>

<template>
  <Sidebar>
    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupLabel>Debug Tools</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="item in dialogItems" :key="item.key">
              <Dialog>
                <DialogTrigger as-child>
                  <SidebarMenuButton @click="openDialog(item.key)">
                    <component :is="item.icon" />
                    <span>{{ item.title }}</span>
                  </SidebarMenuButton>
                </DialogTrigger>
                <DialogContent
                  v-if="currentDialog === item.key"
                  @close="closeDialog"
                >
                  <component :is="item.component" />
                </DialogContent>
              </Dialog>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
  </Sidebar>
</template>
