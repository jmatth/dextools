<template>
  <v-card>
    <v-toolbar
      dark
      :extended="breakToolbar"
    >
      <v-text-field
        label="Filter"
        single-line
        single
        dark
        @input="debounceSearch"
        ></v-text-field>

      <v-spacer></v-spacer>

    <v-checkbox :slot="breakToolbar ? 'extension' : undefined" v-for="code in availableCodes" v-model="categories" :label="code" :value="code"></v-checkbox>

    </v-toolbar>

    <v-expansion-panel :style="{ height: height, overflowY: 'scroll' }">
      <v-expansion-panel-content v-for="(item, index) in items" v-show="shouldShow(item)">
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
            <v-btn flat v-on:click="schedule.addEvent(item)">Add</v-btn>
          </v-card-actions>
        </v-card>
      </v-expansion-panel-content>
    </v-expansion-panel>
  </v-card>
</template>

<script lang="ts">
import Schedule from '../models/schedule';
import Event from '../models/event';
import { Component, Prop, Vue } from 'vue-property-decorator';
import { debounce } from 'lodash';

@Component
export default class EventsList extends Vue {
  @Prop() private eventSchedule!: any;
  @Prop() private schedule!: Schedule;
  @Prop() private height!: string;

  public categories: string[] = [];
  public filter: string = '';

  constructor() {
    super();
    this.debounceSearch = debounce(this.debounceSearch, 500);
  }

  public debounceSearch(input: any): void {
    this.filter = input;
  }

  public beforeMount() {
    this.categories = this.availableCodes.slice(0);
  }

  get availableCodes() {
    return this.eventSchedule.reduce((acc: string[], e: Event) => {
      const code = e.code[0];
      if (!acc.includes(code)) {
        acc.push(code);
      }
      return acc;
    }, []);
  }

  get breakToolbar() {
    // @ts-ignore
    return this.$vuetify.breakpoint.smAndDown;
  }

  public shouldShow(event: Event): boolean {
    return this.categories.includes(event.code[0])
      && (!this.filter || [
          'title',
          'system',
          'description',
          'presenters',
          'authors',
        ].some((field) => (event[field] as string).toLowerCase().includes(this.filter.toLowerCase())));
  }

  get items() {
    return this.eventSchedule;
    // return this.eventSchedule
    //   .filter((e: Event) => this.categories.includes(e.code[0]))
    //   .filter((e: Event) => {
    //     if (!this.filter) {
    //       return true;
    //     }
    //     return [
    //       'title',
    //       'system',
    //       'description',
    //       'presenters',
    //       'authors',
    //     ].some((field) => (e[field] as string).toLowerCase().includes(this.filter.toLowerCase()));
    //   });
  }
}
</script>
