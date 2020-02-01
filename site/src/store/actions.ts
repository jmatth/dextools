import { ActionTree } from 'vuex';
import { RootState } from './types';
import Event from '@/models/event';
import {
  Stitch,
  AnonymousCredential,
} from 'mongodb-stitch-browser-sdk';
import axios from 'axios';
import log from 'loglevel';

const actions: ActionTree<RootState, RootState> = {
  loadSettings(context) {
    return axios.get('/settings.json').then((response: any) => {
      const settings = response.data;
      context.commit('setConName', settings.conName);
      context.commit('setConEmail', settings.conEmail);
      context.commit('setFeedbackUrl', settings.feedbackUrl);
      context.commit('setIsMetatopia', settings.isMetatopia);
      const schedule = settings.schedule.reduce((acc: any, event: any) => {
        acc[event.code] = new Event(event);
        return acc;
      }, {});
      context.commit('setSchedule', schedule);

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
      return settings;
    }).then((settings: any) => {
      const stitchAppId = settings.stitchApp;
      if (!stitchAppId) {
        log.warn('No stitch app configured, not sending analytics.');
        return Promise.resolve();
      }
      const client = Stitch.initializeDefaultAppClient(stitchAppId);
      return client.auth.loginWithCredential(new AnonymousCredential())
        .then((user: any) => {
          context.commit('setClient', client);
          return client.callFunction('startVisit', [{ referrer: document.referrer }]);
        })
        .then((result: any) => context.commit('setVisitId', result));
    });
  },
};

export default actions;
