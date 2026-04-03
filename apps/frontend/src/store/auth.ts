import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Student } from '@/lib/api';

interface AuthState {
    student: Student | null;
    seenNewsIds: string[];
    login: (student: Student) => void;
    logout: () => void;
    markNewsSeen: (ids: string[]) => void;
}

export const useAuthStore = create<AuthState>()(
    persist(
        (set, get) => ({
            student: null,
            seenNewsIds: [],
            login: (student) => set({ student }),
            logout: () => set({ student: null, seenNewsIds: [] }),
            markNewsSeen: (ids) => {
                const current = get().seenNewsIds;
                const merged = [...new Set([...current, ...ids])];
                set({ seenNewsIds: merged });
            },
        }),
        { name: 'zukmove-auth' }
    )
);
