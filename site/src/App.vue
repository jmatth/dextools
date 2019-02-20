<template>
  <v-app id="inspire">
    <v-toolbar color="indigo" dark fixed app>
      <v-toolbar-title>Dextools</v-toolbar-title>
      <v-spacer/>
      <v-dialog
        v-model="about"
        width="500"
      >
        <v-btn
          fab
          depressed
          flat
          slot="activator"
        >
          <v-icon>
            help_outline
          </v-icon>
        </v-btn>
        <v-card>
          <v-card-title
            class="headline grey lighten-2"
            primary-title
          >
            About
          </v-card-title>
          <v-card-text>
            <p>
              This is a site to help with building your schedule for the excellent conventions run by Double Exposure Inc.
              It uses data scraped from the Double Exposure website so may be innacurrate or out of date.
            </p>
            <p>
              This site WILL NOT register you for any events; it will only help you figure out your schedule and export it
              to an ics file to import into a calendar program like Google Calendar. You will still need to register for
              events via the usual method detailed on the Double Exposure website.
              If you find a bug or have a feature suggestion, please submit it
              <a target="_blank" href="https://github.com/jmatth/dextools">on the github</a>.
            </p>
            <p>
              This site is not associated with or endorsed by Double Exposure Inc.
            </p>
          </v-card-text>
          <v-divider/>
          <v-spacer/>
          <v-btn
            @click="about = false"
          >
            Close
          </v-btn>
        </v-card>
      </v-dialog>
    </v-toolbar>
    <v-content>
      <v-container fluid grid-list-md>
        <v-layout row>
          <v-flex
            :md8="display.mode === 'split'"
            :md12="display.mode === 'full'"
          >
            <EventsList :schedule="schedule" :eventSchedule="scheduleList" :height="eventListHeight"/>
          </v-flex>
          <v-flex
            v-if="display.mode === 'split'"
            md4
          >
            <ScheduleCalendar :schedule="schedule" :scheduleEvents="scheduleList" :display='display' :height="calendarHeight" />
          </v-flex>
        </v-layout>
        <v-layout
          v-if="display.mode === 'full'"
          row
        >
          <v-flex md12>
            <ScheduleCalendar :schedule="schedule" :scheduleEvents="scheduleList" :display='display' :height="calendarHeight"/>
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
    ScheduleCalendar,
  },
})
export default class App extends Vue {
  public schedule = new Schedule();
  public about = false;
  public display = {
        mode: 'split',
        getHeight() {
          return this.mode === 'split' ? 700 : 350;
        },
        toggle() {
          this.mode = this.mode === 'split' ? 'full' : 'split';
        },
      };

  public mounted(): void {
    if (localStorage.scheduledEventCodes) {
      try {
        JSON.parse(localStorage.scheduledEventCodes).forEach((c: string) => this.schedule.addEvent(schedule[c]));
      } catch {}
    }
  }

  get scheduleList() {
    return Object.values(schedule);
  }

  get calendarHeight() {
    return (this.display.mode === 'split' ? 700 : 550) + 'px';
  }

  get eventListHeight() {
    return (this.display.mode === 'split' ? 700 : 400) + 'px';
  }
}
</script>
