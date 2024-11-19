<script setup>
import { RouterView } from 'vue-router'
</script>

<template>
  <div class="container">
    <div class="header">
      <nav>
        <ul>
          <li><RouterLink to="/">Home</RouterLink></li>
          <li><RouterLink to="/login">Login</RouterLink></li>
        </ul>
      </nav>
    </div>
    <main>
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.header {
  line-height: 1.5;
}

</style>
