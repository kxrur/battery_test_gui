<script setup lang="ts">
//FIXME: the modal window of the 'download logs' dialog can be closed while the
//file picker is opened
import { toast } from "vue-sonner";
import { Button } from "@/components/ui/button";
import { commands } from "@/bindings";
import { ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";

const storageFolder = ref<string | null>(null);

const openDirectoryPicker = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select Data Folder",
    });

    if (selected) {
      storageFolder.value = selected as string;
      try {
        const result = await commands.exportCsv(storageFolder.value);
        if (result.status === "ok") {
          toast("Battery logs downloaded", {
            description: `Battery logs downloaded to ${storageFolder.value}`,
          });
        }
      } catch (error) {
        toast("Save Failed", {
          description:
            error instanceof Error
              ? error.message
              : "An unexpected error occurred.",
        });
      }
    }
  } catch (error) {
    console.error("Error selecting directory:", error);
  }
};
</script>

<template>
  <div>
    <Button @click="openDirectoryPicker()">Download Logs</Button>
  </div>
</template>
