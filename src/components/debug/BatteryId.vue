<template>
    <div class="space-y-4 px-4">
        <BatterySelection @port-selected="onPortSelected" :show-battery-id="false" />

        <Button @click="onAssignIdClick" :disabled="!port || isLoading" class="w-full">
            <span v-if="isLoading">Assigning ID...</span>
            <span v-else>Assign New Battery ID</span>
        </Button>

        <!-- Display Assigned ID Result -->
        <Card v-if="assignedId !== null" class="w-full">
            <CardHeader>
                <CardTitle>Battery ID Assigned</CardTitle>
                <CardDescription>
                    Successfully assigned ID to battery on Port: {{ port }}
                </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
                <div class="text-center">
                    <div class="text-4xl font-bold text-primary mb-2">{{ assignedId }}</div>
                    <div class="text-sm text-muted-foreground">New Battery ID</div>
                </div>

                <div class="bg-muted/50 p-4 rounded-lg space-y-2">
                    <h4 class="font-medium">Assignment Details</h4>
                    <div class="text-sm space-y-1">
                        <div><strong>Port:</strong> {{ port }}</div>
                        <div><strong>Assigned ID:</strong> {{ assignedId }}</div>
                        <div><strong>Timestamp:</strong> {{ assignmentTimestamp }}</div>
                    </div>
                </div>
            </CardContent>
        </Card>

        <!-- Display Error Message -->
        <Card v-if="errorMessage && assignedId === null" class="w-full border-destructive">
            <CardHeader>
                <CardTitle class="text-destructive">Assignment Failed</CardTitle>
            </CardHeader>
            <CardContent>
                <div class="space-y-2">
                    <p class="text-sm text-destructive">{{ errorMessage }}</p>
                    <div class="text-xs text-muted-foreground">
                        <strong>Port:</strong> {{ port || 'Not selected' }}
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
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { type Bench, commands } from '@/bindings'

const port = ref<string>()
const isLoading = ref(false)
const assignedId = ref<number | null>(null)
const errorMessage = ref<string>('')
const assignmentTimestamp = ref<string>('')

const onPortSelected = (selectedPort: string) => {
    port.value = selectedPort
    // Clear previous data when port changes
    assignedId.value = null
    errorMessage.value = ''
    assignmentTimestamp.value = ''
}

const onAssignIdClick = async () => {
    if (!port.value) {
        toast('Error', {
            description: 'Please select a COM port first.',
        })
        return
    }

    isLoading.value = true
    assignedId.value = null
    errorMessage.value = ''
    assignmentTimestamp.value = ''

    try {
        const bench: Bench = {
            batteries: [],
            port: port.value
        }

        const result = await commands.assignId(bench)

        if (result.status === 'ok') {
            assignedId.value = result.data
            assignmentTimestamp.value = new Date().toLocaleString()

            toast('Success!', {
                description: `Battery ID ${result.data} assigned successfully on ${port.value}`,
            })
        } else {
            errorMessage.value = result.error || 'Unknown error occurred'
            toast('Error', {
                description: result.error || 'Failed to assign battery ID',
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