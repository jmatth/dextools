<template>
  <Dialog
    v-model="showAdvancedFilter"
    @input="e => !e && dialogClosed()"
    title="Advanced Search"
    activator-icon="filter_list"
  >
    <template v-slot:activator="{ on }">
      <v-btn icon small v-on="on" aria-label="Advanced filters">
        <v-icon>filter_list</v-icon>
      </v-btn>
    </template>
    <v-container fluid class="pa-0">
      <v-row dense align="center">
        <v-col
          cols="12" order="1"
          sm="7" order-sm="1"
        >
          <v-select
            v-model.lazy="advancedFilterIndex.days"
            :items="availableDays"
            label="Days"
            multiple
            chips
            small-chips
            outlined
            dense
            clearable
            hide-details
            single-line
          />
        </v-col>
        <v-col
          cols="12" order="2"
          sm="7" order-sm="3"
        >
          <v-dialog
            ref="startTimeDialog"
            v-model="filterStartTimeMenu"
            :return-value.sync="advancedFilterIndex.filterStatTime"
            persistent
            width="290px"
            >
            <template v-slot:activator="{ on }">
              <v-text-field
                v-model="filterStartTime12H"
                label="Start at"
                readonly
                clearable
                outlined
                dense
                single-line
                hide-details
                v-on="on"
              />
            </template>
            <v-time-picker
              v-if="filterStartTimeMenu"
              v-model="advancedFilterIndex.filterStartTime"
              format="ampm"
              full-width
              no-title
              :allowed-minutes="allowedMinutes"
              :allowed-hours="allowedHours"
              @click:hour="handleHourClicked"
            >
              <v-btn text @click="filterStartTimeMenu = false">Cancel</v-btn>
              <v-spacer/>
              <v-btn
                text
                color="primary"
                @click="$refs.startTimeDialog.save(advancedFilterIndex.filterStartTime)"
              >
                Ok
              </v-btn>
            </v-time-picker>
          </v-dialog>
        </v-col>
        <v-col
          cols="6" order="3"
          sm="4" order-sm="2"
        >
          <v-switch
            class="mt-0 pl-sm-1"
            label="Hide filled"
            v-model="advancedFilterIndex.hideFilled"
            dense
            hide-details
          />
        </v-col>
        <v-col
          cols="6" order="4"
          sm="4" order-sm="4"
        >
          <v-switch
            class="mt-0 pl-sm-1"
            label="Hide conflicting"
            v-model="advancedFilterIndex.hideConflicting"
            dense
            hide-details
          />
        </v-col>
      </v-row>
      <v-row dense align="center" v-if="includeMetatopiaFilters">
        <v-col cols="12" sm="7">
          <v-select
            v-model.lazy="advancedFilterIndex.testTypes"
            :items="availableTestTypes"
            label="Test Types"
            multiple
            chips
            small-chips
            outlined
            dense
            clearable
            hide-details
            single-line
          />
        </v-col>
        <v-col cols="12" sm="4">
          <v-select
            v-model.lazy="advancedFilterIndex.hiTestFilter"
            :items="hiTestFilterOptions"
            label="HI-Tests"
            outlined
            dense
            clearable
            single-line
            hide-details
          />
        </v-col>
      </v-row>
    </v-container>
    <template v-slot:actions>
      <v-spacer/>
        <v-btn @click="clearAdvancedFilter">
          Clear
        </v-btn>
        <v-btn @click="applyAdvancedFilter" color="primary">
          Apply
        </v-btn>
    </template>
  </Dialog>
</template>

<script lang="ts">
import Event from '@/models/event';
import Dialog from '@/components/Dialog.vue';
import moment, { Moment } from 'moment';
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import { AdvancedFilter, emptyAdvancedFilter } from './EventsList.vue';

@Component({
  components: {
    Dialog,
  },
})
export default class AdvancedFilterDialog extends Vue {
  public readonly hiTestFilterOptions: any = [
    { text: 'Hide HI-Tests', value: false },
    { text: 'Only HI-Tests', value: true },
  ];

  @Prop() private availableDays!: string[];
  @Prop() private availableTestTypes!: string[];
  @Prop() private includeMetatopiaFilters!: boolean;
  @Prop() private parentFilter!: AdvancedFilter;

  private showAdvancedFilter: boolean = false;
  private filterStartTimeMenu: boolean = false;
  private advancedFilterIndex: AdvancedFilter = Object.assign({}, this.parentFilter);

  @Watch('parentFilter')
  private onFilterReset(): void {
    this.advancedFilterIndex = Object.assign({}, this.parentFilter);
  }

  private allowedMinutes(minutes: number): boolean {
    return this.eventMinutesStart.has(minutes);
  }

  private allowedHours(minutes: number): boolean {
    return this.eventHoursStart.has(minutes);
  }

  get filterStartTime12H(): string | null {
    return this.advancedFilterIndex.filterStartTime
      ? moment(this.advancedFilterIndex.filterStartTime, 'HH:mm').format('h:mmA')
      : null;
  }

  set filterStartTime12H(val: string | null) {
    this.advancedFilterIndex.filterStartTime = val
      ? moment(val, 'h:mmA').format('HH:mm')
      : null;
  }

  get eventMinutesStart(): Set<number> {
    return Object.values(this.$store.state.schedule)
      .reduce(
        (acc: any, e: any) => acc.add(e.startTime.minutes()),
        new Set<number>(),
      ) as Set<number>;
  }

  get eventHoursStart(): Set<number> {
    return Object.values(this.$store.state.schedule)
      .reduce(
        (acc: any, e: any) => acc.add(e.startTime.hours()),
        new Set<number>(),
      ) as Set<number>;
  }

  private handleHourClicked(hour: number): void {
    if (this.eventMinutesStart.size > 1) {
      return;
    }
    this.advancedFilterIndex.filterStartTime = `${hour}:00`;
  }

  private closeAdvancedFilter(): void {
    Object.assign(this.advancedFilterIndex, this.parentFilter);
    this.showAdvancedFilter = false;
  }

  private applyAdvancedFilter(): void {
    this.$emit('apply', this.advancedFilterIndex);
    this.showAdvancedFilter = false;
  }

  private clearAdvancedFilter(): void {
    this.$emit('apply', Object.assign({}, emptyAdvancedFilter));
    this.showAdvancedFilter = false;
  }

  private dialogClosed(): void {
    Object.assign(this.advancedFilterIndex, this.parentFilter);
  }
}
</script>

<style scoped lang="scss">
.force-tiny-input {
  padding-left: 5px;
  padding-top: 0px;
  margin-top: 0px;
  div.v-input__slot {
    margin-bottom: 0px;
  }
}
</style>
