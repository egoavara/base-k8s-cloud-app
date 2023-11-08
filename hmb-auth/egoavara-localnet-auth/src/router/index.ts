import { createRouter, createWebHistory } from 'vue-router'
import Main from '@/views/Main.vue'
import Login from '@/views/Login.vue'
import LoginComplete from '@/views/LoginComplete.vue'
import Forget from '@/views/Forget.vue'

const route = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'main',
            component: Main
        },
        {
            path: '/login',
            name: 'login',
            component: Login
        },
        {
            path: '/login-complete',
            name: 'login-complete',
            component: LoginComplete
        },
        {
            path: '/login-complet2',
            name: 'login-complete',
            component: LoginComplete
        },
        {
            path: '/forget',
            name: 'forget',
            component: Forget
        }
    ]
})
export default route
