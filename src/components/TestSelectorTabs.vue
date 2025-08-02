<script setup lang="ts">
import {Tabs, TabsTrigger, TabsList} from "@/components/ui/tabs";
import { ref } from "vue";
import {Test} from "@/bindings";

const props = defineProps<{
  tests: Test[];
}>();

const selectedTest = ref<Test>();
defineExpose({selectedTest: selectedTest});

const selectTest = (test_id: number | null) => {
  let amountOftests = props.tests?.length ?? 0;
  for(let i = 0; i < amountOftests; i++){
    if(test_id == props.tests[i].test_id){
      selectedTest.value = props.tests[i];
    }
  }
}
</script>

<template>

  <Tabs default-value="etc" orientation="horizontal">
    <H6 class="font-bold"><pre> Select a Test</pre></H6>
    <TabsList>
      <TabsTrigger v-for="test in props.tests"  :value="test.test_id ?? ''" @click="selectTest(test.test_id)">
      </TabsTrigger>
    </TabsList>
  </Tabs>
</template>