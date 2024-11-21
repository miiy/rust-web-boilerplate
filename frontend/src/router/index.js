import { createWebHistory, createRouter } from 'vue-router'

import HomeView from '../views/Home.vue'
import AboutView from '../views/About.vue'
import LoginView from '../views/auth/Login.vue'
import RegisterView from '../views/auth/Register.vue'
import PostListView from '../views/post/List.vue'
import PostDetailView from '../views/post/Detail.vue'


const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/about',
    name: 'about',
    component: AboutView
  },
  {
    path: '/login',
    name: 'login',
    component: LoginView
  },
  {
    path: '/register',
    name: 'register',
    component: RegisterView
  },
  {
    path: '/posts',
    name: 'post_list',
    component: PostListView
  },
  {
    path: '/posts/:id',
    name: 'post_detial',
    component: PostDetailView
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes: routes,
})

export default router
