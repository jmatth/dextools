<template>
  <v-container fluid grid-list-md>
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
    <v-row
      v-if="Object.keys($store.state.schedule).length > 0"
      dense
    >
      <v-col cols="12" md="8">
        <EventsList :height="workspaceHeight"/>
      </v-col>
      <v-col
        v-if="!onMobile"
        cols="4"
        >
        <AgendaCalendar calType='day' :height="workspaceHeight"/>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import EventsList from '@/components/EventsList.vue';
import AgendaCalendar from '@/components/AgendaCalendar.vue';
import Event from '@/models/event';
import Agenda from '@/models/agenda';
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
  public workspaceHeight = 700;
  private refreshing = false;
  private registration: any = {};
  private updateExists = false;

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
    this.workspaceHeight = windowHeight - (workspacePadding * 3 + headerHeight);
  }

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
    const settingsLoaded = this.$store.dispatch('loadSettings');
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
}
</script>
