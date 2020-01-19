<template>
  <v-card height="100%">
    <v-toolbar flat>
      <v-btn
        v-if="calType === 'day'"
        small
        depressed
        :disabled="prevDisabled"
        left
        color="primary"
        class="mr-1"
        @click="prevDay()"
      >
        <v-icon>
          keyboard_arrow_left
        </v-icon>
      </v-btn>
      <v-btn
        v-if="calType === 'day'"
        small
        depressed
        :disabled="nextDisabled"
        left
        color="primary"
        class="mr-2"
        @click="nextDay()"
      >
        <v-icon>
          keyboard_arrow_right
        </v-icon>
      </v-btn>
      <v-spacer/>
      <ExportDialog/>
    </v-toolbar>

    <v-divider/>

    <v-calendar
      class="my-calendar"
      :style="{ height: calendarHeight }"
      ref="calendar"
      v-model="currDate"
      color="primary"
      :interval-height="intervalHeight"
      :type="calType"
      :start='startCal'
      :end='endCal'
      :weekday-format="formatDayHeader"
      :events="calEvents"
      @click:event="eventClicked"
    >
    </v-calendar>
    </div>
    <EventInfoDialog v-model="focusedEvent"/>
  </v-card>
</template>

<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import ExportDialog from './ExportDialog.vue';
import EventInfoDialog from './EventInfoDialog.vue';
import Agenda from '../models/agenda';
import Event from '../models/event';
import moment, { Moment } from 'moment';
import { debounce } from 'lodash';

@Component({
  components: {
    ExportDialog,
    EventInfoDialog,
  },
})
export default class AgendaCalendar extends Vue {
  @Prop() private display!: any;
  @Prop() private calType!: string;

  public currDate = '';
  public intervalHeight = 10;
  public focusedEvent: Event | null = null;

  private weekdayStrMap: string[] = [
    'Sun',
    'Mon',
    'Tue',
    'Wed',
    'Thu',
    'Fri',
    'Sat',
  ];

  public created() {
    this.currDate = this.startCal;
  }

  public mounted() {
    this.updateComputedHeights();
    window.addEventListener('resize', debounce(this.updateComputedHeights, 250));
  }

  private updateComputedHeights() {
    const calendarEl = (this.$refs.calendar as Vue).$el;
    const calendarHeight = calendarEl.getBoundingClientRect().height;
    const headerHeight = calendarEl.querySelector('div.v-calendar-daily__head')!.getBoundingClientRect().height;
    const computedIntervalHeight = ((calendarHeight - headerHeight) - 1) / 24;
    this.intervalHeight = Math.max(computedIntervalHeight, 10);
  }

  get scheduleEvents(): Event[] {
    return Object.values(this.$store.state.schedule);
  }

  get lastEventAdded() {
    return this.$store.state.agenda.lastAdded;
  }

  @Watch('lastEventAdded')
  private onEventAdded(): void {
    this.currDate = this.lastEventAdded
      ? (this.lastEventAdded.startTime as Moment).format('YYYY-MM-DD')
      : this.currDate;
  }

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
    const oldOverlappingCount = this.$store.state.agenda.events.reduce((acc: number, e: Event) => {
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

  get startCal(): string {
    return this.scheduleEvents
      .reduce((acc: Moment, e: Event) =>
        acc && e.startTime.isAfter(acc) ? acc : e.startTime, this.scheduleEvents[0].startTime)
      .format('YYYY-MM-DD');
  }

  get endCal(): string {
    return this.scheduleEvents[this.scheduleEvents.length - 1].endTime.format('YYYY-MM-DD');
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

  get calEvents() {
    const calEventFormat = 'YYYY-M-D H:m';
    return this.$store.state.agenda.events.map((e: Event) => {
      return {
        code: e.code,
        name: `${e.code}: ${e.title}`,
        start: e.startTime.format(calEventFormat),
        end: e.endTime.format(calEventFormat),
      };
    });
  }

  get eventsByStartTime() {
    const ebt = new Map();
    this.$store.state.agenda.events.forEach((e: Event) => {
      const startStr = e.startTime.format();
      if (!ebt.has(startStr)) {
        ebt.set(startStr, []);
      }
      ebt.get(startStr).push(e);
    });
    return Array.from(ebt);
  }

  get allowResizing() {
    // @ts-ignore
    return !this.$vuetify.breakpoint.smAndDown;
  }

  get calendarHeight(): string {
    return `calc(100% - ${this.$vuetify.application.top}px)`;
  }

  public formatDayHeader(dayObj: any): string {
    const weekdayStr = this.weekdayStrMap[dayObj.weekday];
    const day = dayObj.day;
    return `${weekdayStr} ${day}`;
  }

  public eventClicked(vueEvent: any): void {
    this.focusedEvent = this.$store.state.schedule[vueEvent.event.code];
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

.theme--light.v-calendar-daily {
  border-left: none;
}

div.v-calendar.v-calendar-daily.my-calendar {
  border-top: none;

  .v-calendar-daily__scroll-area {
    overflow-y: auto;
  }
}

div.v-calendar-daily__day-container > div.v-calendar-daily__day:last-child,
div.v-calendar-daily__head > div.v-calendar-daily_head-day:last-child {
  border-right: none;
}

.v-calendar-daily_head-day-label {
  display: none;
}
</style>
