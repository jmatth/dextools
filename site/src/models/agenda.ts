import Event from './event';
import * as ics from 'ics';
import { saveAs } from 'file-saver';
import moment from 'moment';
import log from 'loglevel';

export default class Agenda {
  public readonly events: Event[] = [];

  private _lastAdded?: Event = undefined;

  get lastAdded() {
    return this._lastAdded;
  }

  public contains(code: string): boolean {
    return this.events.find((e: Event) => e.code === code) !== undefined;
  }

  public toggleEvent(event: Event) {
    if (this.contains(event.code)) {
      this.removeEvent(event.code);
    } else {
      this.addEvent(event);
    }
  }

  public addEvent(event: Event, skipLastAdded: boolean = false): void {
    // Already added
    log.info(`Adding ${event.code} to agenda`);
    const existingEvent = this.events.find((e: Event) => e.code === event.code);
    if (!!existingEvent) {
      log.warn(`${event.code} already in agenda, only updating lastAdded`);
      this._lastAdded = existingEvent;
      return;
    }
    this.events.push(event);
    if (!skipLastAdded) {
      this._lastAdded = event;
    }
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
    log.info(`Removing ${code} from agenda`);
    const eventIndex = this.events.findIndex((e: Event) => e.code === code);
    // Couldn't find it
    if (eventIndex < 0) {
      log.warn(`Event code ${code} not found in agenda`);
    }
    this.events.splice(eventIndex, 1);
    this._lastAdded = undefined;
    this.updateLocalStorage();
  }

  public toString(): string {
    return this.events.toString();
  }

  public exportIcs(conName: string): void {
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
    saveAs(blob, `${conName.toLowerCase().replace(/ /g, '_')}.ics`);
  }

  private updateLocalStorage() {
    localStorage.agendaEventCodes = JSON.stringify(this.events.map((e: Event) => e.code));
  }
}
