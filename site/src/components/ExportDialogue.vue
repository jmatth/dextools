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
        <p>
          Use the buttons below to export to ICS (for importing into calendar applications)
          or email (to register for events). For email, you can also copy the text below.
        </p>
      <v-divider/>
        <pre>{{ emailText }}</pre>
      </v-card-text>
      <v-card-actions>
        <v-btn
          @click="$store.state.schedule.exportIcs()"
        >
          ICS
        </v-btn>
        <a :href="mailtoLink">
          <v-btn
            @click="exportDialogue = false"
          >
            Email
          </v-btn>
        </a>
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
import Schedule from '../models/schedule';
import Event from '../models/event';
import moment, { Moment } from 'moment';

@Component
export default class ScheduleCalendar extends Vue {
  public exportDialogue: boolean = false;

  get emailText(): string {
    return this.$store.state.schedule.events.reduce(
      (text: string, event: Event) => text += `\n${event.code}`, '<YOUR NAME>\n');
  }

  get mailtoLink(): string {
    const subject = encodeURI(`${this.$store.state.conName} Registration`);
    const body = encodeURI(this.emailText);
    return `mailto:${this.$store.state.conEmail}?subject=${subject}&body=${body}`;
  }
}
</script>
<style lang="scss">
</style>
