<template>
    <v-layout class="h-screen w-screen rounded rounded-md background ">
        <v-app-bar id="noti" name="app-bar">
            <div class="w-100 h-100 ma-0 pa-0 d-flex align-center justify-center">
                <div class="w-100 h-100 ma-0 px-2 py-0 d-flex align-center app-bar-row">
                    <v-row no-gutters>
                        <v-col cols="8"></v-col>
                        <v-spacer />
                        <v-col cols="4">
                            <div class="d-flex justify-end">
                                <ToggleTheme />
                            </div>
                        </v-col>
                    </v-row>
                </div>
            </div>
        </v-app-bar>
        <v-main class="h-100 d-flex align-center justify-center">
            <div class="blur">
                <v-container fluid class="h-100">
                    <v-row style="height: 15%;"></v-row>
                    <v-row align="center" justify="center">
                        <v-col sx="10" sm="8" md="6" xl="4" xxl="3">
                            <v-card elevation="8" max-width="600" rounded="lg" class="pa-2">
                                <v-form @submit.prevent>
                                    <v-card-title>아이고~ 아쉬워라</v-card-title>
                                    <v-card-item>

                                        <div class="text-subtitle-1 text-medium-emphasis">사용자명</div>
                                        <v-text-field class="mx-2 mt-2" v-model="username" placeholder="사용자명"
                                            variant="outlined" density="compact" type="text"
                                            :prepend-inner-icon="mdiAccount" />

                                        <div
                                            class="text-subtitle-1 text-medium-emphasis d-flex align-center justify-space-between">
                                            비밀번호
                                            <router-link class="text-caption text-decoration-none text-blue"
                                                to="login">알아서 찾아요?</router-link>
                                        </div>

                                        <v-text-field class="mx-2 mt-2" v-model="password" ref="passwordForm"
                                            placeholder="비밀번호" variant="outlined" density="compact"
                                            :type="passwordVisible ? 'text' : 'password'"
                                            :prepend-inner-icon="mdiLockOutline"
                                            :append-inner-icon="passwordVisible ? mdiEyeOff : mdiEye"
                                            @click:append-inner="changePasswordVisible" />
                                    </v-card-item>
                                    <v-card-actions class="justify-end">
                                        <v-btn><v-icon class="mr-2" :icon="mdiLogin" />Login</v-btn>
                                    </v-card-actions>
                                </v-form>
                            </v-card>
                        </v-col>
                    </v-row>
                </v-container>
            </div>
        </v-main>
    </v-layout>
</template>

<style scoped>
.background {
    background: url(../assets/wallpaper.jpg) no-repeat center center fixed;
    background-size: cover;
}

.blur {
    width: 100%;
    height: 100%;
    background: rgba(var(--v-theme-background), var(--v-disabled-opacity));
    backdrop-filter: blur(5px);
    -webkit-backdrop-filter: blur(5px);
}
</style>

<script setup lang="ts">
import ToggleTheme from '@/components/button/ToggleTheme.vue'
import { mdiAccount, mdiLockOutline, mdiLogin, mdiEye, mdiEyeOff } from '@mdi/js';
import { useRoute } from 'vue-router';
import { ref } from 'vue';
import type { VTextField } from 'vuetify/components';


const route = useRoute()

const passwordForm = ref<VTextField | null>()

const username = ref('')
const password = ref('')
const passwordVisible = ref(false)

function changePasswordVisible() {
    passwordVisible.value = !passwordVisible.value
    setTimeout(() => {
        const passwordFormEl = passwordForm.value?.$el.querySelector('input')
        passwordFormEl?.setSelectionRange(passwordFormEl?.value.length, passwordFormEl?.value.length)
    }, 1)
}
</script>
