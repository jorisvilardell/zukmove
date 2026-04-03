import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Student } from '@/lib/api';

interface AuthState {
    student: Student | null;
    login: (student: Student) => void;
    logout: () => void;
}

export const useAuthStore = create<AuthState>()(
    persist(
        (set) => ({
            student: null,
            login: (student) => set({ student }),
            logout: () => set({ student: null }),
        }),
        { name: 'zukmove-auth' }
    )
);
