<script setup lang="ts">
import { useUiState } from "@/stores/ui-states"

const uiState = useUiState()

</script>

<template>
    <v-navigation-drawer v-model="uiState.isShowConfiguration" location="right" >
        <v-list>
            <v-list-item title="hello1" subtitle="world"></v-list-item>
            <v-list-item title="hello2" subtitle="world"></v-list-item>
            <v-list-item title="hello3" subtitle="world"></v-list-item>
            <v-list-item title="hello4" subtitle="world"></v-list-item>
        </v-list>
    </v-navigation-drawer>
    <v-navigation-drawer color="grey-darken-2" permanent name="drawer">
        <div class="d-flex justify-center align-center h-100">
        </div>
    </v-navigation-drawer>
</template>
