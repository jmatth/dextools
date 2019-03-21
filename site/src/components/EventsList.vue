<template>
  <v-card>
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
        :style="{ maxWidth: (47 * availableCodes.length) + 'px' }"
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
          <v-card-title
            class="headline grey lighten-2"
            primary-title
          >
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
                    background-color="rgba(0,0,0,0)"
                    :style="{ maxWidth: (32 * availableCodes.length) + 'px' }"
                  />
                </v-flex>
                <v-flex sm12 md6>
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
                      ></v-time-picker>
                  </v-menu>
                </v-flex>
                <v-flex sm12 md2>
                  <v-checkbox label="Hide filled" v-model="hideFilled"/>
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

    <v-expansion-panel :style="{ height: height, overflowY: 'scroll' }">
      <v-expansion-panel-content
        v-for="(item, index) in items"
        v-show="shouldShow(item)"
        :key="item.code + '-epc'"
      >
        <v-layout row wrap slot="header">
          <v-flex xs12 sm5>
            <!-- <v&#45;avatar color="red" size="20" class="mr&#45;1"> -->
            <!--   <span class="white&#45;&#45;text">{{ item.code[0] }}</span> -->
            <!-- </v&#45;avatar> -->
            <span>{{ item.code }}: {{ item.title }}</span>
          </v-flex>
          <v-flex xs12 sm7>
            {{ item.system }}
          </v-flex>
          <v-flex xs12 sm5>
            {{ item.startTime.format('ddd, HH:mm') }} - {{ item.endTime.format('HH:mm') }}
          </v-flex>
          <v-flex xs12 sm5>
            {{ item.presenters }}
          </v-flex>
          <v-flex xs12 sm2>
            <v-icon class="ml-1" size="20" v-if="item.filled">lock</v-icon>
          </v-flex>
        </v-layout>

        <v-card>
          <v-card-text>
            {{ item.description }}
          </v-card-text>
          <v-card-actions>
            <v-spacer />
            <v-btn
              flat
              v-on:click="$store.commit('addEventToAgenda', item)">
                Add
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-expansion-panel-content>
    </v-expansion-panel>
  </v-card>
</template>

<script lang="ts">
import Agenda from '../models/agenda';
import Event from '../models/event';
import { Component, Prop, Vue } from 'vue-property-decorator';
import { debounce } from 'lodash';

@Component
export default class EventsList extends Vue {
  @Prop() private height!: string;

  public categories: string[] = [];
  public days: string[] = [];
  public filter: string = '';
  public items?: Event[] = undefined;
  public hideFilled: boolean = false;
  public filterStartTime?: any = null;
  public filterStartTimeMenu: boolean = false;
  public showAdvancedFilter: boolean = false;

  constructor() {
    super();
    this.updateSearch = debounce(this.updateSearch, 500);
  }

  get scheduleEvents(): Event[] {
    return Object.values(this.$store.state.schedule);
  }

  public updateSearch(input: any): void {
    this.filter = input;
  }

  public created(): void {
    this.items = this.scheduleEvents.map((e: Event) => {
      return Object.assign({}, e);
    });
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
    return this.scheduleEvents.reduce((acc: any, event: Event) => {
      const day = event.startTime.clone().format('dd');
      if (!acc.includes(day)) {
        acc.push(day);
      }
      return acc;
    }, []);
  }

  public timePickerStep(minutes: number): boolean {
    return minutes % 30 === 0;
  }

  public clearAdvancedFilter(): void {
    this.filterStartTime = null;
    this.days = [];
    this.showAdvancedFilter = false;
  }

  public shouldShow(event: Event): boolean {
    return (
      (!this.hideFilled || !event.filled) &&
      (this.categories.length < 1 || this.categories.includes(event.code[0])) &&
      (this.days.length < 1 || this.days.includes(event.startTime.format('dd'))) &&
      (this.filterStartTime ? event.startTime.format('H:mm') === this.filterStartTime : true) &&
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
