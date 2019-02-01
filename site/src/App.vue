<template>
  <v-app id="inspire">
    <v-toolbar color="indigo" dark fixed app>
      <v-toolbar-title>Dextools</v-toolbar-title>
    </v-toolbar>
    <v-content>
      <v-container fluid grid-list-md>
        <v-layout row>
          <v-flex md8>
            <EventsList :schedule="schedule" :eventSchedule="scheduleList" />
          </v-flex>
          <v-flex md4>
            <ScheduleView :schedule="schedule" />
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
import EventsList from './components/EventsList.vue';
import ScheduleView from './components/ScheduleView.vue';
import Event from './models/event';
import Schedule from './models/schedule';
import scheduleJson from './schedule';
import { Component, Prop, Vue } from 'vue-property-decorator';

const schedule = {};
scheduleJson.forEach((e: any) => {
  schedule[e.code] = new Event(e);
});

@Component({
  components: {
    EventsList,
    ScheduleView,
  },
})
export default class App extends Vue {
  get scheduleList(){
    return Object.values(schedule);
  }

  data() {
    return {
      schedule: new Schedule(),
    };
  }
};
</script>

