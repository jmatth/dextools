<template>
  <v-dialog
    scrollable
    max-width="700px"
    :value="!!event"
    @input="e => !e && emitClose()">
    <v-card v-if="!!event">
      <v-card-title>
        {{ event.code }}: {{ event.title }}
      </v-card-title>
      <v-divider/>
      <v-card-text>
        <v-container fluid>
          <v-row no-gutters>
            <v-col cols="12" sm="9">
              <template v-if="event.system">
                {{ event.system }}
              </template>
              <template v-if="event.authors">
                by {{ event.authors }};
              </template>
              <template v-if="event.presenters">
                presented by {{ event.presenters }}
              </template>
            </v-col>
            <v-col cols="12" sm="3">
              <span class="float-sm-right">
                {{ weekday }}, {{ from }} - {{ to }}
              </span>
            </v-col>
          </v-row no-gutters>
          <v-row class="">
            <v-col class="pb-0">
              <p align="justify" class="mb-0">
                {{ event.description }}
              </p>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
      <v-divider/>
      <v-card-actions>
        <v-btn @click="emitClose">
          Close
        </v-btn>
        <v-spacer/>
        <v-btn
          min-width="80px"
          :color="toggleBtnColor"
          @click="toggleEvent"
        >
          {{ toggleBtnText }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Agenda from '../models/agenda';
import Event from '../models/event';
import moment, { Moment } from 'moment';

@Component
export default class EventInfoDialog extends Vue {
  @Prop() private event!: Event;

  get weekday(): string {
    return this.event.startTime.format('dddd');
  }

  get from(): string {
    return this.formatTime(this.event.startTime);
  }

  get to(): string {
    return this.formatTime(this.event.endTime);
  }

  get toggleBtnText(): string {
    return this.$store.state.agenda.contains(this.event.code)
      ? 'Remove'
      : 'Add';
  }

  get toggleBtnColor(): string {
    return this.$store.state.agenda.contains(this.event.code)
      ? 'error'
      : 'success';
  }

  private formatTime(m: Moment): string {
    return m.minute() === 0 ? m.format('hA') : m.format('h:mmA');
  }

  private toggleEvent(): void {
    if (this.$store.state.agenda.contains(this.event.code)) {
      this.$store.state.agenda.removeEvent(this.event.code);
    } else {
      this.$store.state.agenda.addEvent(this.event);
    }
    this.emitClose();
  }

  private emitClose(): void {
    this.$emit('close');
  }
}
</script>
