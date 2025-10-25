<template>
    <div class="space-y-4 px-4">
        <BatterySelection @port-selected="onPortSelected" @battery-id-selected="onBatteryIdSelected" />

        <!-- State Selection -->
        <div class="space-y-2">
            <Label for="state-select">Battery State</Label>
            <Select v-model="selectedState" @update:model-value="onStateChange">
                <SelectTrigger id="state-select" class="w-full">
                    <SelectValue placeholder="Select battery state" />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="Standby">
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 rounded-full bg-gray-400"></div>
                            Standby
                        </div>
                    </SelectItem>
                    <SelectItem value="Charge">
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 rounded-full bg-green-500"></div>
                            Charge
                        </div>
                    </SelectItem>
                    <SelectItem value="Discharge">
                        <div class="flex items-center gap-2">
                            <div class="w-3 h-3 rounded-full bg-red-500"></div>
                            Discharge
                        </div>
                    </SelectItem>
                </SelectContent>
            </Select>
        </div>

        <Button @click="onSetStateClick" :disabled="!port || batteryId === undefined || !selectedState || isLoading"
            class="w-full">
            <span v-if="isLoading">Setting State...</span>
            <span v-else>Set Battery State</span>
        </Button>

        <!-- Display State Change Result -->
        <Card v-if="stateChangeResult" class="w-full">
            <CardHeader>
                <CardTitle>State Change Successful</CardTitle>
                <CardDescription>
                    Battery ID {{ batteryId }} on Port: {{ port }}
                </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="flex items-center justify-center space-x-4">
                    <div class="text-center">
                        <div class="text-sm text-muted-foreground mb-1">Previous State</div>
                        <div class="flex items-center gap-2 justify-center">
                            <div :class="getStateColor(previousState)" class="w-3 h-3 rounded-full"></div>
                            <span class="font-medium">{{ previousState || 'Unknown' }}</span>
                        </div>
                    </div>

                    <div class="text-2xl">→</div>

                    <div class="text-center">
                        <div class="text-sm text-muted-foreground mb-1">New State</div>
                        <div class="flex items-center gap-2 justify-center">
                            <div :class="getStateColor(selectedState)" class="w-3 h-3 rounded-full"></div>
                            <span class="font-medium text-primary">{{ selectedState }}</span>
                        </div>
                    </div>
                </div>

                <div class="bg-muted/50 p-4 rounded-lg space-y-2">
                    <h4 class="font-medium">Change Details</h4>
                    <div class="text-sm space-y-1">
                        <div><strong>Port:</strong> {{ port }}</div>
                        <div><strong>Battery ID:</strong> {{ batteryId }}</div>
                        <div><strong>New State:</strong> {{ selectedState }}</div>
                        <div><strong>Timestamp:</strong> {{ changeTimestamp }}</div>
                        <div class="text-xs text-muted-foreground mt-2">{{ stateChangeResult }}</div>
                    </div>
                </div>
            </CardContent>
        </Card>

        <!-- Display Error Message -->
        <Card v-if="errorMessage && !stateChangeResult" class="w-full border-destructive">
            <CardHeader>
                <CardTitle class="text-destructive">State Change Failed</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="space-y-2">
                    <p class="text-sm text-destructive">{{ errorMessage }}</p>
                    <div class="text-xs text-muted-foreground">
                        <div><strong>Port:</strong> {{ port || 'Not selected' }}</div>
                        <div><strong>Battery ID:</strong> {{ batteryId ?? 'Not selected' }}</div>
                        <div><strong>Target State:</strong> {{ selectedState || 'Not selected' }}</div>
                    </div>
                </div>
            </CardContent>
        </Card>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { toast } from 'vue-sonner'
import BatterySelection from './BatterySelection.vue'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '@/components/ui/select'
import { type Battery, type Bench, type BatteryState, commands } from '@/bindings'

const port = ref<string>()
const batteryId = ref<number>()
const selectedState = ref<BatteryState>()
const previousState = ref<BatteryState>()
const isLoading = ref(false)
const stateChangeResult = ref<string>('')
const errorMessage = ref<string>('')
const changeTimestamp = ref<string>('')

const onPortSelected = (selectedPort: string) => {
    port.value = selectedPort
    // Clear previous data when port changes
    clearResults()
}

const onBatteryIdSelected = (selectedId: number) => {
    batteryId.value = selectedId
    // Clear previous data when battery ID changes
    clearResults()
}

const onStateChange = (state: any) => {
    selectedState.value = state as BatteryState
    // Clear previous data when state changes
    clearResults()
}

const getStateColor = (state: BatteryState | undefined): string => {
    switch (state) {
        case 'Standby': return 'bg-gray-400'
        case 'Charge': return 'bg-green-500'
        case 'Discharge': return 'bg-red-500'
        default: return 'bg-gray-300'
    }
}

const clearResults = () => {
    stateChangeResult.value = ''
    errorMessage.value = ''
    changeTimestamp.value = ''
    previousState.value = undefined
}

const onSetStateClick = async () => {
    if (!port.value || batteryId.value === undefined || !selectedState.value) {
        toast('Error', {
            description: 'Please select port, battery ID, and target state.',
        })
        return
    }

    isLoading.value = true
    clearResults()
    previousState.value = selectedState.value // Store current state as previous

    try {
        const battery: Battery = {
            id: batteryId.value,
            state: selectedState.value
        }

        const bench: Bench = {
            batteries: [],
            port: port.value
        }

        const result = await commands.setState(bench, battery, selectedState.value)

        if (result.status === 'ok') {
            stateChangeResult.value = result.data || 'State changed successfully'
            changeTimestamp.value = new Date().toLocaleString()

            toast('Success!', {
                description: `Battery ${batteryId.value} state changed to ${selectedState.value}`,
            })
        } else {
            errorMessage.value = result.error || 'Unknown error occurred'
            toast('Error', {
                description: result.error || 'Failed to change battery state',
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