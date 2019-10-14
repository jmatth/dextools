import Vue from 'vue';
import Vuex from 'vuex';
import Agenda from './models/agenda';
import Event from './models/event';
import axios from 'axios';
import {
    Stitch,
    RemoteMongoClient,
    AnonymousCredential,
} from 'mongodb-stitch-browser-sdk';

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
    stitchClient: null,
    visitId: '',
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
    setUserName(state: any, userName: string) {
      state.userName = userName;
      localStorage.userName = userName;
    },
    setClient(state: any, client: any) {
      state.stitchClient = client;
    },
    setVisitId(state: any, newId: string) {
      state.visitId = newId;
    },
    setFeedbackUrl(state: any, newUrl: string) {
      state.feedbackUrl = newUrl;
    },
  },
  actions: {
    loadSettings(context) {
      const client = Stitch.initializeDefaultAppClient('dextools-crjjz');
      const creds = new AnonymousCredential();
      const stitchPromise = client.auth.loginWithCredential(new AnonymousCredential())
        .then((user: any) => {
          context.commit('setClient', client);
          return client.callFunction('startVisit', [{ referrer: document.referrer }]);
        })
        .then((result: any) => context.commit('setVisitId', result));

      const axiosPromise = axios.get('/settings.json').then((response: any) => {
        const settings = response.data;
        context.commit('setConName', settings.conName);
        context.commit('setConEmail', settings.conEmail);
        context.commit('setFeedbackUrl', settings.feedbackUrl);
        const schedule = scheduleJsonToEvents(settings.schedule);
        context.commit('setSchedule', schedule);
      });
      return Promise.all([stitchPromise, axiosPromise]);
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
