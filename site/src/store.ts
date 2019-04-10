import Vue from 'vue';
import Vuex from 'vuex';
import Agenda from './models/agenda';
import Event from './models/event';
import axios from 'axios';

interface Schedule {
  [key: string]: Event;
}

Vue.use(Vuex);

const store = new Vuex.Store({
  state: {
    agenda: new Agenda(),
    schedule: {},
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
    setSchedule(state: any, schedule: Schedule) {
      state.schedule = schedule;
    },
  },
  actions: {
    loadSchedule(context) {
      axios.get('/schedule.json').then((response: any) => {
        const schedule = Object.assign({}, ...(response.data.map((e: any) => ({ [e.code]: new Event(e) }))));
        context.commit('setSchedule', schedule);
      });
    },
  },
});

export default store;

store.dispatch('loadSchedule');
