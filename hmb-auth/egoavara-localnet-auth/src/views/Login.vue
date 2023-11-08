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
                                <v-form ref="form">
                                    <v-card-title>로그인</v-card-title>
                                    <v-card-item>
                                        <div class="text-subtitle-1 text-medium-emphasis">사용자명</div>
                                        <v-text-field name="identifier" class="mx-2 mt-2" ref="identifierForm" placeholder="사용자명"
                                            variant="outlined" density="compact" type="text"
                                            :prepend-inner-icon="mdiAccount" />
                                        <div
                                            class="text-subtitle-1 text-medium-emphasis d-flex align-center justify-space-between">
                                            비밀번호
                                            <router-link class="text-caption text-decoration-none text-blue"
                                                to="forget">비밀번호를 잊어버렸나요?</router-link>
                                        </div>

                                        <v-text-field name="password" class="mx-2 mt-2" ref="passwordForm" placeholder="비밀번호"
                                            variant="outlined" density="compact" :type="passwordType"
                                            :prepend-inner-icon="mdiLockOutline" :append-inner-icon="passwordIcon"
                                            @click:append-inner="changePasswordVisible" />
                                        <div v-if="isExpiresWarning" class="text-caption text-right">로그인 세션 만료까지 {{
                                            expiresAgo }}</div>
                                    </v-card-item>
                                    <v-card-actions class="justify-end">
                                        <v-btn v-if="isFinished" type="submit" @click="login">
                                            <v-icon class="mr-2" :icon="mdiLogin" />Login
                                        </v-btn>
                                        <v-progress-circular v-else indeterminate />
                                    </v-card-actions>

                                    <template v-for="elem in dynamicNodes">
                                        <input class="hidden" :type="elem.type" :value="elem.attributes.value" :name="elem.attributes.name" />
                                    </template>
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

.hidden {
    display: none;
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
import { ref, watch, computed } from 'vue';
import type { VForm, VTextField } from 'vuetify/components';
import { computedEager, extendRef, refDefault, useFetch, useTimeAgo, useTimeoutFn, useTimestamp, whenever } from '@vueuse/core';
import dayjs from 'dayjs'

// 시스템 정보
const route = useRoute()
const now = useTimestamp()

// vue 레퍼런스
const form = ref<VForm>()
const passwordForm = ref<VTextField>()

// 패스워드 보이기 옵션 관리
const passwordVisible = ref(false) // 비밀번호 보이기 on/off
const passwordType = computed(() => passwordVisible.value ? 'text' : 'password')
const passwordIcon = computed(() => passwordVisible.value ? mdiEyeOff : mdiEye)
function changePasswordVisible() {
    passwordVisible.value = !passwordVisible.value
    setTimeout(() => {
        const passwordFormEl = passwordForm.value?.$el.querySelector('input')
        passwordFormEl.setSelectionRange(passwordFormEl?.value.length, passwordFormEl?.value.length)
    }, 1)
}

// 로그인 세션 관리
const { data, isFinished } = useFetch(`/self-service/login/flows?id=${route.query.flow}`, {
    mode: 'cors',
    headers: {
        'Accept': 'application/json',
    },
}).json()

// 에러가 난 경우에는 만료일이 지금 당장이 됨
const expiresAt = computed(() => dayjs(data.value?.expires_at))
const expiresDuration = computed(() => expiresAt.value.diff(now.value))
const isExpiresWarning = computedEager<boolean>(() => dayjs.duration(expiresDuration.value).asMinutes() <= 3)// 3분부터 경고
const expiresAgo = computed(() => dayjs.duration(expiresDuration.value).humanize(true))

function expiresSession() {
    alert('로그인 세션이 만료되었습니다.')
    window.location.href = '/flow/login-start'
}

const { start } = useTimeoutFn(expiresSession, expiresDuration, { immediate: false })
const dynamicNodes = computed<any[]>(() => data.value?.ui?.nodes?.filter((v: any) => !(['identifier', 'password'].includes(v.attributes.name))) ?? [])

whenever(isFinished, () => {
    start()
    if (form.value == null || passwordForm.value == null) {
        expiresSession()
        return
    }
    if (data.value !== null) {
        form.value.method = data.value.ui.method
        form.value.action = data.value.ui.action
    }
})

// 로그인 기능 관리
async function login() {
    // setup formdata

    console.log(data.value)
}

</script>
