import Vue from 'vue';
import Vuetify from 'vuetify';
import log from 'loglevel';

Vue.use(Vuetify);

declare const global: any;
global.requestAnimationFrame = (cb: any) => cb();
global.localStorage = window.localStorage;

log.setLevel(log.levels.SILENT);
