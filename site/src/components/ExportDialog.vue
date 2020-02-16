<template>
  <v-card>
    <v-card-title primary-title>
      Export
    </v-card-title>
    <v-divider/>
    <v-card-text>
      <v-text-field
          label="Enter your name"
          :value="userName"
          @input="updateUserName"
      />
      <p>
        Use the buttons below to export to ICS (for importing into calendar applications)
        or email (to register for events). For email, you can also copy the text below.
      </p>
      <span v-if="!disableEmail">To: <a :href="'mailto:' + conEmail">{{ conEmail }}</a></span>
      <v-divider style="margin-bottom:5px; margin-top: 2px;"/>
      <pre ref="emailText" class="email-text">{{ emailText }}</pre>
    </v-card-text>
    <v-divider/>
    <v-card-actions>
      <v-spacer/>
      <v-btn
        class="mr-2"
        @click="$store.state.agenda.exportIcs($store.state.conName)"
      >
        ICS
      </v-btn>
      <v-tooltip top :disabled="!disableEmail">
        <template v-slot:activator="{ on }">
          <a :href="mailtoLink" v-on="on">
            <v-btn
              class="mr-2"
              @click="exportDialog = false"
              :disabled="disableEmail"
            >
              Email
            </v-btn>
          </a>
        </template>
        <span>{{ this.$store.state.conName }} is not accepting event registrations at this time.</span>
      </v-tooltip>
      <v-tooltip top v-model="shouldShowCopyMessage">
        <template v-slot:activator="{ on }">
          <v-btn v-on="{ on }" @click="copyEmailText">
            Copy
          </v-btn>
        </template>
        <span>{{ this.copyMessage }}</span>
      </v-tooltip>
    </v-card-actions>
  </v-card>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import Event from '@/models/event';
import { debounce } from 'lodash';

@Component
export default class ExportDialog extends Vue {
  public exportDialog: boolean = false;
  public shouldShowCopyMessage: boolean = false;
  public copyMessage: string = '';
  public userName: string = '';

  constructor() {
    super();
    this.updateUserNameStore = debounce(this.updateUserNameStore, 3000);
  }

  public updateUserName(name: string) {
    this.userName = name;
    this.updateUserNameStore(name);
  }

  public updateUserNameStore(name: string): void {
    this.$store.commit('setUserName', name);
  }

  get emailText(): string {
    const userName: string = this.userName || '<YOUR NAME>';
    return this.$store.state.agenda.events.reduce(
      (text: string, event: Event) => text += `\n${event.code}`, `Name: ${userName}\n\nEvents:`);
  }

  get conEmail(): string {
    return this.$store.state.conEmail;
  }

  get mailtoLink(): string {
    if (this.disableEmail) {
      return '#';
    }
    const subject = encodeURI(`${this.$store.state.conName} Registration`);
    const body = encodeURI(this.emailText);
    return `mailto:${this.conEmail}?subject=${subject}&body=${body}`;
  }

  get disableEmail(): boolean {
    return !this.$store.state.conEmail;
  }

  public showCopyTooltip(message: string): void {
    this.copyMessage = message;
    this.shouldShowCopyMessage = true;
    setTimeout(function() {
      // @ts-ignore
      this.shouldShowCopyMessage = false;
    }.bind(this), 5000);
  }

  public copyEmailText(): void {
    const container = this.$refs.emailText;
    this.$copyText(this.emailText, container).then(this.onCopy, this.onFailedCopy);
  }

  public onCopy(): void {
    this.showCopyTooltip('Email text copied to clipboard.');
  }

  public onFailedCopy(): void {
    this.showCopyTooltip('Error, please copy the email text manually.');
  }
}
</script>
<style scoped lang="scss">
.email-text {
  max-height: 200px;
  overflow: auto;
}
</style>
