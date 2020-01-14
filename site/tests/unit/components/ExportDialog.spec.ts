import { expect } from 'chai';
import { spy, SinonSpy } from 'sinon';
import { mount, createLocalVue, Wrapper } from '@vue/test-utils';
import Vuex from 'vuex';
import Vuetify from 'vuetify';
import VueClipboard from 'vue-clipboard2';

import Agenda from '@/models/agenda';
import ExportDialog from '@/components/ExportDialog.vue';

const localVue = createLocalVue();
localVue.use(Vuex);
localVue.use(VueClipboard);

describe('components/ExportDialog.vue', () => {
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

    it('renders the export button', () => {
      const button = wrapper.find('button.v-btn');
      expect(button.isVisible()).to.be.true;
      expect(button.text()).to.equal('Export');
    });

    describe('open dialog', () => {
      beforeEach(() => {
        wrapper.find('button.v-btn').trigger('click');
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

      it('has the expected action buttons', () => {
        const buttons = wrapper.findAll('div.v-card__actions button.v-btn');
        expect(buttons.length).to.equal(4);
        expect(buttons.wrappers[0].text()).to.equal('Close');
        expect(buttons.wrappers[1].text()).to.equal('ICS');
        expect(buttons.wrappers[2].text()).to.equal('Email');
        expect(buttons.wrappers[3].text()).to.equal('Copy');
      });

      describe('the ICS export button is pushed', () => {
        let exportIcs: SinonSpy;
        beforeEach(() => {
          exportIcs = spy();
          store.state.agenda.exportIcs = exportIcs;
          const icsButton = wrapper.findAll('div.v-card__actions button.v-btn').wrappers[1];
          icsButton.trigger('click');
        });

        it('calls the exportIcs method on the agenda', () => {
          expect(exportIcs.called).to.be.true;
        });
      });
    });
  });
});
