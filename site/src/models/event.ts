import moment, { Moment } from 'moment';

export default class Event {
  [key: string]: any;
  public code: string;
  public title: string;
  public description: string;
  public system: string;
  public presenters: string;
  public authors: string;
  public startTime: Moment;
  public endTime: Moment;
  public filled: boolean;
  public hiTest: boolean;
  public testType: string;

  constructor({
      code,
      title,
      description,
      system,
      presenters,
      authors,
      start_time,
      end_time,
      filled,
      hi_test,
      test_type,
      tags,
    }: {
      code: string,
      title: string,
      description: string,
      system: string,
      presenters: string,
      authors: string,
      start_time: string,
      end_time: string,
      filled: boolean,
      hi_test: boolean
      test_type: string,
      tags: string,
    },
  ) {
    this.code = code;
    this.title = title;
    this.description = description;
    this.system = system;
    this.presenters = presenters;
    this.authors = authors;
    this.startTime = moment(start_time);
    this.endTime = moment(end_time);
    this.filled = filled;
    this.hiTest = hi_test;
    this.testType = test_type;
  }

  public conflicts(that: Event): boolean {
    // Let's just say events don't conflict with themselves
    if (this.code === that.code) { return false; }
    return (this.startTime.isSameOrBefore(that.startTime) && this.endTime.isSameOrAfter(that.startTime)) ||
           (this.startTime.isSameOrBefore(that.endTime) && this.endTime.isSameOrAfter(that.endTime));
  }
}
