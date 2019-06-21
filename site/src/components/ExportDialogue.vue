<template>
  <v-dialog
    v-model="exportDialogue"
    width="500"
  >
    <template slot="activator">
      <slot>
        <v-btn
          dark
          depressed
          small
          color="primary"
        >
          Export
        </v-btn>
      </slot>
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
        <v-divider/>
        <pre>{{ emailText }}</pre>
      </v-card-text>
      <v-card-actions>
        <v-btn
          @click="$store.state.agenda.exportIcs()"
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
          <span>{{ this.$store.state.conName }} is not accepting event registrations yet.</span>
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
export default class AgendaCalendar extends Vue {
  public exportDialogue: boolean = false;

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
      (text: string, event: Event) => text += `\n${event.code}`, `${userName}\n`);
  }

  get mailtoLink(): string {
    if (this.disableEmail) {
      return '#';
    }
    const subject = encodeURI(`${this.$store.state.conName} Registration`);
    const body = encodeURI(this.emailText);
    return `mailto:${this.$store.state.conEmail}?subject=${subject}&body=${body}`;
  }

  get disableEmail(): boolean {
    return !this.$store.state.conEmail;
  }
}
</script>
<style lang="scss">
</style>
