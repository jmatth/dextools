<template>
  <v-card :style="{ height }">
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
      <v-btn
        v-show="allowResizing"
        small
        depressed
        left
        color="primary"
        @click="toggleDisplay()"
      >
        <v-icon>
          {{ calType === 'day' ? 'horizontal_split' : 'vertical_split' }}
        </v-icon>
      </v-btn>
      <v-spacer/>
      <ExportDialogue/>
    </v-toolbar>
    <v-divider/>
    <v-calendar
      class="my-calendar"
      :height="calendarHeight + 'px'"
      ref="calendar"
      v-model="currDate"
      color="primary"
      :interval-height="intervalHeight"
      :type="calType"
      :start='startCal'
      :end='endCal'
      :weekday-format="formatDayHeader"
    >
      <template v-slot:day-body="{ date, timeToY, minutesToPixels }">
        <template v-for="(startWithEvents, startIndex) in eventsByStartTime">
          <template v-if="startDateMatchesDate(startWithEvents[0], date)">
            <template v-for="(event, index) in startWithEvents[1]">
              <div
                :key="event.code"
                :style="{
                  top: (timeToY(event.startTime.format('HH:MM')) - 2) + 'px',
                  height: minutesToPixels(getEventMinutes(event)) + 'px',
                  marginLeft: overlappingEventsCount(event) * 5 + 100/startWithEvents[1].length * index + '%'}"
                class="my-event with-time"
                @click="$store.commit('removeEventFromAgenda', event.code)"
                >
                  {{ `${event.code}: ${event.title}` }}
                  <v-icon
                    v-if="event.filled"
                    small
                  >lock</v-icon>
                  <v-icon
                    v-if="event.hiTest"
                    small
                  >error_outline</v-icon>
              </div>
            </template>
          </template>
        </template>
      </template>
    </v-calendar>
    </div>
  </v-card>
</template>

<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import ExportDialogue from './ExportDialogue.vue';
import Agenda from '../models/agenda';
import Event from '../models/event';
import moment, { Moment } from 'moment';

@Component({
  components: {
    ExportDialogue,
  },
})
export default class AgendaCalendar extends Vue {
  @Prop() private display!: any;
  @Prop() private height!: number;

  public categories: string[] = ['L', 'R'];
  public filter: string = '';
  public currDate = '';
  public calendarHeight = 300;
  public intervalHeight = 10;

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
  }

  public beforeUpdate() {
    this.updateComputedHeights();
  }

  private updateComputedHeights() {
    // @ts-ignore
    const toolbarHeight = this.$el.querySelector('div#app div header.v-sheet').offsetHeight;
    this.calendarHeight = this.height - toolbarHeight;
    // @ts-ignore
    const headerHeight = this.$el.querySelector('div.v-calendar-daily__head')!.offsetHeight;
    const computedIntervalHeight = ((this.calendarHeight - headerHeight) - 1) / 24;
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

  get calType(): string {
    return this.display.mode === 'split' ? 'day' : 'custom-daily';
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

  public toggleDisplay() {
    this.display.toggle();
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

  get items() {
    return this.$store.state.agenda.events
      .filter((e: Event) => this.categories.includes(e.code[0]))
      .filter((e: Event) => {
        if (!this.filter) {
          return true;
        }
        return [
          'title',
          'system',
          'description',
          'presenters',
        ].some((field) => (e[field] as string).toLowerCase().includes(this.filter.toLowerCase()));
      });
  }

  get allowResizing() {
    // @ts-ignore
    return !this.$vuetify.breakpoint.smAndDown;
  }

  public formatDayHeader(dayObj: any): string {
    const weekdayStr = this.weekdayStrMap[dayObj.weekday];
    const day = dayObj.day;
    return `${weekdayStr} ${day}`;
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
}

div.v-calendar-daily__day-container > div.v-calendar-daily__day:last-child,
div.v-calendar-daily__head > div.v-calendar-daily_head-day:last-child {
  border-right: none;
}

.v-calendar-daily_head-day-label {
  display: none;
}

/* .cal-hide-right-margin div.v-calendar-daily__day-container > div.v-calendar-daily__day, */
/* .cal-hide-right-margin div.v-calendar-daily__head > div.v-calendar-daily_head-day { */
/*   border-right: none; */
/* } */
</style>
