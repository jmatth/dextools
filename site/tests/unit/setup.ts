import Vue from 'vue';
import Vuetify from 'vuetify';
import VueClipboard from 'vue-clipboard2';

Vue.use(Vuetify);
Vue.use(VueClipboard);

declare const global: any;
global.requestAnimationFrame = (cb: any) => cb();
