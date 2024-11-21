<script setup>
import { RouterView } from 'vue-router'
import Navbar from './components/Navbar.vue'
import Footer from './components/Footer.vue'
</script>

<template>
  <header>
    <Navbar />
  </header>
  <div class="main-container container">
    <RouterView />
  </div>
  <Footer />
</template>

<style scoped>

</style>
