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
import { debounce } from 'lodash';
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component({
  components: {
    EventsList,
    AgendaCalendar,
  },
})
export default class App extends Vue {
  public about = false;
  private workspaceHeight = 700;
  public display = {
        mode: 'split',
        getHeight() {
          return this.mode === 'split' ? 700 : 350;
        },
        toggle() {
          this.mode = this.mode === 'split' ? 'full' : 'split';
        },
      };

  constructor() {
    super();
    this.handleResize = debounce(this.handleResize, 250);
  }

  get onMobile(): boolean {
    // @ts-ignore
    return this.$vuetify.breakpoint.smAndDown;
  }

  public handleResize() {
    const windowHeight = window.innerHeight;
    const workspacePaddingStr = window
      .getComputedStyle(document.querySelector('div.container')!, null)
      .getPropertyValue('padding-bottom');
    const workspacePadding = workspacePaddingStr.endsWith('px')
      ? parseInt(workspacePaddingStr.substring(0, workspacePaddingStr.length - 2), 10)
      : 16;
    // @ts-ignore
    const headerHeight = document.querySelector('nav.v-toolbar')!.offsetHeight;
    this.workspaceHeight = windowHeight - (workspacePadding * 2 + headerHeight);
  }

  public mounted(): void {
    // Start the request to load the schedule json file as soon as possible.
    const settingsLoaded = this.$store.dispatch('loadSettings');
    if (this.onMobile) {
      this.display.mode = 'full';
    }
    window.addEventListener('resize', this.handleResize);
    this.handleResize();
    settingsLoaded.then(() => {
      // If the site has been updated for a new con, blow out the agenda cache.
      if (localStorage.agendaConName !== this.$store.state.conName) {
        // tslint:disable-next-line
        console.log(`Detected convention change from ${localStorage.agendaConName} to ${this.$store.state.conName}, resetting agenda.`);
        localStorage.agendaEventCodes = [];
        localStorage.agendaConName = this.$store.state.conName;
      }
      // Reload the user's name for email generation if it was set
      if (localStorage.userName) {
        this.$store.commit('setUserName', localStorage.userName);
      }
      // Reload the saved agenda if it exists.
      // TODO: use a library to do this automatically.
      if (localStorage.agendaEventCodes) {
        JSON.parse(localStorage.agendaEventCodes).forEach((c: string) => this.$store.commit('addEventToAgenda', c));
      }
    });
  }

  get scheduleLoading() {
    return Object.keys(this.$store.state.schedule).length < 1;
  }

  get calendarHeight(): number {
    if (this.onMobile) {
      return this.workspaceHeight;
    }
    return this.display.mode === 'split' ? this.workspaceHeight : (this.workspaceHeight * 0.6) - 5;
  }

  get eventListHeight(): number {
    if (this.onMobile) {
      // TODO: Calculate this offset rather than hardcode it
      return this.workspaceHeight - 56;
    }
    return this.display.mode === 'split' ? this.workspaceHeight : (this.workspaceHeight * 0.4) - 5;
  }
}
</script>
