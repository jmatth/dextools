<template>
  <v-dialog
    v-model="showAdvancedFilter"
    max-width="700px"
    @input="e => !e && dialogClosed()"
  >
    <template v-slot:activator="{ on }">
      <v-btn icon small v-on="on" aria-label="Advanced filters">
        <v-icon>filter_list</v-icon>
      </v-btn>
    </template>
    <v-card>
      <v-card-title>
        Advanced Search
      </v-card-title>
      <v-divider/>
      <v-card-text>
        <v-container fluid>
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
              <v-menu
                ref="startTimeMenu"
                v-model="filterStartTimeMenu"
                :close-on-content-click="false"
                :nudge-right="40"
                :return-value.sync="advancedFilterIndex.filterStartTime"
                transition="scale-transition"
                offset-y
                max-width="290px"
                min-width="290px"
                >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    v-model="advancedFilterIndex.filterStartTime"
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
                  format="24hr"
                  :allowed-minutes="timePickerStep"
                  @click:hour="$refs.startTimeMenu.save($event + ':00')"
                />
              </v-menu>
            </v-col>
            <v-col
              cols="6" order="3"
              sm="4" order-sm="2"
            >
              <v-switch
                class="force-tiny-input"
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
                class="force-tiny-input"
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
      </v-card-text>
      <v-divider/>
      <v-card-actions>
        <v-btn @click="closeAdvancedFilter">
          Close
        </v-btn>
        <v-spacer/>
        <v-btn @click="clearAdvancedFilter">
          Clear
        </v-btn>
        <v-btn @click="applyAdvancedFilter" color="primary">
          Apply
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script lang="ts">
import Event from '../models/event';
import { Moment } from 'moment';
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import { AdvancedFilter, emptyAdvancedFilter } from './EventsList.vue';

@Component
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

  private timePickerStep(minutes: number): boolean {
    return minutes % 30 === 0;
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
