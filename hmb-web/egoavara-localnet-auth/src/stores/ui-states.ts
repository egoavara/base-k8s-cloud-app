import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
const useUiStateKey = '$.ui'
export const useUiState = defineStore(useUiStateKey, {
    state: () => {
        const isShowConfiguration = ref(false)
        const isShowLogin = ref(false)
        return {
            isShowConfiguration,
            isShowLogin
        }
    }
})
