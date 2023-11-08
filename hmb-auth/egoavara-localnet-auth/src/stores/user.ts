import { useFetch, useLocalStorage } from '@vueuse/core'
import { defineStore } from 'pinia'
import { watch } from 'vue'
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const useUserStoreKey = '$.user'
export const useUserStore = defineStore(useUserStoreKey, {
    state: () => {
        const username = useLocalStorage<string | null>(`${useUserStoreKey}.username`, null)
        const scopes = useLocalStorage<Array<string>>(`${useUserStoreKey}.roles`, [])
        const token = useLocalStorage<string | null>(`${useUserStoreKey}.token`, null)
        const isLogin = computed(() => {
            return token.value !== null
        })
        return {
            username,
            scopes,
            token,
            isLogin
        }
    },
    actions: {
        async login(username: string, password: string) {
            const { data } = await useFetch<any>(
                import.meta.env.VITE_URL_LOGIN_REQUEST,
                {
                    headers: {
                        Accept: 'application/json'
                    }
                },
                {}
            ).json()
            const flowId = data.value.id
            const csrfToken = data.value.ui.nodes.find((node: any) => node?.attributes?.name === 'csrf_token')?.attributes?.value

            const { data: loginResult } = await useFetch(
                `https://auth.egoavara.net/self-service/login?flow=${flowId}`,
                {
                    method: 'POST',
                    headers: {
                        Accept: 'application/json',
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({
                        csrf_token: csrfToken,
                        identifier: username,
                        method: 'password',
                        password: password
                    })
                },
                {}
            ).json()
            console.log(loginResult)
            return ''
        },
        async logout() {
            this.username = null
            this.token = null
            this.scopes = []
        }
    }
})
