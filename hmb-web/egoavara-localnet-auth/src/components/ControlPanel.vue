<template>
    <v-toolbar>
        <v-spacer />
        <template v-if="user.isLogin">
            <v-btn id="notification-button" icon class="ml-2">
                <!-- @click.stop="notification.insert({ isRead: false, severity: 'info', content: 'hello, world', summary: 'hello' })"> -->
                <v-badge :dot="display.notificationStyle === 'dot'" :content="notification.count"
                    :model-value="notification.count > 0" :color="notificationColor" :max="display.notificationMax">
                    <v-icon ref="notificationElement" class="jiggle" :icon="notificationIcon" />
                </v-badge>
            </v-btn>
            <v-btn icon class="ml-2">
                <v-icon icon="md:settings" @click="onConfigButtonClick" />
            </v-btn>
            <v-btn icon class="ml-2" @click.stop="user.logout()">
                <v-icon icon="md:logout" />
            </v-btn>
            <NotificationPopup activator="#notification-button" />
        </template>
        <template v-else>
            <v-btn icon class="ml-2" @click.stop="uiState.isShowLogin=true">
                <v-icon icon="md:login" />
            </v-btn>
        </template>
    </v-toolbar>
</template>

<style scoped>
.jiggle {
    animation: icon-shaking 0.2s ease-in-out normal forwards paused;
}
</style>

<script setup lang="ts">
import { useUserStore } from "@/stores/user";
import { useNotificationStore } from "@/stores/notification";
import { useDisplayConfig } from "@/stores/display-config";
import { watch, ref, onMounted } from "vue";
import { storeToRefs } from "pinia";
import type { VIcon } from "vuetify/components";
import { mdiBellOutline, mdiBellRingOutline } from '@mdi/js';
import { useUiState } from "@/stores/ui-states";
import NotificationPopup from "./NotificationPopup.vue";

const notificationElement = ref<VIcon | null>(null)

const user = useUserStore()
const notification = useNotificationStore()
const display = useDisplayConfig()
const uiState = useUiState()

const notificationRefs = storeToRefs(notification)

const notificationIcon = ref(mdiBellOutline)
const notificationColor = ref("primary")

function onConfigButtonClick() {
    uiState.isShowConfiguration = !uiState.isShowConfiguration
}

watch(notificationRefs.level, (value) => {
    switch (value) {
        case "error":
            notificationColor.value = "error"
            break
        case "warning":
            notificationColor.value = "warning"
            break
        case "info":
            notificationColor.value = "info"
            break
        case "success":
            notificationColor.value = "success"
            break
    }
})

watch(notificationRefs.count, () => {
    notificationElement
        .value
        ?.$el
        ?.getAnimations()
        ?.forEach((animation: CSSAnimation) => {
            animation.cancel()
            animation.play()
            notificationIcon.value = mdiBellRingOutline
        })
})

onMounted(() => {
    notificationElement
        .value
        ?.$el
        ?.getAnimations()
        ?.forEach((animation: CSSAnimation) => {
            animation.onremove = () => {
                notificationIcon.value = mdiBellOutline
            }
            animation.onfinish = () => {
                notificationIcon.value = mdiBellOutline
            }
            animation.onremove = () => {
                notificationIcon.value = mdiBellOutline
            }
        })
})

</script>