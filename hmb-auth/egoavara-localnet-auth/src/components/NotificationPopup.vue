<template>
    <v-menu :activator="props.activator" :close-on-content-click="false" eager min-width="300px">
        <v-card width="400">
            <v-card-text>
                <div class="font-weight-bold ms-1 mb-2">
                    Today
                </div>

                <v-timeline density="compact" align="start">
                    <v-timeline-item v-for="message in messages" :key="message.time" :dot-color="message.color"
                        size="x-small">
                        <div class="mb-4">
                            <div class="font-weight-normal">
                                <strong>{{ message.from }}</strong> @{{ message.time }}
                            </div>
                            <div>{{ message.message }}</div>
                        </div>
                    </v-timeline-item>
                </v-timeline>
            </v-card-text>
        </v-card>
    </v-menu>
</template>

<script setup lang="ts">
import { ref, defineProps } from "vue";
import { useUiState } from "@/stores/ui-states";
import type { VMenu } from "vuetify/components";

const props = defineProps({
    activator: {
        type: String,
        default: ""
    }
})

const uiState = useUiState()

const messages = ref([
    {
        from: 'You',
        message: `Sure, I'll see you later.`,
        time: '10:42am',
        color: 'deep-purple-lighten-1',
    },
    {
        from: 'John Doe',
        message: 'Yeah, sure. Does 1:00pm work?',
        time: '10:37am',
        color: 'green',
    },
    {
        from: 'You',
        message: 'Did you still want to grab lunch today?',
        time: '9:47am',
        color: 'deep-purple-lighten-1',
    },
])

</script>