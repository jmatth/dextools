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
        F003: createEvent({
          code: 'F003',
          start_time: '2020-11-03 09:00',
          end_time: '2020-11-03 10:00',
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

    it('computes the start and end dates correctly', () => {
      // @ts-ignore
      expect(wrapper.vm.startCal).to.equal('2020-11-01');
      // @ts-ignore
      expect(wrapper.vm.endCal).to.equal('2020-11-03');
    });

    it('defaults to day view', () => {
      expect(wrapper.findAll('.v-calendar .v-calendar-daily__day').length).to.equal(1);
    });

    it('has the "Day" button selected', () => {
      expect(wrapper.findAll('.v-btn-toggle .v-btn').at(0).classes('v-btn--active')).to.be.true;
      expect(wrapper.findAll('.v-btn-toggle .v-btn').at(1).classes('v-btn--active')).to.be.false;
    });

    it('starts on the first day', () => {
      expect(wrapper.get('.v-calendar-daily_head-weekday').text()).to.equal('Sun 1');
    });

    it('shows the day switcher buttons', () => {
      const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
      expect(buttons.length).to.equal(2);
    });

    it('disables the back day switcher button', () => {
      const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
      const backButton = buttons.at(0);
      expect(backButton.attributes('disabled')).to.equal('disabled');
    });

    it('does not disable the forward day switcher button', () => {
      const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
      const forwardButton = buttons.at(1);
      expect(forwardButton.attributes('disabled')).to.not.exist;
    });

    describe('step forward one day', () => {
      beforeEach(() => {
        const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
        const forwardButton = buttons.at(1);
        forwardButton.trigger('click');
        return wrapper.vm.$nextTick();
      });

      it('advances to the next day', () => {
        expect(wrapper.get('.v-calendar-daily_head-weekday').text()).to.equal('Mon 2');
      });

      it('does not disable either day switcher button', () => {
        const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
        const backButton = buttons.at(1);
        const forwardButton = buttons.at(1);
        expect(backButton.attributes('disabled')).to.not.exist;
        expect(forwardButton.attributes('disabled')).to.not.exist;
      });

      describe('step to final day', () => {
        beforeEach(() => {
          const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
          const forwardButton = buttons.at(1);
          forwardButton.trigger('click');
          return wrapper.vm.$nextTick();
        });

        it('advances to the next day', () => {
          expect(wrapper.get('.v-calendar-daily_head-weekday').text()).to.equal('Tue 3');
        });

        it('does not disable the back day switcher button', () => {
          const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
          const backButton = buttons.at(0);
          expect(backButton.attributes('disabled')).to.not.exist;
        });

        it('disables the forward day switcher button', () => {
          const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
          const forwardButton = buttons.at(1);
          expect(forwardButton.attributes('disabled')).to.equal('disabled');
        });

        describe('step back one day', () => {
          beforeEach(() => {
            const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
            const backButton = buttons.at(0);
            backButton.trigger('click');
            return wrapper.vm.$nextTick();
          });

          it('moves to the previous day', () => {
            expect(wrapper.get('.v-calendar-daily_head-weekday').text()).to.equal('Mon 2');
          });

          it('does not disable either day switcher button', () => {
            const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
            const backButton = buttons.at(1);
            const forwardButton = buttons.at(1);
            expect(backButton.attributes('disabled')).to.not.exist;
            expect(forwardButton.attributes('disabled')).to.not.exist;
          });
        });
      });
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
      expect(wrapper.findAll('.v-calendar .v-calendar-daily__day').length).to.be.equal(3);
    });

    it('has the "Week" button selected', () => {
      expect(wrapper.findAll('.v-btn-toggle .v-btn').at(0).classes('v-btn--active')).to.be.false;
      expect(wrapper.findAll('.v-btn-toggle .v-btn').at(1).classes('v-btn--active')).to.be.true;
    });

    it('does not show the day switcher buttons', () => {
      const buttons = wrapper.findAll('.v-toolbar__content > .v-btn');
      expect(buttons.length).to.equal(0);
    });
  });
});
