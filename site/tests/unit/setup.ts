import Vue from 'vue';
import Vuetify from 'vuetify';
import log from 'loglevel';

Vue.use(Vuetify);

// TODO: Remove this when vue and/or vuetify figure themselves out.
const ignoreWarnMessage = 'The .native modifier for v-on is only valid on components but it was used on <div>.';
Vue.config.warnHandler = function(msg, vm, trace) {
  if (msg === ignoreWarnMessage) {
    // @ts-ignore
    msg = null;
    // @ts-ignore
    vm = null;
    // @ts-ignore
    trace = null;
  }
};

declare const global: any;
global.requestAnimationFrame = (cb: any) => cb();
global.localStorage = window.localStorage;

log.setLevel(log.levels.SILENT);
