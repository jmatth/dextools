import Vue from 'vue';
import Vuex from 'vuex';
import Agenda from './models/agenda';
import Event from './models/event';
import scheduleJson from './schedule.json';

const schedule: { [index: string]: Event } = {};
scheduleJson.forEach((e: any) => {
  schedule[e.code] = new Event(e);
});

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    agenda: new Agenda(),
    schedule,
    userName: '',
    conName: 'Dreamation',
    conEmail: 'josh@jmatth.com',
  },
  mutations: {
    addEventToAgenda(state: any, code: string): void {
      state.agenda.addEvent(state.schedule[code]);
    },
    removeEventFromAgenda(state: any, code: string) {
      state.agenda.removeEvent(code);
    },
  },
  actions: {
  },
});
