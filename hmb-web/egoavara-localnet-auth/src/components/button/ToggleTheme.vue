<template>
    <v-btn :icon="darkButtonIcon" density="default" @click="toggleTheme" />
</template>

<script setup lang="ts">

import { computed } from "vue"
import { mdiLightbulb, mdiLightbulbOutline } from "@mdi/js"
import { useTheme } from "vuetify";

const theme = useTheme()
const darkButtonIcon = computed(() => theme.name.value === "dark" ? mdiLightbulb : mdiLightbulbOutline)

function toggleTheme() {
    if (theme.global.name.value === "dark") {
        theme.global.name.value = "light"
    }
    else {
        theme.global.name.value = "dark"
    }
}

</script>