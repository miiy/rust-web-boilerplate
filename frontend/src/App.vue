<script setup>
import { RouterView } from 'vue-router'
import Navbar from './components/Navbar.vue'
</script>

<template>
  <header>
    <Navbar />
  </header>
  <div class="main-container container">
    <RouterView />
  </div>
</template>

<style scoped>

</style>
