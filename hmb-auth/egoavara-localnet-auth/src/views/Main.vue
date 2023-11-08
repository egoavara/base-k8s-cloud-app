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
                                    <v-card-title>로그인</v-card-title>
                                    <v-card-item>
                                        <v-btn :href="`/flow/login-start?redirect_url=${encodeURIComponent('https://auth.egoavara.net/login-complet2')}`">로그인</v-btn>
                                        <!-- <v-btn :disabled="isNotPrepared" :href="state?.url ?? ''">로그인</v-btn> -->
                                    </v-card-item>
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
</script>