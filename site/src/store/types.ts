import Agenda from '@/models/agenda';
import Event from '@/models/event';
import { StitchAppClient } from 'mongodb-stitch-browser-sdk';

export interface Schedule {
  [index: string]: Event;
}

export interface RootState {
    loading: boolean;
    agenda: Agenda;
    schedule: Schedule;
    userName: string;
    conName: string;
    conEmail: string;
    isMetatopia: boolean;
    stitchClient: StitchAppClient | null;
    visitId: string;
    feedbackUrl: string;
    notice: string;
}
