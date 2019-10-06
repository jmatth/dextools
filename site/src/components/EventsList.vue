<template>
  <v-card :style="{ height: height + 'px' }">
    <v-toolbar dark>
      <v-text-field
        label="Filter"
        single-line
        single
        dark
        clearable
        @input="updateSearch"
      />

      <v-spacer/>

      <v-select
        v-model.lazy="categories"
        :items="availableCodes"
        label="Categories"
        multiple
        chips
        solo
        dense
        clearable
        flat
        background-color="rgba(0,0,0,0)"
        :style="{ maxWidth: (Math.max(47 * availableCodes.length, 130)) + 'px' }"
      />


      <v-dialog v-model="showAdvancedFilter">
        <v-btn
          fab
          depressed
          flat
          slot="activator"
        >
          <v-icon>remove_red_eye</v-icon>
        </v-btn>
        <v-card>
          <v-card-title class="headline grey darken-4 grey--text">
            Advanced Search
          </v-card-title>
          <v-card-text>
            <v-container grid-list-md>
              <v-layout wrap>
                <v-flex sm12 md6>
                  <v-select
                    v-model.lazy="days"
                    :items="availableDays"
                    label="Days"
                    multiple
                    chips
                    solo
                    dense
                    clearable
                  />
                </v-flex>
                <v-flex sm6 md3>
                  <v-checkbox label="Hide filled" v-model="hideFilled"/>
                </v-flex>
                <v-flex sm6 md3>
                  <v-checkbox label="Hide conflicting" v-model="hideConflicting"/>
                </v-flex>
                <v-flex sm12>
                  <v-menu
                    ref="startTimeMenu"
                    v-model="filterStartTimeMenu"
                    :close-on-content-click="false"
                    :nudge-right="40"
                    :return-value.sync="filterStartTime"
                    lazy
                    transition="scale-transition"
                    offset-y
                    full-width
                    max-width="290px"
                    min-width="290px"
                    >
                    <template v-slot:activator="{ on }">
                      <v-text-field
                        v-model="filterStartTime"
                        label="Start at"
                        readonly
                        clearable
                        v-on="on"
                      />
                    </template>
                    <v-time-picker
                      v-if="filterStartTimeMenu"
                      v-model="filterStartTime"
                      full-width
                      format="24hr"
                      :allowed-minutes="timePickerStep"
                      @click:hour="$refs.startTimeMenu.save($event + ':00')"
                    />
                  </v-menu>
                </v-flex>
                <v-flex sm12 md6>
                  <v-select
                    v-model.lazy="testTypes"
                    :items="availableTestTypes"
                    label="Test Types"
                    multiple
                    chips
                    solo
                    dense
                    clearable
                  />
                </v-flex>
                <v-flex sm6 md3>
                  <v-select
                    v-model.lazy="hiTestFilter"
                    :items="hiTestFilterOptions"
                    label="HI-Tests"
                    solo
                    dense
                    clearable
                  />
                </v-flex>
              </v-layout>
            </v-container>
          </v-card-text>
          <v-divider/>
          <v-spacer/>
          <v-btn @click="showAdvancedFilter = false" color="primary">
            Apply
          </v-btn>
          <v-btn @click="clearAdvancedFilter">
            clear
          </v-btn>
        </v-card>
      </v-dialog>
    </v-toolbar>

    <DynamicScroller
      class="scroller"
      :items="filteredItems"
      :min-item-size="78"
      key-field="code"
      :style="{ height: dynamicScrollerHeight }"
    >
      <template v-slot="{ item, index, active }">
        <DynamicScrollerItem
          :item="item"
          :active="active"
          :data-index="index"
          :size-dependencies="[ expandedCode ]"
          :class="{ 'event-item-row': true, 'event-item-row-expanded': isExpanded(item) }"
        >
        <v-layout
          row
          wrap
          class="event-item-row-header"
          @click="toggleExpanded(item)"
        >
          <v-flex xs11>
            <v-layout row wrap>
              <v-flex xs12 sm5>
                <!-- <v&#45;avatar color="red" size="20" class="mr&#45;1"> -->
                <!--   <span class="white&#45;&#45;text">{{ item.code[0] }}</span> -->
                <!-- </v&#45;avatar> -->
                <span>{{ item.code }}: {{ item.title }}</span>
              </v-flex>
              <v-flex xs12 sm5>
                {{ item.system }}
              </v-flex>
              <v-flex xs12 sm1>
                <v-icon
                  v-if="item.testType === 'FOCUS GROUP'"
                  class="ml-1"
                  size="20"
                >
                  group
                </v-icon>
                <span
                  v-else
                  class="ml-1"
                >
                  {{ testTypeText(item.testType) }}
                </span>
              </v-flex>
              <v-flex xs12 sm5>
                {{ item.startTime.format('ddd, HH:mm') }} - {{ item.endTime.format('HH:mm') }}
              </v-flex>
              <v-flex xs12 sm5>
                {{ item.presenters }}
              </v-flex>
              <v-flex xs12 sm1>
                <v-tooltip
                  bottom
                  v-if="item.filled"
                >
                  <template v-slot:activator="{ on }">
                    <v-icon
                      class="ml-1"
                      size="20"
                      v-on="on"
                    >
                      lock
                    </v-icon>
                  </template>
                  <span>This event has been filled, you may sign up as an alternate at the convention.</span>
                </v-tooltip>
                <v-tooltip
                  bottom
                  v-if="item.hiTest"
                >
                  <template v-slot:activator="{ on }">
                    <v-icon
                      class="ml-1"
                      size="20"
                      v-on="on"
                    >
                      error_outline
                    </v-icon>
                  </template>
                  <span>This is a HI-TEST session.</span>
                </v-tooltip>
              </v-flex>
            </v-layout>
          </v-flex>
          <v-flex xs1>
            <v-layout row wrap>
              <v-flex xs12 order-xs2 sm6 order-sm1>
                <v-btn
                  flat
                  icon
                  depressed
                >
                  <v-icon size="20">{{ isExpanded(item) ? 'keyboard_arrow_up' : 'keyboard_arrow_down' }}</v-icon>
                </v-btn>
              </v-flex>
              <v-flex xs12 order-xs1 sm6 order-sm2>
                <v-btn
                  flat
                  icon
                  @click.stop="$store.commit('addEventToAgenda', item.code)"
                >
                  <v-icon size="20">add</v-icon>
                </v-btn>
              </v-flex>
            </v-layout>
          </v-flex>
        </v-layout>
        <div v-show="expandedCode === item.code" class="event-item-row-body">
          {{ item.description }}
        </div>
        </DynamicScrollerItem>
      </template>
    </DynamicScroller>
  </v-card>
</template>

<script lang="ts">
import Agenda from '../models/agenda';
import Event from '../models/event';
import { Moment } from 'moment';
import { Component, Prop, Vue } from 'vue-property-decorator';
import { debounce } from 'lodash';

@Component
export default class EventsList extends Vue {
  @Prop() private height!: number;

  public categories: string[] = [];
  public days: string[] = [];
  public testTypes: string[] = [];
  public filter: string = '';
  public items?: Event[] = undefined;
  public hideFilled: boolean = false;
  public hideConflicting: boolean = false;
  public hiTestFilter: boolean | null = null;
  public hiTestFilterOptions: any = [
    { text: 'Hide HI-Tests', value: false },
    { text: 'Only HI-Tests', value: true },
  ];
  public filterStartTime?: any = null;
  public filterStartTimeMenu: boolean = false;
  public showAdvancedFilter: boolean = false;
  public expandedCode: string = '';
  public dynamicScrollerHeight: string = '300px';

  // public expansionPanel: any = {
  //   panelClick(): any {
  //     console.log('panel clicked');
  //     console.log(arguments);
  //   },
  // };

  constructor() {
    super();
    this.updateSearch = debounce(this.updateSearch, 500);
  }

  get scheduleEvents(): Event[] {
    return Object.values(this.$store.state.schedule);
  }

  public updateSearch(input: string|null): void {
    this.filter = input === null ? '' : input.trim();
  }

  public created(): void {
    this.items = this.scheduleEvents.map((e: Event) => {
      return Object.assign({}, e);
    });
  }

  public toggleExpanded(event: Event): void {
    this.expandedCode = this.expandedCode === event.code ? '' : event.code;
  }

  public isExpanded(event: Event): boolean {
    return this.expandedCode === event.code;
  }

  get availableCodes() {
    return this.scheduleEvents.reduce((acc: string[], e: Event) => {
      const code = e.code[0];
      if (!acc.includes(code)) {
        acc.push(code);
      }
      return acc;
    }, []);
  }

  get availableDays() {
    const days = this.scheduleEvents.reduce((acc: any, event: Event) => {
      const day = event.startTime.clone();
      if (!acc.includes(day.format('dd'))) {
        acc.push(day);
      }
      return acc;
    }, []);
    days.sort((d1: Moment, d2: Moment) => d1.isBefore(d2) ? -1 : 1);
    return days.map((d: Moment) => d.format('dd'));
  }

  get availableTestTypes() {
    const typesSet = this.scheduleEvents.reduce((acc: any, event: Event) => {
      if (event.testType) {
        acc.add(event.testType);
      }
      return acc;
    }, new Set());
    const typesArr: string[] = Array.from(typesSet.values());
    typesArr.sort();
    return ['<None>'].concat(typesArr);
  }

  public timePickerStep(minutes: number): boolean {
    return minutes % 30 === 0;
  }

  public clearAdvancedFilter(): void {
    this.filterStartTime = null;
    this.days = [];
    this.hideFilled = false;
    this.hideConflicting = false;
    this.showAdvancedFilter = false;
  }

  get filteredItems(): Event[] {
    return this.items ? this.items.filter((e: Event) => this.shouldShow(e)) : [];
  }

  public mounted() {
    this.updateDynamicScrollerHeight();
  }

  public beforeUpdate() {
    this.updateDynamicScrollerHeight();
  }

  private updateDynamicScrollerHeight() {
    // @ts-ignore
    const toolbarHeight = this.$el.querySelector('nav.v-toolbar').offsetHeight;
    this.dynamicScrollerHeight = (this.height - toolbarHeight) + 'px';
  }

  public shouldShow(event: Event): boolean {
    return (
      (!this.hideFilled || !event.filled) &&
      (!this.hideConflicting || !this.$store.state.agenda.events.some((se: Event) => se.conflicts(event))) &&
      (this.hiTestFilter === null || this.hiTestFilter === undefined || this.hiTestFilter === event.hiTest) &&
      (this.categories.length < 1 || this.categories.includes(event.code[0])) &&
      (this.days.length < 1 || this.days.includes(event.startTime.format('dd'))) &&
      (this.filterStartTime ? event.startTime.format('H:mm') === this.filterStartTime : true) &&
      (this.testTypes.length < 1 || this.testTypes.map((t) => t === '<None>' ? '' : t).includes(event.testType)) &&
      (!this.filter || [
          'code',
          'title',
          'system',
          'description',
          'presenters',
          'authors',
        ].some((field) => (event[field] as string).toLowerCase().includes(this.filter.toLowerCase())))
    );
  }

  public testTypeText(type?: string): string {
    switch (type) {
      case 'ALPHA TEST': {
        return 'α';
      }
      case 'BETA TEST': {
        return 'β';
      }
      case '': {
        return '';
      }
      case undefined: {
        return '';
      }
    }
    console.error(`Found unknown test type ${type}`);
    return '';
  }
}
</script>

<style scoped lang="scss">
.event-item-row {
  border-top: 1px solid rgba(0,0,0,0.12);

  &-expanded {
    margin-bottom: 10px;
  }

  &-header {
    padding: 12px 24px 12px 24px;
    cursor: pointer;
  }

  &-body {
    padding: 0px 24px 12px 24px;
  }
}
</style>
