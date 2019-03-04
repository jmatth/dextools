import Vue from 'vue';
import Vuex from 'vuex';
import Schedule from './models/schedule';
import Event from './models/event';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    schedule: new Schedule(),
  },
  mutations: {
    addEventToSchedule(state: any, event: Event) {
      state.schedule.addEvent(event);
    },
  },
  actions: {
  },
});
