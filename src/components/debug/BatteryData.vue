<template>
    <div class="space-y-4 px-4">
        <BatterySelection @port-selected="onPortSelected" @battery-id-selected="onBatteryIdSelected" />

        <Button @click="onGetDataClick" :disabled="!port || batteryId === undefined || isLoading" class="w-full">
            <span v-if="isLoading">Getting Data...</span>
            <span v-else>Get Data</span>
        </Button>

        <!-- Display Battery Log Data -->
        <Card v-if="batteryLogData" class="w-full">
            <CardHeader>
                <CardTitle>Battery Data</CardTitle>
                <CardDescription>
                    Data from Battery ID: {{ batteryLogData.id }} on Port: {{ batteryLogData.port }}
                </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="grid grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <h4 class="font-medium">Temperature</h4>
                        <div class="text-sm space-y-1">
                            <div><strong>Battery:</strong> {{ batteryLogData.battery_temperature }}°C</div>
                            <div><strong>MOSFET:</strong> {{ batteryLogData.bench_temperature_mosfet }}°C</div>
                            <div><strong>Resistor:</strong> {{ batteryLogData.bench_temperature_resistor }}°C</div>
                        </div>
                    </div>

                    <div class="space-y-2">
                        <h4 class="font-medium">Electrical</h4>
                        <div class="text-sm space-y-1">
                            <div><strong>Voltage:</strong> {{ batteryLogData.voltage }}V</div>
                            <div><strong>Current:</strong> {{ batteryLogData.current }}A</div>
                            <div><strong>Load:</strong> {{ batteryLogData.load }}</div>
                        </div>
                    </div>

                    <div class="space-y-2">
                        <h4 class="font-medium">Status</h4>
                        <div class="text-sm space-y-1">
                            <div><strong>State:</strong> {{ batteryLogData.state }}</div>
                            <div><strong>Status:</strong> {{ batteryLogData.status }}</div>
                            <div><strong>Test ID:</strong> {{ batteryLogData.test_id }}</div>
                        </div>
                    </div>

                    <div class="space-y-2">
                        <h4 class="font-medium">Timestamps</h4>
                        <div class="text-sm space-y-1">
                            <div><strong>Start:</strong> {{ batteryLogData.start_date || 'N/A' }}</div>
                            <div><strong>End:</strong> {{ batteryLogData.end_date || 'N/A' }}</div>
                        </div>
                    </div>
                </div>
            </CardContent>
        </Card>

        <!-- Display Error Message -->
        <Card v-if="errorMessage && !batteryLogData" class="w-full border-destructive">
            <CardHeader>
                <CardTitle class="text-destructive">Error</CardTitle>
            </CardHeader>
            <CardContent>
                <p class="text-sm text-destructive">{{ errorMessage }}</p>
            </CardContent>
        </Card>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { toast } from 'vue-sonner'
import BatterySelection from './BatterySelection.vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { type Battery, type Bench, type BatteryLog, commands } from '@/bindings'

const port = ref<string>()
const batteryId = ref<number>()
const isLoading = ref(false)
const batteryLogData = ref<BatteryLog | null>(null)
const errorMessage = ref<string>('')

const onPortSelected = (selectedPort: string) => {
    port.value = selectedPort
    batteryLogData.value = null
    errorMessage.value = ''
}

const onBatteryIdSelected = (selectedId: number) => {
    batteryId.value = selectedId
    batteryLogData.value = null
    errorMessage.value = ''
}

const onGetDataClick = async () => {
    if (!port.value || batteryId.value === undefined) {
        toast('Error', {
            description: 'Please select both a port and battery ID.',
        })
        return
    }

    isLoading.value = true
    batteryLogData.value = null
    errorMessage.value = ''

    try {
        const battery: Battery = {
            id: batteryId.value,
            state: 'Standby'
        }

        const bench: Bench = {
            batteries: [],
            port: port.value
        }

        const result = await commands.dataRequest(bench, battery)

        if (result.status === 'ok') {
            batteryLogData.value = result.data
            toast('Success!', {
                description: `Battery data retrieved successfully from ID ${batteryId.value}`,
            })
        } else {
            errorMessage.value = result.error || 'Unknown error occurred'
            toast('Error', {
                description: result.error || 'Failed to get battery data',
            })
        }
    } catch (err) {
        const message = err instanceof Error ? err.message : 'Unexpected error occurred'
        errorMessage.value = message
        toast('Error', {
            description: message,
        })
    } finally {
        isLoading.value = false
    }
}
</script>