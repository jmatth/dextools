import Vue from 'vue';
import Vuetify from 'vuetify/lib';

Vue.use(Vuetify);

const dark = localStorage.darkMode || false;

export default new Vuetify({
  iconfont: 'mdi',
  theme: {
    dark,
  },
});
