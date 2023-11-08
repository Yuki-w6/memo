<script setup lang="ts">
// import axios from "axios";
import Home from "@/Pages/Home/Home.vue";
// import { onMounted, ref } from "vue";

// const responseData = ref("");

// onMounted(
//     axios.get()
// );
</script>

<template>
    <div>
        <h1>Response from Rust server:</h1>
        <!-- <p>{{ responseData }}</p> -->
    </div>
</template>

<style scoped>
.logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
}
.logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
    filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
