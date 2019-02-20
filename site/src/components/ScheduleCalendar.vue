<template>
  <v-layout>
    <v-flex>
      <v-toolbar
        class="blue-grey white--text"
      >
        <v-btn
          v-if="calType === 'day'"
          fab
          small
          depressed
          :disabled="prevDisabled"
          left
          color="primary"
          @click="prevDay()"
        >
          <v-icon dark>
            keyboard_arrow_left
          </v-icon>
        </v-btn>
        <v-btn
          v-if="calType === 'day'"
          fab
          small
          depressed
          :disabled="nextDisabled"
          left
          color="primary"
          @click="nextDay()"
        >
          <v-icon dark>
            keyboard_arrow_right
          </v-icon>
        </v-btn>
        <v-btn
          fab
          small
          depressed
          left
          color="primary"
          @click="toggleDisplay()"
        >
          <v-icon dark>
            {{ calType === 'day' ? 'fullscreen' : 'fullscreen_exit' }}
          </v-icon>
        </v-btn>
        <v-spacer/>
        <v-btn
          dark
          depressed
          small
          @click="schedule.exportIcs()"
        >
          Export
        </v-btn>
      </v-toolbar>
      <v-card :height="calType == 'day' ? height : 'auto'">
        <!-- now is normally calculated by itself, but to keep the calendar in this date range to view events -->
        <v-calendar
          ref="calendar"
          v-model="currDate"
          color="primary"
          :interval-height="intervalHeight"
          :type="calType"
          :start='startCal'
          :end='endCal'
        >
          <template
            slot="dayBody"
            slot-scope="{ date, timeToY, minutesToPixels }"
          >
            <template v-for="(startWithEvents, startIndex) in eventsByStartTime">
              <template v-if="startDateMatchesDate(startWithEvents[0], date)">
                <template v-for="(event, index) in startWithEvents[1]">
                  <div
                    :key="event.code"
                    :style="{
                      top: timeToY(event.startTime.format('HH:MM')) + 'px',
                      height: minutesToPixels(getEventMinutes(event)) + 'px',
                      marginLeft: overlappingEventsCount(event) * 5 + 100/startWithEvents[1].length * index + '%'}"
                    class="my-event with-time"
                    @click="schedule.removeEvent(event.code)"
                    >
                      {{ `${event.code}: ${event.title}` }}
                      <v-icon
                        v-if="event.filled"
                        small
                        dark
                      >lock</v-icon>
                  </div>
                </template>
              </template>
            </template>
          </template>
        </v-calendar>
      </v-card>
    </v-flex>
  </v-layout>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Schedule from '../models/schedule';
import Event from '../models/event';
import moment from 'moment';

@Component
export default class ScheduleCalendar extends Vue {
  @Prop() private schedule!: Schedule;
  @Prop() private scheduleEvents!: Event[];
  @Prop() private display!: any;
  @Prop() private height!: string;

  public categories: string[] = ['L', 'R'];
  public filter: string = '';
  public currDate = '';

  public created() {
    this.currDate = this.startCal;
  }

  public mounted() {
    // @ts-ignore
    this.$refs.calendar.scrollToTime('09:00');
  };

  public startDateMatchesDate(startDate: any, date: any): boolean {
    const dateMoment = moment(date);
    const startDateMoment = moment(startDate);
    const match = startDateMoment.isSame(dateMoment, 'day');
    return match;
  }

  public eventMatchesDate(event: Event, date: any): boolean {
    return event.startTime.isSame(date, 'day');
  }

  public getEventMinutes(event: Event): number {
    const eventMins = moment.duration(event.endTime.diff(event.startTime)).asMinutes();
    return eventMins;
  }

  public overlappingEventsCount(event: Event): number {
    const oldOverlappingCount = this.schedule.events.reduce((acc: number, e: Event) => {
      if (e.code === event.code) {
        return acc;
      }
      if (e.endTime.isAfter(event.startTime) && e.startTime.isBefore(event.startTime)) {
        return acc + 1;
      }
      return acc;
    }, 0);
    return oldOverlappingCount;
  }

  get intervalHeight() {
    return this.display.mode === 'split' ? 26 : 20;
  }

  get calType(): string {
    return this.display.mode === 'split' ? 'day' : 'custom-daily';
  }

  get startCal(): string {
    return this.scheduleEvents[0].startTime.format("YYYY-MM-DD");
  }

  get endCal(): string {
    return this.scheduleEvents[this.scheduleEvents.length - 1].endTime.format("YYYY-MM-DD");
  }

  get nextDisabled() {
    return this.currDate === this.endCal;
  }

  get prevDisabled() {
    return this.currDate === this.startCal;
  }

  public nextDay() {
    if (this.nextDisabled) {
      return;
    }
    // @ts-ignore
    this.$refs.calendar.next();
  }

  public prevDay() {
    if (this.prevDisabled) {
      return;
    }
    // @ts-ignore
    this.$refs.calendar.prev();
  }

  public toggleDisplay() {
    this.display.toggle();
  }

  get eventsByStartTime() {
    const ebt = new Map();
    this.schedule.events.forEach(e => {
      const startStr = e.startTime.format();
      if (!ebt.has(startStr)) {
        ebt.set(startStr, []);
      }
      ebt.get(startStr).push(e);
    })
    return Array.from(ebt);
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
<style lang="scss">
.my-event {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  border-radius: 2px;
  background-color: #1867c0;
  color: #ffffff;
  border: 1px solid #1867c0;
  font-size: 12px;
  padding: 3px;
  cursor: pointer;
  margin-bottom: 1px;
  left: 4px;
  margin-right: 8px;
  position: relative;

  &.with-time {
    position: absolute;
    right: 4px;
    margin-right: 0px;
    borderwidth: 1px;
    border-color: white;
  }
}
</style>
