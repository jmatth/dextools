<template>
  <v-dialog
    v-model="exportDialogue"
    width="500"
  >
    <template v-slot:activator="{ on }">
      <v-btn
        dark
        depressed
        small
        color="primary"
        v-on="on"
      >
        Export
      </v-btn>
    </template>
    <v-card>
      <v-card-title
        class="headline grey lighten-2"
        primary-title
      >
        Export
      </v-card-title>
      <v-card-text>
        <v-text-field
            label="Enter your name"
            single-line
            :value="userName"
            @input="updateUserName"
        />
        <p>
          Use the buttons below to export to ICS (for importing into calendar applications)
          or email (to register for events). For email, you can also copy the text below.
        </p>
        <span v-if="!disableEmail">To: <a :href="'mailto:' + conEmail">{{ conEmail }}</a></span>
        <v-divider style="margin-bottom:5px; margin-top: 2px;"/>
        <pre>{{ emailText }}</pre>
      </v-card-text>
      <v-card-actions>
        <v-btn
          @click="$store.state.agenda.exportIcs($store.state.conName)"
        >
          ICS
        </v-btn>
        <v-tooltip top :disabled="!disableEmail">
          <template v-slot:activator="{ on }">
            <a :href="mailtoLink" v-on="on">
              <v-btn
                @click="exportDialogue = false"
                :disabled="disableEmail"
              >
                Email
              </v-btn>
            </a>
          </template>
          <span>{{ this.$store.state.conName }} is not accepting event registrations at this time.</span>
        </v-tooltip>
        <v-tooltip top v-model="shouldShowCopyMessage">
          <template v-slot:activator="{ on }">
            <v-btn
              v-on="{ on }"
              v-clipboard:copy="emailText"
              v-clipboard:success="onCopy"
              v-clipboard:error="onFailedCopy"
            >
              Copy
            </v-btn>
          </template>
          <span>{{ this.copyMessage }}</span>
        </v-tooltip>
        <v-spacer/>
        <v-btn
          @click="exportDialogue = false"
        >
          Close
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import { Component, Vue, Watch } from 'vue-property-decorator';
import Agenda from '../models/agenda';
import Event from '../models/event';
import moment, { Moment } from 'moment';
import { debounce } from 'lodash';

@Component
export default class ExportDialogue extends Vue {
  public exportDialogue: boolean = false;
  public shouldShowCopyMessage: boolean = false;
  public copyMessage: string = '';

  constructor() {
    super();
    this.updateUserName = debounce(this.updateUserName, 500);
  }

  get userName(): string {
    return this.$store.state.userName;
  }

  public updateUserName(name: string): void {
    this.$store.commit('setUserName', name);
  }

  get emailText(): string {
    const userName: string = this.$store.state.userName || '<YOUR NAME>';
    return this.$store.state.agenda.events.reduce(
      (text: string, event: Event) => text += `\n${event.code}`, `Name: ${userName}\n\nEvents:`);
  }

  get conEmail(): string {
    return this.$store.state.conEmail;
  }

  get mailtoLink(): string {
    if (this.disableEmail) {
      return '#';
    }
    const subject = encodeURI(`${this.$store.state.conName} Registration`);
    const body = encodeURI(this.emailText);
    return `mailto:${this.conEmail}?subject=${subject}&body=${body}`;
  }

  get disableEmail(): boolean {
    return !this.$store.state.conEmail;
  }

  public showCopyTooltip(message: string): void {
    this.copyMessage = message;
    this.shouldShowCopyMessage = true;
    setTimeout(function() {
      // @ts-ignore
      this.shouldShowCopyMessage = false;
    }.bind(this), 5000);
  }

  public onCopy(): void {
    this.showCopyTooltip('Email text copied to clipboard.');
  }

  public onFailedCopy(): void {
    this.showCopyTooltip('Error, please copy the email text manually.');
  }
}
</script>
<style lang="scss">
</style>
