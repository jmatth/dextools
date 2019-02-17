<template>
  <v-app id="inspire">
    <v-toolbar color="indigo" dark fixed app>
      <v-toolbar-title>Dextools</v-toolbar-title>
    </v-toolbar>
    <v-content>
      <v-container fluid grid-list-md>
        <v-layout row>
          <v-flex
            :md8="display.mode === 'split'"
            :md12="display.mode === 'full'"
          >
            <EventsList :schedule="schedule" :eventSchedule="scheduleList" />
          </v-flex>
          <v-flex
            v-if="display.mode === 'split'"
            md4
          >
            <ScheduleCalendar :schedule="schedule" :scheduleEvents="scheduleList" :display='display' />
          </v-flex>
        </v-layout>
        <v-layout
          v-if="display.mode === 'full'"
          row
        >
          <v-flex md12>
            <ScheduleCalendar :schedule="schedule" :scheduleEvents="scheduleList" :display='display' />
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
import ScheduleCalendar from './components/ScheduleCalendar.vue';
import Event from './models/event';
import Schedule from './models/schedule';
import scheduleJson from './schedule.json';
import { Component, Prop, Vue } from 'vue-property-decorator';

const schedule: { [index: string]: Event } = {};
scheduleJson.forEach((e: any) => {
  schedule[e.code] = new Event(e);
});

@Component({
  components: {
    EventsList,
    ScheduleView,
    ScheduleCalendar,
  },
})
export default class App extends Vue {
  get scheduleList() {
    return Object.values(schedule);
  }

  public data() {
    return {
      schedule: new Schedule(),
      display: {
        mode: 'split',
        toggle() {
          this.mode = this.mode === 'split' ? 'full' : 'split';
        },
      },
    };
  }
}
</script>
