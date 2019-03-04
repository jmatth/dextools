import Vue from 'vue';
import Vuex from 'vuex';
import Agenda from './models/agenda';
import Event from './models/event';

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    agenda: new Agenda(),
    userName: '',
    conName: 'Dreamation',
    conEmail: 'josh@jmatth.com',
  },
  mutations: {
    addEventToAgenda(state: any, event: Event) {
      state.agenda.addEvent(event);
    },
    removeEventFromAgenda(state: any, code: string) {
      state.agenda.removeEvent(code);
    },
  },
  actions: {
  },
});
