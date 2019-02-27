import Vue from 'vue';
import './plugins/vuetify';
import Event from './models/event';
import App from './App.vue';
import moment from 'moment';
import 'moment-timezone';
import './registerServiceWorker';

moment.tz.setDefault('America/New_York');

Vue.config.productionTip = false;

new Vue({
  render: (h) => h(App),
}).$mount('#app');
