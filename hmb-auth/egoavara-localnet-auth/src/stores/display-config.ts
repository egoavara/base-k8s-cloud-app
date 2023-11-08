import { useLocalStorage, type RemovableRef } from '@vueuse/core'
import { defineStore } from 'pinia'

const useDisplayConfigKey = '$.display'
export type NotificationStyle = 'dot' | 'count'
export const useDisplayConfig = defineStore(useDisplayConfigKey, {
    state: () => {
        const notificationStyle: RemovableRef<'dot' | 'count'> = useLocalStorage<'dot' | 'count'>(`${useDisplayConfigKey}.notificationStyle`, 'count')
        const notificationMax: RemovableRef<number> = useLocalStorage<number>(`${useDisplayConfigKey}.notificationMax`, 99)
        return {
            notificationStyle,
            notificationMax
        }
    }
})
