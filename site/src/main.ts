import Vue from 'vue';
import './plugins/vuetify';
import Event from './models/event';
import App from './App.vue';

Vue.config.productionTip = false;

new Vue({
  render: (h) => h(App),
}).$mount('#app');
