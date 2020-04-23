import Vue from 'vue';
import Vuex, { StoreOptions } from 'vuex';
import { RootState } from './types';
import actions from './actions';
import mutations from './mutations';
import Agenda from '@/models/agenda';
import Event from '@/models/event';
import axios from 'axios';
import log from 'loglevel';
import { StitchAppClient } from 'mongodb-stitch-browser-sdk';

Vue.use(Vuex);

// const store = new Vuex.Store({
const store: StoreOptions<RootState> = {
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
    feedbackUrl: '',
    notice: '',
  },
  mutations,
  actions,
  strict: process.env.NODE_ENV !== 'production',
};

export default new Vuex.Store<RootState>(store);
