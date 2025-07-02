<script setup lang="ts">
import { ref } from "vue";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { toast } from "vue-sonner";

// Import the commands if they're available in your setup
import { commands } from "@/bindings"; // Adjust the import path as needed

const formData = ref({
  id: 0,
  port: "",
  temperature: 0,
  battery_temperature: 0,
  electronic_load_temperature: 0,
  voltage: 0,
  current: 0,
  state: "",
  status: "",
});

const isLoading = ref(false);

const states = ["charging", "discharging", "idle", "maintenance", "error"];

const statuses = ["active", "inactive", "completed", "failed"];

const ports = ["COM1", "COM2", "COM3", "USB1", "USB2"];

const handleSubmit = async () => {
  isLoading.value = true;

  try {
    const result = await commands.insertBatteryLog({
      record_id: null, // will be set by the database
      id: formData.value.id,
      port: formData.value.port,
      temperature: formData.value.temperature,
      battery_temperature: formData.value.battery_temperature,
      electronic_load_temperature: formData.value.electronic_load_temperature,
      voltage: formData.value.voltage,
      current: formData.value.current,
      state: formData.value.state,
      status: formData.value.status,
      start_date: null, // will be set by the database if needed
      end_date: null, // will be set by the database if needed
    });

    if (result.status === "ok") {
      toast({
        title: "Success",
        description: "Battery log has been saved successfully",
        variant: "default",
      });
      // Reset form or navigate away
      formData.value = {
        id: 0,
        port: "",
        temperature: 0,
        battery_temperature: 0,
        electronic_load_temperature: 0,
        voltage: 0,
        current: 0,
        state: "",
        status: "",
      };
    } else {
      toast({
        title: "Error",
        description: result.error,
        variant: "destructive",
      });
    }
  } catch (error) {
    toast({
      title: "Error",
      description:
        error instanceof Error ? error.message : "An unknown error occurred",
      variant: "destructive",
    });
  } finally {
    isLoading.value = false;
  }
};
</script>

<template>
  <div class="max-w-md mx-auto p-6 bg-card rounded-lg shadow-sm">
    <h1 class="text-2xl font-bold mb-6">Add Battery Log</h1>

    <form @submit.prevent="handleSubmit" class="space-y-4">
      <div>
        <Label for="id">Battery ID</Label>
        <Input
          id="id"
          v-model.number="formData.id"
          type="number"
          placeholder="Enter battery ID"
          required
        />
      </div>

      <div>
        <Label for="port">Port</Label>
        <Select v-model="formData.port" required>
          <SelectTrigger>
            <SelectValue placeholder="Select a port" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="port in ports" :key="port" :value="port">
              {{ port }}
            </SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <Label for="temperature">Temperature (°C)</Label>
          <Input
            id="temperature"
            v-model.number="formData.temperature"
            type="number"
            step="0.1"
            placeholder="0.0"
            required
          />
        </div>

        <div>
          <Label for="battery_temperature">Battery Temp (°C)</Label>
          <Input
            id="battery_temperature"
            v-model.number="formData.battery_temperature"
            type="number"
            step="0.1"
            placeholder="0.0"
            required
          />
        </div>
      </div>

      <div>
        <Label for="electronic_load_temperature"
          >Electronic Load Temp (°C)</Label
        >
        <Input
          id="electronic_load_temperature"
          v-model.number="formData.electronic_load_temperature"
          type="number"
          step="0.1"
          placeholder="0.0"
          required
        />
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <Label for="voltage">Voltage (V)</Label>
          <Input
            id="voltage"
            v-model.number="formData.voltage"
            type="number"
            step="0.01"
            placeholder="0.00"
            required
          />
        </div>

        <div>
          <Label for="current">Current (A)</Label>
          <Input
            id="current"
            v-model.number="formData.current"
            type="number"
            step="0.01"
            placeholder="0.00"
            required
          />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <Label for="state">State</Label>
          <Select v-model="formData.state" required>
            <SelectTrigger>
              <SelectValue placeholder="Select state" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem v-for="state in states" :key="state" :value="state">
                {{ state }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div>
          <Label for="status">Status</Label>
          <Select v-model="formData.status" required>
            <SelectTrigger>
              <SelectValue placeholder="Select status" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem
                v-for="status in statuses"
                :key="status"
                :value="status"
              >
                {{ status }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <Button type="submit" class="w-full" :disabled="isLoading">
        <span v-if="isLoading">Saving...</span>
        <span v-else>Save Battery Log</span>
      </Button>
    </form>
  </div>
</template>
