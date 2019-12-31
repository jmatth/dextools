import Vue from 'vue';
import './plugins/vuetify';
import './plugins/virtual-scroller';
import './plugins/clipboard2';
import Event from './models/event';
import App from './App.vue';
import moment from 'moment';
import 'moment-timezone';
import './registerServiceWorker';
import store from './store';
import vuetify from './plugins/vuetify';

moment.tz.setDefault('America/New_York');

Vue.config.productionTip = false;

new Vue({
  store,
  // @ts-ignore
  vuetify,
  render: (h) => h(App),
}).$mount('#app');
