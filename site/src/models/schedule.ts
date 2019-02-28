import Event from './event';
import * as ics from 'ics';
import { saveAs } from 'file-saver';

export default class Schedule {
  public readonly events: Event[] = [];
  public readonly conName: string = 'Dreamation';
  public readonly conEmail: string = 'josh@jmatth.com';

  public userName: string = '';

  private _lastAdded?: Event = undefined;

  get lastAdded() {
    return this._lastAdded;
  }

  public addEvent(event: Event): void {
    // Already added
    console.log(`Adding ${event.code} to schedule`);
    if (this.events.find((e: Event) => e.code === event.code) !== undefined) {
      console.log(`${event.code} already in schedule, nopping`);
      return;
    }
    this.events.push(event);
    this._lastAdded = event;
    this.events.sort((first, second) => {
      if (first.startTime.isSame(second.startTime)) {
        if (first.endTime.isSame(second.endTime)) {
          // Take place at exactly the same time, sort by code
          return first.code <= second.code ? -1 : 1;
        }
        // Shorter events go first
        return first.endTime.isBefore(second.endTime) ? -1 : 1;
      }
      // Earlier events go first
      return first.startTime.isBefore(second.startTime) ? -1 : 1;
    });
    this.updateLocalStorage();
  }

  public removeEvent(code: string): void {
    const eventIndex = this.events.findIndex((e: Event) => e.code === code);
    // Couldn't find it
    if (eventIndex < 0) {
      console.log(`Event code ${code} not found in schedule`);
    }
    this.events.splice(eventIndex, 1);
    this._lastAdded = undefined;
    this.updateLocalStorage();
  }

  public toString(): string {
    return this.events.toString();
  }

  public exportIcs(): void {
    const icsEventObjs = this.events.map((e) => {
      const start = e.startTime.format('YYYY-M-D-H-m').split('-');
      const end = e.endTime.format('YYYY-M-D-H-m').split('-');
      return {
        title: `${e.code} - ${e.title}`,
        description: e.description,
        start,
        end,
      };
    });
    const { error, value: icsEvent } = ics.createEvents(icsEventObjs);
    if ( !!error ) {
      throw error;
    }
    const blob = new Blob([icsEvent]);
    saveAs(blob, 'dextools.ics');
  }

  private updateLocalStorage() {
    localStorage.scheduledEventCodes = JSON.stringify(this.events.map((e: Event) => e.code));
  }
}
