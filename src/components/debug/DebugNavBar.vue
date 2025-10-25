<script setup lang="ts">
import { ref } from "vue";
import { Plus, Download, CloudUpload, CloudDownload, Play, Bolt } from "lucide-vue-next";

import {
  Dialog,
  DialogContent,
  DialogTrigger,
  DialogTitle,
  DialogDescription,
} from "@/components/ui/dialog";
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
import SendSerial from "./SendSerial.vue";
import GenerateFakeData from "./GenerateFakeData.vue";
import BatteryState from "./BatteryState.vue";
import BatteryData from "./BatteryData.vue";

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
    key: "serial",
    title: "Serial Command",
    icon: Play,
    component: SendSerial,
  },
  {
    key: "fakeData",
    title: "Fake Data",
    icon: CloudUpload,
    component: GenerateFakeData,
  },
  {
    key: "setState",
    title: "Set State",
    icon: Bolt,
    component: BatteryState,
  },
  {
    key: "retrieveData",
    title: "Retrieve Data",
    icon: CloudDownload,
    component: BatteryData,
  },
] as const;

const openStates = ref<Record<string, boolean>>({});

dialogItems.forEach((item) => {
  openStates.value[item.key] = false;
});
</script>

<template>
  <Sidebar>
    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupLabel>Debug Tools</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="item in dialogItems" :key="item.key">
              <Dialog v-model:open="openStates[item.key]">
                <DialogTrigger as-child>
                  <SidebarMenuButton @click="openStates[item.key] = true">
                    <component :is="item.icon" />
                    <span>{{ item.title }}</span>
                  </SidebarMenuButton>
                </DialogTrigger>

                <DialogContent class="pt-10">
                  <DialogTitle class="text-lg font-medium">
                    {{ item.title }} Test
                  </DialogTitle>
                  <DialogDescription></DialogDescription>
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
