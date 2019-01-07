<template>
  <v-app id="inspire">
    <v-toolbar color="indigo" dark fixed app>
      <v-toolbar-title>Dextools</v-toolbar-title>
    </v-toolbar>
    <v-content>
      <v-container fluid grid-list-md>
        <v-layout row>
          <v-flex md8>
            <EventsList :schedule="schedule" />
          </v-flex>
          <v-flex md4>
          </v-flex>
        </v-layout>
      </v-container>
    </v-content>
    <v-footer color="indigo" app>
      <span class="white--text">&copy; 2019</span>
    </v-footer>
  </v-app>
</template>

<script lang="ts">
import HelloWorld from './components/HelloWorld.vue';
import EventsList from './components/EventsList.vue';
import schedule from './schedule';
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component({
  components: {
    HelloWorld,
    EventsList,
  },
})
export default class App extends Vue {
  @Prop() private source!: string;

  get schedule(){
    return schedule;
  }
};
</script>

