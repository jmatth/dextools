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
    conName: '',
    conEmail: '',
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
    setConName(state: any, conName: string) {
      state.conName = conName;
    },
    setConEmail(state: any, conEmail: string) {
      state.conEmail = conEmail;
    },
  },
  actions: {
    loadSettings(context) {
      return axios.get('/settings.json').then((response: any) => {
        const settings = response.data;
        context.commit('setConName', settings.conName);
        context.commit('setConEmail', settings.conEmail);
        const schedule = Object.assign({}, ...(response.data.schedule.map((e: any) => ({ [e.code]: new Event(e) }))));
        context.commit('setSchedule', schedule);
      });
    },
  },
});

export default store;
