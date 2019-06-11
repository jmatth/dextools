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
        const schedule = scheduleJsonToEvents(settings.schedule);
        context.commit('setSchedule', schedule);
      });
    },
  },
});

function scheduleJsonToEvents(jsonObject: any): any {
  const scheduleObject = jsonObject.reduce((acc: any, event: any) => {
    acc[event.code] = event;
    return acc;
  }, {});
  Object.values(scheduleObject).forEach((event: any) => {
    const matches = event.description.match(/See [A-Z][0-9]{3}.$/g);
    if (matches) {
      const dereferencedCode = matches[matches.length - 1].substring(4, 8);
      const dereferencedEvent = scheduleObject[dereferencedCode];
      if (!dereferencedEvent) {
        console.error(`Tried to dereference non-existent event ${dereferencedCode}.`);
        return;
      }
      event.description += ` ${dereferencedEvent.description}`;
    }
  });
  return Object.assign({}, ...(Object.values(scheduleObject).map((e: any) => ({ [e.code]: new Event(e) }))));
}

export default store;
