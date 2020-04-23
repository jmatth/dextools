<template>
  <v-app id="app">
    <Dialog
      v-model="notice"
      title="Notice"
    >
      <p class="text-justify" v-html="$store.state.notice"/>
    </Dialog>
    <NavDrawer v-model="showNav"/>
    <v-app-bar
      color="indigo"
      app
      dark
      clipped-left
    >
      <v-app-bar-nav-icon @click.stop="showNav = !showNav" />
      <v-toolbar-title>Dextools</v-toolbar-title>
      <v-spacer/>
      <Dialog
        v-model="feedback"
        title="Feedback"
        max-width="500px"
      >
        <template v-slot:activator="{ on }">
          <v-btn
            fab
            depressed
            text
            aria-label="Feedback"
            v-on="on"
          >
            <v-icon>
              feedback
            </v-icon>
          </v-btn>
        </template>
        <p>
          Have feedback about the site? I would love it if you could take a minute (literally a minute, I promise) to fill out <a :href="this.$store.state.feedbackUrl" target="_blank">this anonymous form</a>.
        </p>
      </Dialog>
      <HelpDialog/>
    </v-app-bar>
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
    <v-content>
      <router-view/>
    </v-content>
  </v-app>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import NavDrawer from '@/components/NavDrawer.vue';
import Dialog from '@/components/Dialog.vue';
import HelpDialog from '@/components/HelpDialog.vue';
import log from 'loglevel';

@Component({
  components: {
    NavDrawer,
    Dialog,
    HelpDialog,
  },
})
export default class App extends Vue {
  public showNav: boolean | null = null;
  public about = false;
  public feedback = false;
  public notice = false;

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
        log.info('Caught controllerchange, refreshing...');
        this.refreshing = true;
        window.location.reload();
      },
    );
  }

  private showRefreshUI(e: any) {
    log.info('Caught swUpdated, update available.');
    this.registration = e.detail;
    this.updateExists = true;
  }

  public refreshApp(): void {
    log.info('Refreshing app to update.');
    this.updateExists = false;
    if (!this.registration || !this.registration.waiting) {
      log.warn('Registration does not exist or is not waiting, bailing out of update.');
      return;
    }
    this.registration.waiting.postMessage('skipWaiting');
  }

  public mounted(): void {
    this.$store.dispatch('loadSettings').then(() => {
      this.notice = !!this.$store.state.notice;
    });
  }
}
</script>
