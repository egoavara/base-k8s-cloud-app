import { createRouter, createWebHistory } from 'vue-router'
import Login from '@/views/Login.vue'
import Forget from '@/views/Forget.vue'

const route = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/ui/login',
            name: 'login',
            component: Login
        },
        {
            path: '/ui/forget',
            name: 'forget',
            component: Forget
        }
    ]
})
export default route
