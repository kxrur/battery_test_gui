<template>
    <div class="space-y-2">
        <div class="space-y-2">
            <Label for="port-select">COM Port</Label>
            <Select v-model="selectedPort" @update:model-value="onPortChange">
                <SelectTrigger id="port-select" class="w-full">
                    <SelectValue placeholder="Select a COM port" />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem v-for="port in ports" :key="port" :value="port">
                        {{ port }}
                    </SelectItem>
                </SelectContent>
            </Select>
            <Button variant="outline" size="sm" @click="fetchPorts" :disabled="isLoadingPorts">
                {{ isLoadingPorts ? 'Refreshing...' : 'Refresh Ports' }}
            </Button>
        </div>

        <div class="space-y-2">
            <Label for="battery-id">Battery ID</Label>
            <Input id="battery-id" v-model.number="batteryId" type="number" placeholder="Enter battery ID (0-255)"
                min="0" max="255" @input="onBatteryIdChange" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { commands } from '@/bindings'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select'

const emit = defineEmits<{
    portSelected: [port: string]
    batteryIdSelected: [id: number]
}>()

// Reactive state
const ports = ref<string[]>([])
const selectedPort = ref<string>('')
const batteryId = ref<number | undefined>(undefined)
const isLoadingPorts = ref(false)

const fetchPorts = async () => {
    isLoadingPorts.value = true
    try {
        const result = await commands.detectSerialPorts()

        if (result.status === "ok") {
            ports.value = result.data
        } else {
            console.error("Failed to detect serial ports:", result.error)
        }
    } catch (err) {
        console.error("Unexpected error while detecting ports:", err)
    } finally {
        isLoadingPorts.value = false
    }
}

const onPortChange = (value: any) => {
    const port = value as string
    selectedPort.value = port || ''
    if (port) {
        emit('portSelected', port)
    }
}

const onBatteryIdChange = () => {
    if (batteryId.value !== undefined) {
        emit('batteryIdSelected', batteryId.value)
    }
}

onMounted(() => {
    fetchPorts()
})
</script>