import Vue from 'vue';
import Vuetify from 'vuetify';

Vue.use(Vuetify);

declare const global: any;
global.requestAnimationFrame = (cb: any) => cb();
