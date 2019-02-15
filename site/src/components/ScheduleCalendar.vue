<template>
  <v-card>
    <v-calendar
      ref="calendar"
      type="custom-daily"
      :start="startCal"
      :end="endCal"
    >
    </v-calendar>
  </v-card>
</template>
<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Schedule from '../models/schedule';
import Event from '../models/event';

@Component
export default class EventsList extends Vue {
  @Prop() private schedule!: Schedule;
  @Prop() private scheduleEvents!: Event[];

  public categories: string[] = ['L', 'R'];
  public filter: string = '';

  public created(): void {
    this.scheduleEvents.sort((e1: Event, e2: Event) =>
      e1.startTime.isBefore(e2.startTime) ? -1 : 1);
  }

  get startCal(): string {
    return this.scheduleEvents[0].startTime.format("YYYY-MM-DD");
  }

  get endCal(): string {
    return this.scheduleEvents[this.scheduleEvents.length - 1].endTime.format("YYYY-MM-DD");
  }

  get items() {
    return this.schedule.events
      .filter((e) => this.categories.includes(e.code[0]))
      .filter((e) => {
        if (!this.filter) {
          return true;
        }
        return [
          'title',
          'system',
          'description',
          'presenters',
        ].some((field) => (<string>e[field]).toLowerCase().includes(this.filter.toLowerCase()));
      });
  }
}
</script>

