import Vue from 'vue';
import Vuex from 'vuex';
import Agenda from './models/agenda';
import Event from './models/event';
import axios from 'axios';
import log from 'loglevel';
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
    loading: true,
    agenda: new Agenda(),
    schedule: {},
    userName: '',
    conName: '',
    conEmail: '',
    isMetatopia: false,
    stitchClient: null,
    visitId: '',
  },
  mutations: {
    finishLoading(state: any) {
      state.loading = false;
    },
    addEventToAgenda(
      state: any,
      payload: { code: string, skipLastAdded: boolean },
    ): void {
      const event = state.schedule[payload.code];
      if (!event) {
        log.error(`Trying to add event ${payload.code} which does not exist in schedule.`);
        return;
      }
      state.agenda.addEvent(event, payload.skipLastAdded);
    },
    removeEventFromAgenda(state: any, code: string) {
      state.agenda.removeEvent(code);
    },
    toggleEvent(state: any, code: string) {
      const event = state.schedule[code];
      if (!event) {
        log.error(`Trying to toggle event ${code} which does not exist in schedule.`);
        return;
      }
      state.agenda.toggleEvent(state.schedule[code]);
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
    setIsMetatopia(state: any, isMetatopia: boolean) {
      state.isMetatopia = isMetatopia;
    },
  },
  actions: {
    loadSettings(context) {
      return axios.get('/settings.json').then((response: any) => {
        const settings = response.data;
        context.commit('setConName', settings.conName);
        context.commit('setConEmail', settings.conEmail);
        context.commit('setFeedbackUrl', settings.feedbackUrl);
        context.commit('setIsMetatopia', settings.isMetatopia);
        const schedule = scheduleJsonToEvents(settings.schedule);
        context.commit('setSchedule', schedule);
        return settings;
      }).then((settings: any) => {
        const stitchAppId = settings.stitchApp;
        let stitchPromise;
        if (!stitchAppId) {
          log.warn('No stitch app configured, not sending analytics.');
          stitchPromise = Promise.resolve();
        } else {
          const client = Stitch.initializeDefaultAppClient(stitchAppId);
          stitchPromise = client.auth.loginWithCredential(new AnonymousCredential())
            .then((user: any) => {
              context.commit('setClient', client);
              return client.callFunction('startVisit', [{ referrer: document.referrer }]);
            })
            .then((result: any) => context.commit('setVisitId', result));
        }

        // If the site has been updated for a new con, blow out the agenda cache.
        if (localStorage.agendaConName !== context.state.conName) {
          // tslint:disable-next-line
          log.info(`Detected convention change from ${localStorage.agendaConName} to ${context.state.conName}, resetting agenda.`);
          localStorage.agendaEventCodes = [];
          localStorage.agendaConName = context.state.conName;
        }
        // Reload the user's name for email generation if it was set
        if (localStorage.userName) {
          context.commit('setUserName', localStorage.userName);
        }
        // Reload the saved agenda if it exists.
        // TODO: use a library to do this automatically.
        if (localStorage.agendaEventCodes) {
          JSON.parse(localStorage.agendaEventCodes)
            .forEach((c: string) => context.commit('addEventToAgenda', { code: c }));
        }

        context.commit('finishLoading');
        return stitchPromise;
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
        log.error(`Tried to dereference non-existent event ${dereferencedCode}.`);
        return;
      }
      event.description += ` ${dereferencedEvent.description}`;
    }
  });
  return Object.assign({}, ...(Object.values(scheduleObject).map((e: any) => ({ [e.code]: new Event(e) }))));
}

export default store;
