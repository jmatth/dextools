import Vue from 'vue';
import '@/plugins/virtual-scroller';
import '@/plugins/clipboard2';
import '@/plugins/observe-visibility';
import Event from './models/event';
import App from './App.vue';
import moment from 'moment';
import 'moment-timezone';
import log from 'loglevel';
import '@/registerServiceWorker';
import store from '@/store';
import vuetify from '@/plugins/vuetify';
import router from '@/router';

moment.tz.setDefault('America/New_York');

log.setLevel(process.env.NODE_ENV === 'production' ? log.levels.INFO : log.levels.DEBUG, false);

Vue.config.productionTip = false;

new Vue({
  store,
  // @ts-ignore
  vuetify,
  // @ts-ignore
  router,
  render: (h) => h(App),
}).$mount('#app');
