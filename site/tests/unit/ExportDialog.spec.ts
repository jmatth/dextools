import { expect } from 'chai';
import { mount, createLocalVue, Wrapper } from '@vue/test-utils';
import Vue from 'vue';
import Vuex from 'vuex';
import Vuetify from 'vuetify';
import VueClipboard from 'vue-clipboard2';

import Agenda from '@/models/agenda';
import ExportDialog from '@/components/ExportDialog.vue';

import './setup';

const localVue = createLocalVue();
localVue.use(Vuex);

describe('ExportDialog.vue', () => {
  const actions: any = null;
  let state: any;
  let store: any;
  let vuetify: any;

  before(() => {
    const app = document.createElement('div');
    app.setAttribute('data-app', 'true');
    document.body.append(app);
  });

  beforeEach(() => {
    vuetify = new Vuetify();
    state = {
      agenda: new Agenda(),
    };
    store = new Vuex.Store({
      actions,
      state,
    });
  });

  describe('initial render', () => {
    let wrapper: Wrapper<ExportDialog>;

    beforeEach(() => {
      wrapper = mount(ExportDialog, {
        store,
        localVue,
        vuetify,
      });
    });

    it('has an empty userName', () => {
      expect(wrapper.vm.userName).to.equal('');
    });

    it('has the dialog closed', () => {
      expect(wrapper.find('.v-dialog').isVisible()).to.be.false;
    });

    describe('open dialog', () => {
      beforeEach(() => {
        wrapper.setData({ exportDialog: true });
        return wrapper.vm.$nextTick();
      });

      it('renders the dialog', () => {
        expect(wrapper.find('.v-dialog').isVisible()).to.be.true;
      });

      it('has the the expected placeholder text', () => {
        const label = wrapper.find('.v-input.v-text-field label.v-label');
        expect(label).to.exist;
        expect(label.text()).to.equal('Enter your name');
      });

      it('has the expected registration text', () => {
        const pre = wrapper.find('.v-card__text pre');
        expect(pre).to.exist;
        expect(pre.text()).to.equal('Name: <YOUR NAME>\n\nEvents:');
      });
    });
  });
});
