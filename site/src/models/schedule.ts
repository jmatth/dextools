import Event from './event';

export default class Schedule {
  readonly events: Event[] = [];

  public addEvent(event: Event): void {
    // Already added
    console.log(`Adding ${event.code} to schedule`);
    if (this.events.find((e: Event) => e.code === event.code) !== undefined) {
      console.log(`${event.code} already in schedule, nopping`);
      return;
    }
    this.events.push(event);
  }

  public removeEvent(code: string): void {
    const eventIndex = this.events.findIndex((e: Event) => e.code === code);
    // Couldn't find it
    if (eventIndex < 0) {
      console.log(`Event code ${code} not found in schedule`);
    }
    this.events.splice(eventIndex, 1);
  }

  public toString(): string {
    return this.events.toString();
  }
}
