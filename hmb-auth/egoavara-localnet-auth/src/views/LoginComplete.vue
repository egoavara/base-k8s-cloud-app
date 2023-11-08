<template></template>

<script setup lang="ts">
import { useRoute } from "vue-router";
import { onMounted } from "vue";

const route = useRoute()

onMounted(() => {
    console.log("mounted")
    fetch(`https://auth.egoavara.net/flow/login-complete?code=${route.query.code}&state=${route.query.state}&scope=${route.query.scope}`, {

        headers: {
            'Content-Type': 'application/json'
        },
        credentials: 'include',
    }).then(res => res.json()).then(res => {
        console.log(res)
    })
})
</script>
