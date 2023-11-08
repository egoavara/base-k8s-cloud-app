import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createVuetify } from 'vuetify'

import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import * as labs from 'vuetify/labs/components'

import { md, aliases as mdAliases } from 'vuetify/iconsets/md'
import { mdi, aliases as mdiAliases } from 'vuetify/iconsets/mdi-svg'

import App from './App.vue'
import router from './router'

import './assets/main.css'
import 'vuetify/dist/vuetify.min.css'
import '@mdi/font/css/materialdesignicons.min.css'

const app = createApp(App)
const vuetify = createVuetify({
    aliases: {
        PageFeatureChip: components.VChip,
        NewInChip: components.VChip,
        SettingsSwitch: components.VSwitch
    },
    components: {
        ...components,
        ...labs
    },
    directives,
    defaults: {
        global: {
            eager: false
        },
        PageFeatureChip: {
            variant: 'tonal',
            border: true,
            class: 'text-medium-emphasis me-2 mb-2',
            size: 'small'
        },
        NewInChip: {
            appendIcon: 'mdi-page-next',
            class: 'ms-2 text-mono',
            color: 'success',
            label: true,
            size: 'small',
            tag: 'div',
            variant: 'flat',

            VIcon: {
                class: 'ms-2',
                size: 'small'
            }
        },
        SettingsSwitch: {
            class: 'ps-1 mb-2',
            color: 'primary',
            density: 'compact',
            inset: true,
            trueIcon: 'mdi-check',
            falseIcon: '$close'
        }
    },
    locale: {
        locale: 'en'
    },
    icons: {
        defaultSet: 'mdi',
        aliases: {
            ...mdAliases,
            ...mdiAliases
        },
        sets: {
            mdi,
            md
        }
    },
    theme: {
        defaultTheme:'dark',
        themes: {
            light: {
                colors: {
                    primary: '#1867c0',
                    secondary: '#5CBBF6',
                    tertiary: '#E57373',
                    accent: '#005CAF',
                    quarternary: '#B0D1E8',
                    'surface-bright': '#fafafa'
                }
            },
            dark: {
                colors: {
                    primary: '#2196F3',
                    secondary: '#424242',
                    tertiary: '#E57373',
                    accent: '#FF4081',
                    quarternary: '#B0D1E8',
                    'surface-bright': '#474747'
                }
            },
            blackguard: {
                dark: true,
                colors: {
                    background: '#0f0c24',
                    primary: '#e7810d',
                    surface: '#1e184a',
                    'on-surface-variant': '#4c219e',
                    info: '#9c27b0',
                    accent: '#FF4081',
                    success: '#84b38a',
                    'surface-bright': '#362b89'
                },
                variables: {
                    'theme-code': '#15123d'
                }
            }
        }
    }
})

app.use(createPinia())
app.use(router)
app.use(vuetify)

app.mount('#app')
