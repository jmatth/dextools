import Event from './event';
import * as ics from 'ics';
import { saveAs } from 'file-saver';
import moment from 'moment';

export default class Agenda {
  public readonly events: Event[] = [];

  private _lastAdded?: Event = undefined;

  get lastAdded() {
    return this._lastAdded;
  }

  public addEvent(event: Event): void {
    // Already added
    console.log(`Adding ${event.code} to agenda`);
    if (this.events.find((e: Event) => e.code === event.code) !== undefined) {
      console.log(`${event.code} already in agenda, nopping`);
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
      console.log(`Event code ${code} not found in agenda`);
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
      const start = moment.utc(e.startTime).format('YYYY-M-D-H-m').split('-');
      const end = moment.utc(e.endTime).format('YYYY-M-D-H-m').split('-');
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
    localStorage.agendaEventCodes = JSON.stringify(this.events.map((e: Event) => e.code));
  }
}
