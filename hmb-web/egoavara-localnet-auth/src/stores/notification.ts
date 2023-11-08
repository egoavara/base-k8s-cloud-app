import { defineStore, storeToRefs } from 'pinia'
import { ref, watch } from 'vue'
import { computed } from 'vue'
import { useUserStore } from './user'

export type MessageSeverity = 'error' | 'success' | 'warning' | 'info'
export type Message = {
    content: string
    summary: string
    severity: MessageSeverity
    isRead: boolean
}
const useNotificationStoreKey = '$.noti'
export const useNotificationStore = defineStore(useNotificationStoreKey, {
    state: () => {
        const user = storeToRefs(useUserStore())
        const messages = ref<Message[]>([])
        const count = computed(() => {
            return messages.value.length
        })
        const level = computed(() => {
            return messages.value.reduce(
                (prev, current) => {
                    if (prev === 'error') {
                        return prev
                    }
                    if (current.severity === 'error') {
                        return current.severity
                    }
                    if (prev === 'warning') {
                        return prev
                    }
                    if (current.severity === 'warning') {
                        return current.severity
                    }
                    if (prev === 'success') {
                        return prev
                    }
                    if (current.severity === 'success') {
                        return current.severity
                    }
                    if (prev === 'info') {
                        return prev
                    }
                    if (current.severity === 'info') {
                        return current.severity
                    }
                    return prev
                },
                null as MessageSeverity | null
            )
        })

        watch(user.isLogin, (current) => {
            if (!current) {
                messages.value = []
            }
        })

        return {
            messages,
            count,
            level
        }
    },
    actions: {
        clear() {
            this.messages = []
        },
        insert(...message: Message[]) {
            this.messages.push(...message)
        }
    }
})
