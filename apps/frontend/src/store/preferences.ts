import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface PreferencesState {
    channel: 'email' | 'discord';
    contact: string;
    enabled: boolean;
    setChannel: (ch: 'email' | 'discord') => void;
    setContact: (c: string) => void;
    setEnabled: (e: boolean) => void;
    reset: () => void;
}

export const usePreferencesStore = create<PreferencesState>()(
    persist(
        (set) => ({
            channel: 'email',
            contact: '',
            enabled: true,
            setChannel: (channel) => set({ channel }),
            setContact: (contact) => set({ contact }),
            setEnabled: (enabled) => set({ enabled }),
            reset: () => set({ channel: 'email', contact: '', enabled: true }),
        }),
        { name: 'zukmove-preferences' }
    )
);
