<template>
  <v-app>
    <!-- <v&#45;dialog :value="scheduleLoading" width="130" hide&#45;overlay persistent> -->
    <!--   <v&#45;card color="primary" dark> -->
    <!--     <v&#45;card&#45;text> -->
    <!--       Loading... -->
    <!--       <v&#45;progress&#45;circular -->
    <!--         indeterminate -->
    <!--         color="white" -->
    <!--         class="mb&#45;0" -->
    <!--       /> -->
    <!--     </v&#45;card&#45;text> -->
    <!--   </v&#45;card> -->
    <!-- </v&#45;dialog> -->
    <v-app-bar
      color="indigo"
      dark
      fixed
      app
      :dense="onMobile"
    >
      <v-toolbar-title>Dextools</v-toolbar-title>
      <v-spacer/>
      <v-dialog
        v-model="feedback"
        width="500"
      >
        <template v-slot:activator="{ on }">
          <v-btn
            fab
            depressed
            text
            v-on="on"
          >
            <v-icon>
              feedback
            </v-icon>
          </v-btn>
        </template>
        <v-card>
          <v-card-title
            class="headline grey lighten-2"
            primary-title
          >
            Feedback
          </v-card-title>
          <v-card-text>
            <p>
            Have feedback about the site? I would love it if you could take a minute (literally a minute, I promise) to fill out <a :href="this.$store.state.feedbackUrl" target="_blank">this anonymous form</a>.
            </p>
          </v-card-text>
          <v-divider/>
          <v-spacer/>
          <v-btn
            @click="feedback = false"
          >
            Close
          </v-btn>
        </v-card>
      </v-dialog>
      <v-dialog v-model="about" width="500">
        <template v-slot:activator="{ on }">
          <v-btn
            fab
            depressed
            text
            v-on="on"
          >
            <v-icon>
              help_outline
            </v-icon>
          </v-btn>
        </template>
        <v-card>
          <v-card-title
            class="headline grey lighten-2"
            primary-title
          >
            About
          </v-card-title>
          <v-card-text>
            <h2>What's this?</h2>
            <p>
              This is a site to help with building your schedule for the excellent conventions run by Double Exposure Inc.
              It uses data scraped from the Double Exposure website so may be innacurrate or out of date.
            </p>
            <h2>How can I build my convention schedule?</h2>
            <p>
              Use the <b>+</b> buttons in the event list to add an event to your schedule. You can see your current schedule in the calendar view.
              To remove an event from your schedule, just click its entry in the calendar. When you are satisfied with your schedule, you can click
              the <b>Export</b> button at the top of the calendar to export your schedule to an email to sign up for events, or to a format for importing
              into calendar applications like Google Calendar.
              <b>
                Simply adding events to your schedule will not register you for those events, you must still send email to the
                appropriate email address for the convention. This site will just help you produce the text for that email.
              </b>
            </p>
            <h2>How do I use the search features?</h2>
            <p>
            It is possible to filter the events list by several criteria:
            <ul>
              <li>
                <b>Categories</b> can be used to filter by the first character in the event code.
                This usually corresponds to the event type in some way, such as R for RPGs, L for LARPs, B for board games, etc.
              </li>
              <li>
                The <b>Filter</b> box can be used to type an arbitrary query.
                Events that contains that string in any of their fields will be included in the results.
              </li>
            </ul>
            By clicking the eye icon, you can view advanced search options:
            <ul>
              <li>
                <b>Days</b> can be used to restrict your search to events that take place on a particular day.
              </li>
              <li>
                <b>Start at</b> limits your search to events that start at <em>exactly</em> the specified time.
              </li>
              <li>
                <b>Hide filled</b> will hide events that have already been marked as filled.
                There can be significant lag between an event filling up and that information being reflected on the Double Exposure
                website, so information on which events are filled will often be out of date.
              </li>
              <li>
                <b>Hide Conflicting</b> will not show any events that overlap with events already added to your schedule.
                Useful for when you have your schedule mostly figured out and are just looking for something to fill some time.
              </li>
            </ul>
            </p>
            <h2>Questions/bug reports/feedback?</h2>
            <p>
            Any feedback via <a :href="this.$store.state.feedbackUrl" target="_blank">this anonymous form</a> is greatly appreciated.
            If you are so inclined, you can also open issues directly <a href="https://github.com/jmatth/dextools/issues/new" target="_blank">on the github repo</a>.
            </p>
            <h2>I should probably add...</h2>
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
    </v-app-bar>
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
  public feedback = false;
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
    const headerHeight = document.querySelector('div#app div header.v-sheet')!.offsetHeight;
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
