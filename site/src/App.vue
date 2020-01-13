<template>
  <v-app id="app">
    <v-navigation-drawer
      app
      bottom
      clipped
      v-model="showNav"
    >
      <v-list dense>
        <v-list-item link to="/">
          <v-list-item-action>
            <v-icon>list_alt</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>Schedule</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
        <v-list-item link to="/calendar">
          <v-list-item-action>
            <v-icon>calendar_today</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>Calendar</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>
    <v-app-bar
      color="indigo"
      app
      dark
      clipped-left
    >
      <v-app-bar-nav-icon @click.stop="showNav = !showNav" />
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
          <v-card-title primary-title>
            Feedback
          </v-card-title>
          <v-card-text>
            <p>
            Have feedback about the site? I would love it if you could take a minute (literally a minute, I promise) to fill out <a :href="this.$store.state.feedbackUrl" target="_blank">this anonymous form</a>.
            </p>
          </v-card-text>
          <v-divider/>
          <v-card-actions>
            <v-btn
              @click="feedback = false"
            >
              Close
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
      <v-dialog v-model="about" scrollable eager width="500">
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
          <v-card-title primary-title>
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
                Events that contain that string in any of their fields will be included in the results.
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
          <v-card-actions>
            <v-btn
              @click="about = false"
            >
              Close
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
    </v-app-bar>
    <v-content>
      <router-view/>
      <v-snackbar
        color="warning"
        bottom
        v-model="updateExists"
        :timeout="0"
      >
        A new version of this site is available.
        <v-btn dark @click="refreshApp()">Refresh</v-btn>
        <v-btn icon @click="updateExists = false"><v-icon>close</v-icon></v-btn>
      </v-snackbar>
    </v-content>
  </v-app>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component({})
export default class App extends Vue {
  public showNav: boolean = false;
  public about = false;
  public feedback = false;

  // These are specific to the SW update process.
  public updateExists = false;
  public refreshing = false;
  public registration: any = {};

  public created(): void {
    document.addEventListener(
      'swUpdated', this.showRefreshUI, { once: true },
    );
    navigator.serviceWorker.addEventListener(
      'controllerchange', () => {
        if (this.refreshing) {
          return;
        }
        console.log('Caught controllerchange, refreshing...');
        this.refreshing = true;
        window.location.reload();
      },
    );
  }

  private showRefreshUI(e: any) {
    console.log('Caught swUpdated, update available.');
    this.registration = e.detail;
    console.log(e.detail);
    this.updateExists = true;
  }

  public refreshApp(): void {
    console.log('Refreshing app to update.');
    this.updateExists = false;
    if (!this.registration || !this.registration.waiting) {
      console.log('Registration does not exist or is not waiting, bailing out of update.');
      return;
    }
    this.registration.waiting.postMessage('skipWaiting');
  }

  public mounted(): void {
    // Start the request to load the schedule json file as soon as possible.
    this.$store.dispatch('loadSettings').then(() => {
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
}
</script>
