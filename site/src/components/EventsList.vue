<template>
  <v-card height="100%">
    <v-toolbar flat>
      <v-container fluid class="toolbar-container">
        <v-row dense align="center" justify="space-between">
          <v-col cols="5">
            <v-text-field
              label="Filter"
              outlined
              dense
              clearable
              hide-details
              single-line
              @input="updateSearch"
            />
          </v-col>
          <v-col cols="6">
            <v-select
              v-model.lazy="categories"
              :items="availableCodes"
              label="Categories"
              multiple
              chips
              small-chips
              outlined
              dense
              clearable
              hide-details
              single-line
              menu-props="below, offsetY"
            />
          </v-col>
          <v-col cols="1">
            <AdvancedFilterDialog
              @apply="applyAdvancedSearch"
              :availableDays="availableDays"
              :availableTestTypes="availableTestTypes"
              :includeMetatopiaFilters="includeMetatopiaFilters"
              :parentFilter="advancedFilter"
            />
          </v-col>
        </v-row>
      </v-container>
    </v-toolbar>

    <v-divider/>

    <v-expansion-panels accordion :style="{ height: dynamicScrollerHeight, overflowY: 'auto' }">
      <v-expansion-panel v-for="item in items" :key="item.code">
        <v-expansion-panel-header>
          <v-container fluid class="event-item-row-header">
            <v-row no-gutters>
              <v-col cols="11">
                <v-row no-gutters>
                  <v-col cols="12" sm="5">
                    <span>{{ item.code }}: {{ item.title }}</span>
                  </v-col>
                  <v-col cols="12" sm="5">
                    {{ item.system }}
                  </v-col>
                  <v-col cols="12" sm="1">
                    <v-icon
                      v-if="item.testType === 'FOCUS GROUP'"
                      size="20"
                    >
                      group
                    </v-icon>
                    <span
                      v-else
                    >
                      {{ testTypeText(item.testType) }}
                    </span>
                  </v-col>
                  <v-col cols="12" sm="5">
                    {{ item.startTime.format('ddd, h:mmA') }} - {{ item.endTime.format('h:mmA') }}
                  </v-col>
                  <v-col cols="12" sm="5">
                    {{ item.presenters }}
                  </v-col>
                  <v-col cols="12" sm="1">
                    <v-tooltip
                      bottom
                      v-if="item.filled"
                    >
                      <template v-slot:activator="{ on }">
                        <v-icon
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
                          size="20"
                          v-on="on"
                        >
                          error_outline
                        </v-icon>
                      </template>
                      <span>This is a HI-TEST session.</span>
                    </v-tooltip>
                  </v-col>
                </v-row>
              </v-col>
              <v-col cols="1">
                <v-btn
                  text
                  icon
                  @click.stop="toggleEvent(item)"
                >
                  <v-icon
                    size="20"
                    :color="success"
                  >
                    add_circle
                  </v-icon>
                </v-btn>
              </v-col>
            </v-row>
          </v-container>
        </v-expansion-panel-header>
        <v-expansion-panel-content>
          {{ item.description }}
        </v-expansion-panel-content>
      </v-expansion-panel>
    </v-expansion-panels>
  </v-card>
</template>

<script lang="ts">
import Agenda from '../models/agenda';
import Event from '../models/event';
import EventsListItem from './EventsListItem.vue';
import AdvancedFilterDialog from './AdvancedFilterDialog.vue';
import { Moment } from 'moment';
import { Component, Prop, Vue } from 'vue-property-decorator';
import { debounce } from 'lodash';
import log from 'loglevel';

export interface AdvancedFilter {
  days: string[];
  filterStartTime?: any;
  hideFilled: boolean;
  hideConflicting: boolean;
  hiTestFilter: boolean | null;
  testTypes: string[];
}

export const emptyAdvancedFilter: AdvancedFilter = {
  days: [],
    filterStartTime: undefined,
    hideFilled: false,
    hideConflicting: false,
    hiTestFilter: null,
    testTypes: [],
};

@Component({
  components: {
    AdvancedFilterDialog,
    EventsListItem,
  },
})
export default class EventsList extends Vue {
  public categories: string[] = undefined;
  public filter: string = '';
  public items?: any[] = undefined;
  public advancedFilter: AdvancedFilter = Object.assign({}, emptyAdvancedFilter);
  public hiTestFilterOptions: any = [
    { text: 'Hide HI-Tests', value: false },
    { text: 'Only HI-Tests', value: true },
  ];
  public expandedCode: string = '';
  public resizeTicker: boolean = false;

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
    this.categories = [];
  }

  public toggleExpanded(code: string): void {
    this.expandedCode = this.expandedCode === code ? '' : code;
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

  get availableDays(): string[] {
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

  get includeMetatopiaFilters(): boolean {
    return !!this.$store.state.isMetatopia;
  }

  get dynamicScrollerHeight(): string {
    return `calc(100% - ${this.$vuetify.application.top + 1}px)`;
  }

  public applyAdvancedSearch(updated: AdvancedFilter): void {
    this.advancedFilter = Object.assign({}, this.advancedFilter, updated);
  }

  public shouldShow(event: Event): boolean {
    return (
      (!this.advancedFilter.hideFilled || !event.filled) &&
      (!this.advancedFilter.hideConflicting
        || !this.$store.state.agenda.events.some((se: Event) => se.conflicts(event))) &&
      (this.advancedFilter.hiTestFilter === null
        || this.advancedFilter.hiTestFilter === undefined
        || this.advancedFilter.hiTestFilter === event.hiTest) &&
      (this.categories.length < 1 || this.categories.includes(event.code[0])) &&
      (this.advancedFilter.days.length < 1 || this.advancedFilter.days.includes(event.startTime.format('dd'))) &&
      (this.advancedFilter.filterStartTime
        ? event.startTime.format('H:mm') === this.advancedFilter.filterStartTime
        : true) &&
      (this.advancedFilter.testTypes.length < 1
        || this.advancedFilter.testTypes.map((t) => t === '<None>' ? '' : t).includes(event.testType)) &&
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
}
</script>

<style scoped lang="scss">
.container.toolbar-container {
  padding: 0px;
}
</style>
