import { createWebHistory, createRouter } from 'vue-router'

import IndexView from '../views/Index.vue'
import AboutView from '../views/About.vue'
import LoginView from '../views/Login.vue'
import RegisterView from '../views/Register.vue'
import PostListView from '../views/post/List.vue'
import PostDetailView from '../views/post/Detail.vue'


const routes = [
  {
    path: '/',
    name: 'home',
    component: IndexView
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
