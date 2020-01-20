import Vue from 'vue';
import Vuetify from 'vuetify/lib';

Vue.use(Vuetify);

const dark = localStorage.darkMode === 'true';

export default new Vuetify({
  iconfont: 'mdi',
  theme: {
    dark,
  },
});
