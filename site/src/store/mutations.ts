import { MutationTree } from 'vuex';
import { RootState, Schedule } from './types';
import { StitchAppClient } from 'mongodb-stitch-browser-sdk';
import log from 'loglevel';

const mutations: MutationTree<RootState> = {
  finishLoading(state: RootState) {
    state.loading = false;
  },
  addEventToAgenda(
    state: RootState,
    payload: { code: string, skipLastAdded: boolean },
  ): void {
    const event = state.schedule[payload.code];
    if (!event) {
      log.error(`Trying to add event ${payload.code} which does not exist in schedule.`);
      return;
    }
    state.agenda.addEvent(event, payload.skipLastAdded);
  },
  removeEventFromAgenda(state: RootState, code: string) {
    state.agenda.removeEvent(code);
  },
  toggleEvent(state: RootState, code: string) {
    log.debug(`Toggling event ${code}`);
    const event = state.schedule[code];
    if (!event) {
      log.error(`Trying to toggle event ${code} which does not exist in schedule.`);
      return;
    }
    state.agenda.toggleEvent(state.schedule[code]);
  },
  setSchedule(state: RootState, schedule: Schedule) {
    state.schedule = schedule;
  },
  setConName(state: RootState, conName: string) {
    state.conName = conName;
  },
  setConEmail(state: RootState, conEmail: string) {
    state.conEmail = conEmail;
  },
  setUserName(state: RootState, userName: string) {
    state.userName = userName;
    localStorage.userName = userName;
  },
  setClient(state: RootState, client: StitchAppClient) {
    state.stitchClient = client;
  },
  setVisitId(state: RootState, newId: string) {
    state.visitId = newId;
  },
  setFeedbackUrl(state: RootState, newUrl: string) {
    state.feedbackUrl = newUrl;
  },
  setIsMetatopia(state: RootState, isMetatopia: boolean) {
    state.isMetatopia = isMetatopia;
  },
};

export default mutations;
