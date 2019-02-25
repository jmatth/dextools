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
        <div slot="header">
          <v-avatar color="red" size="20" class="mr-1">
            <span class="white--text">{{ item.code[0] }}</span>
          </v-avatar>
          <span>{{ item.code }}: {{ item.title }}</span>
          <v-chip class="caption text-truncate" v-if="item.system" small>{{ item.system }}</v-chip>
          <v-icon class="ml-1" size="20" v-if="item.filled">lock</v-icon>
        </div>

        <v-card>
          <v-card-text>
            <p v-if="item.system" class="caption">{{ item.system }}</p>
            <p>{{ item.description }}</p>
          </v-card-text>
          <v-card-actions>
            <v-list-tile class="grow">
              <v-list-tile-content>
                <v-list-tile-title>{{ item.presenters || item.authors }}</v-list-tile-title>
              </v-list-tile-content>

              <v-layout
                align-center
                justify-end
                >
                <span class="mr-2">{{ item.startTime.format('ddd, HH:mm') }} - {{ item.endTime.format('ddd, HH:mm') }}</span>
                <v-btn flat v-on:click="schedule.addEvent(item)">Add</v-btn>
              </v-layout>
            </v-list-tile>
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
