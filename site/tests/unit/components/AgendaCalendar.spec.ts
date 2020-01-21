import { expect } from 'chai';
import { spy, SinonSpy } from 'sinon';
import { mount, createLocalVue, Wrapper } from '@vue/test-utils';
import Vuex from 'vuex';
import Vuetify from 'vuetify';

import Agenda from '@/models/agenda';
import AgendaCalendar from '@/components/AgendaCalendar.vue';
import { createEvent } from '../models/common';

const localVue = createLocalVue();
localVue.use(Vuex);

describe('components/AgendaCalendar.vue', () => {
  let vuetify: any;
  let store: any;
  let state: any;
  const actions: any = null;

  before(() => {
    const app = document.createElement('div');
    app.setAttribute('data-app', 'true');
    document.body.append(app);
  });

  beforeEach(() => {
    vuetify = new Vuetify();
    state = {
      agenda: new Agenda(),
      schedule: {
        F001: createEvent({
          code: 'F001',
          start_time: '2020-11-01 09:00',
          end_time: '2020-11-01 10:00',
        }),
        F002: createEvent({
          code: 'F002',
          start_time: '2020-11-02 09:00',
          end_time: '2020-11-02 10:00',
        }),
      },
    };
    store = new Vuex.Store({
      actions,
      state,
    });
  });

  describe('rendered', () => {
    let wrapper: Wrapper<AgendaCalendar>;

    beforeEach(() => {
      wrapper = mount(AgendaCalendar, {
        store,
        localVue,
        vuetify,
      });
    });

    it('defaults to day view', () => {
      expect(wrapper.vm.calType).to.equal('day');
      expect(wrapper.findAll('.v-calendar .v-calendar-daily__day').length).to.equal(1);
      expect(wrapper.findAll('.v-btn-toggle .v-btn').at(0).classes('v-btn--active')).to.be.true;
      expect(wrapper.findAll('.v-btn-toggle .v-btn').at(1).classes('v-btn--active')).to.be.false;
    });

    it('shows the day switcher buttons', () => {
      const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
      expect(buttons.length).to.equal(2);
    });
  });

  describe('rendered with initialView="custom-daily"', () => {
    let wrapper: Wrapper<AgendaCalendar>;

    beforeEach(() => {
      wrapper = mount(AgendaCalendar, {
        store,
        localVue,
        vuetify,
        propsData: {
          initialView: 'custom-daily',
        },
      });
    });

    it('renders the multi-day view', () => {
      expect(wrapper.vm.calType).to.equal('custom-daily');
      expect(wrapper.findAll('.v-calendar .v-calendar-daily__day').length).to.be.equal(2);
      expect(wrapper.findAll('.v-btn-toggle .v-btn').at(0).classes('v-btn--active')).to.be.false;
      expect(wrapper.findAll('.v-btn-toggle .v-btn').at(1).classes('v-btn--active')).to.be.true;
    });
  });
});
