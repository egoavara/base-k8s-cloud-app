<template>
    <v-row justify="center">
        <v-dialog v-model="uiState.isShowLogin" width="400">
            <v-card class="pa-1">
                <v-form @submit.prevent="loginFlow">
                    <v-card-title class="text-center">
                        <v-icon :icon="mdiLockOutline"></v-icon>
                    </v-card-title>
                    <v-card-text>
                        <v-container>
                            <v-row>
                                <v-col cols="12">
                                    <v-text-field label="ID" required v-model="username"></v-text-field>
                                </v-col>
                                <v-col cols="12">
                                    <v-text-field label="Password*" type="password" required
                                        v-model="password"></v-text-field>
                                </v-col>
                            </v-row>
                        </v-container>
                        <small>*해당 항목은 필수 필드입니다.</small>
                    </v-card-text>
                    <v-card-actions>
                        <v-spacer></v-spacer>
                        <v-btn color="blue-darken-1" variant="text" @click="uiState.isShowLogin = false">
                            Close
                        </v-btn>
                        <v-btn color="blue-darken-1" variant="tonal" type="submit">
                            Login
                        </v-btn>
                    </v-card-actions>
                </v-form>
            </v-card>
        </v-dialog>
    </v-row>
</template>

<script setup lang="ts">
import { useUiState } from "@/stores/ui-states";
import { useUserStore } from "@/stores/user";
import { mdiLockOutline } from "@mdi/js";
import { storeToRefs } from "pinia";
import { watch } from "vue";
import { ref } from "vue";


const uiState = useUiState();
const uiStateRef = storeToRefs(uiState);

const userStore = useUserStore();

const username = ref("");
const password = ref("");

function loginFlow() {
    userStore.login(username.value, password.value);
}

watch(uiStateRef.isShowLogin, (value) => {
    if (value) {
        username.value = "";
        password.value = "";
    }
});

</script>