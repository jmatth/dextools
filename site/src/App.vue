<template>
  <v-app id="inspire">
    <v-dialog :value="scheduleLoading" width="130" hide-overlay persistent>
      <v-card color="primary" dark>
        <v-card-text>
          Loading...
          <v-progress-circular
            indeterminate
            color="white"
            class="mb-0"
          />
        </v-card-text>
      </v-card>
    </v-dialog>
    <v-toolbar
      color="indigo"
      dark
      fixed
      app
      :dense="onMobile"
    >
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
        <v-layout
          v-if="Object.keys($store.state.schedule).length > 0"
          row
          wrap
        >
          <v-flex
            :md12="display.mode === 'full'"
            :md8="display.mode === 'split'"
          >
            <EventsList
              :height="eventListHeight"
            />
          </v-flex>
          <v-flex
            :md12="display.mode === 'full'"
            :md4="display.mode === 'split'"
          >
            <AgendaCalendar
              :display='display'
              :height="calendarHeight"
            />
          </v-flex>
        </v-layout>
      </v-container>
    </v-content>
  </v-app>
</template>

<script lang="ts">
import EventsList from './components/EventsList.vue';
import AgendaCalendar from './components/AgendaCalendar.vue';
import Event from './models/event';
import Agenda from './models/agenda';
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component({
  components: {
    EventsList,
    AgendaCalendar,
  },
})
export default class App extends Vue {
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

  get onMobile(): boolean {
    // @ts-ignore
    return this.$vuetify.breakpoint.smAndDown;
  }

  public mounted(): void {
    if (localStorage.agendaEventCodes) {
      try {
        JSON.parse(localStorage.agendaEventCodes).forEach((c: string) =>
          this.$store.commit('addEventToAgenda', c));
      } catch (err) {
        console.log(`Failed to load agenda from localStorage: ${err}`);
      }
    }
    if (this.onMobile) {
      this.display.mode = 'full';
    }
  }

  get scheduleLoading() {
    return Object.keys(this.$store.state.schedule).length < 1;
  }

  get calendarHeight() {
    return (this.display.mode === 'split' ? 700 : 550) + 'px';
  }

  get eventListHeight() {
    return (this.display.mode === 'split' ? 764 : 400) + 'px';
  }
}
</script>
