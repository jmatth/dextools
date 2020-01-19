<template>
  <v-navigation-drawer
    :value="value"
    @input="$emit('input', $event)"
    app
    clipped
    bottom
    disable-resize-watcher
    :mobile-break-point="$vuetify.breakpoint.thresholds.sm"
  >
    <v-list dense>
      <v-list-item link to="/">
        <v-list-item-action>
          <v-icon>list_alt</v-icon>
        </v-list-item-action>
        <v-list-item-content>
          <v-list-item-title>Schedule</v-list-item-title>
        </v-list-item-content>
      </v-list-item>
      <v-list-item link to="/calendar">
        <v-list-item-action>
          <v-icon>calendar_today</v-icon>
        </v-list-item-action>
        <v-list-item-content>
          <v-list-item-title>Calendar</v-list-item-title>
        </v-list-item-content>
      </v-list-item>
    </v-list>
    <v-divider/>
    <v-list>
      <v-list-item>
        <v-switch
          class="mt-0 pt-0"
          label="Dark Mode"
          hide-details
          v-model="$vuetify.theme.dark"
        />
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class NavDrawer extends Vue {
  @Prop() private value!: boolean;

  private toggleDarkMode(): void {
    this.$vuetify.theme.dark = !this.$vuetify.theme.dark;
    localStorage.darkMode = this.$vuetify.theme.dark;
  }
}
</script>
