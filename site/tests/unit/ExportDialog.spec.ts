import { expect } from 'chai';
import { mount, createLocalVue, Wrapper } from '@vue/test-utils';
import Vue from 'vue';
import Vuex from 'vuex';
import Vuetify from 'vuetify';

import Agenda from '@/models/agenda';
import ExportDialog from '@/components/ExportDialog.vue';

Vue.use(Vuetify);

const localVue = createLocalVue();
localVue.use(Vuex);
// localVue.use(Vuetify);

declare const global: any;
global.requestAnimationFrame = (cb: any) => cb();

describe('ExportDialog.vue', () => {
  const actions: any = null;
  let state: any;
  let store: any;
  let vuetify: any;

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
    });
  });
});
