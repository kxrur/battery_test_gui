<script setup lang="ts">
import { Tabs, TabsTrigger, TabsList } from "@/components/ui/tabs";
import { ScrollArea, ScrollBar } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import { Plus, Trash2 } from "lucide-vue-next";
import { ref } from "vue";
import { toast } from "vue-sonner";
import { commands, Test } from "@/bindings";

const props = defineProps<{
  tests: Test[];
}>();

const selectedTest = ref<Test>();

const emit = defineEmits<{
  testSelected: [test: Test]
  testDeleted: []
  testAdded: []
}>()

const selectTest = (test: Test) => {
  selectedTest.value = test;
  emit('testSelected', test);
}

const addTest = async () => {
  try {
    const result = await commands.insertNewTest();

    if (result.status === "ok") {
      const newTest = result.data;

      toast("Success!", {
        description: `Test "${newTest.test_name}" was created successfully.`,
      });

      emit('testAdded');

      selectedTest.value = newTest;
      emit('testSelected', newTest);
    } else {
      toast("Error creating test", {
        description: result.error,
      });
    }

  } catch (err) {
    toast("Error creating test", {
      description: err instanceof Error ? err.message : "Unexpected error occurred.",
    });
  }
}

const deleteTest = async () => {
  if (selectedTest.value && selectedTest.value.test_id) {
    try {
      await commands.deleteTest(selectedTest.value.test_id);

      toast("Success!", {
        description: `Test "${selectedTest.value.test_name}" was deleted successfully.`,
      });

      selectedTest.value = undefined;

      emit('testDeleted');

    } catch (err) {
      toast("Error deleting test", {
        description: err instanceof Error ? err.message : "Unexpected error occurred.",
      });
    }
  }
}
</script>

<template>
  <div class="flex flex-col gap-3 w-full">
    <h1 class="text-3xl font-bold">Tests</h1>
    <ScrollArea class="w-full whitespace-nowrap">
      <div class="w-full relative">
        <Tabs default-value="etc" orientation="horizontal">
          <TabsList class="inline-flex">
            <TabsTrigger v-for="test in props.tests" :value="test.test_id?.toString() ?? ''" @click="selectTest(test)">
              {{ test?.test_name }}
            </TabsTrigger>
          </TabsList>
        </Tabs>
      </div>
      <ScrollBar orientation="horizontal" />
    </ScrollArea>
    <div class="flex gap-2 justify-end">
      <Button @click="addTest" size="sm" class="flex items-center gap-2">
        <Plus class="h-4 w-4" />
        Add Test
      </Button>
      <Button @click="deleteTest" variant="destructive" size="sm" class="flex items-center gap-2"
        :disabled="!selectedTest">
        <Trash2 class="h-4 w-4" />
        Delete
      </Button>
    </div>
  </div>
</template>