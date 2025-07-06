<script setup lang="ts">
import { toast } from "vue-sonner";
import { Button } from "@/components/ui/button";
import { commands } from "@/bindings";

const onClick = () => {
  commands.exportCsv("/home/kxr/Desktop/meh");
};
</script>

<template>
  <div>
    <Button @click="onClick">Download Logs</Button>
    <Button
      variant="outline"
      @click="
        () => {
          toast('Event has been created', {
            description: 'Sunday, December 03, 2023 at 9:00 AM',
            action: {
              label: 'Undo',
              onClick: () => console.log('Undo'),
            },
          });
        }
      "
    >
      Add to calendar
    </Button>
  </div>
</template>
