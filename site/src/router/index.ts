import Vue from 'vue';
import VueRouter from 'vue-router';
import Schedule from '@/views/Schedule.vue';
import Calendar from '@/views/Calendar.vue';
import Export from '@/views/Export.vue';

Vue.use(VueRouter);

const routes = [
  {
    path: '/',
    name: 'schedule',
    component: Schedule,
  },
  {
    path: '/calendar',
    name: 'calendar',
    component: Calendar,
  },
  {
    path: '/export',
    name: 'export',
    component: Export,
  },
  // {
  //   path: '/about',
  //   name: 'about',
  //   // route level code-splitting
  //   // this generates a separate chunk (about.[hash].js) for this route
  //   // which is lazy-loaded when the route is visited.
  //   component: () => import(/* webpackChunkName: "about" */ '../views/About.vue'),
  // },
];

const router = new VueRouter({
  routes,
});

export default router;
